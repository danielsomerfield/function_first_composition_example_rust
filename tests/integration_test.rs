extern crate functionfirst;

#[cfg(test)]
mod integration_tests {
    use postgres::{Client, Error, NoTls};
    use reqwest::blocking::get;
    use reqwest::StatusCode;
    use serde_json::{to_vec, Value};
    use testcontainers::{clients, RunnableImage};
    use testcontainers::images::postgres::Postgres;

    use functionfirst::Server;

    use crate::test_utils::{create_rating_by_user_for_restaurant, create_restaurant, create_user, Restaurant, User};

    #[test]
    fn the_restaurant_endpoint_ranks_by_recommendations() {
        let docker = clients::Cli::default();
        let postgres = docker.run(
            RunnableImage::from(Postgres::default())
                .with_volume(("./db", "/docker-entrypoint-initdb.d"))
        );

        postgres.start();

        let mut client = Client::connect(
            format!("postgresql://postgres:postgres@localhost:{}/postgres",
                    postgres.get_host_port_ipv4(5432)).as_str(),
            NoTls,
        ).expect("Failed to connect");

        let users = [
            User { id: "user1".to_string(), name: "User 1".to_string(), trusted: true },
            User { id: "user2".to_string(), name: "User 2".to_string(), trusted: false },
            User { id: "user3".to_string(), name: "User 3".to_string(), trusted: false },
        ];

        for user in &users {
            create_user(&user, &mut client);
        }

        let restaurants = [
            Restaurant { id: "cafegloucesterid".to_string(), name: "Cafe Gloucester".to_string() },
            Restaurant { id: "burgerkingid".to_string(), name: "Burger King".to_string() },
        ];

        for restaurant in &restaurants {
            create_restaurant(&restaurant, &mut client)
        }

        let ratings = [
            ("rating1".to_string(), &users[0], &restaurants[0], "EXCELLENT".to_string()),
            ("rating2".to_string(), &users[1], &restaurants[0], "TERRIBLE".to_string()),
            ("rating3".to_string(), &users[2], &restaurants[0], "AVERAGE".to_string()),
            ("rating4".to_string(), &users[2], &restaurants[1], "ABOVE_AVERAGE".to_string()),
        ];

        for rating in ratings {
            create_rating_by_user_for_restaurant(rating, &mut client)
        }

        let server = functionfirst::start();

        // Hit the restaurant endpoint
        let response = get("http://localhost:3000/vancouverbc/restaurants/recommended")
            .expect("HTTP GET failed.");

        assert_eq!(StatusCode::OK, response.status());

        let body = response
            .json::<serde_json::Value>()
            .expect("Failed to deserialize");

        let restaurants = body.get("restaurants").expect("missing restaurants field");
        assert_eq!(true, restaurants.is_array());
        let restaurants_array = restaurants.as_array().unwrap();
        assert_eq!(2, restaurants_array.len());
        let ids: Vec<&str> = restaurants_array.into_iter().map(|r| r["id"].as_str().unwrap()).collect();
        assert_eq!(["cafegloucesterid", "burgerking"], ids.as_slice());

        // Verify return code and payload

        server.stop();
        postgres.stop();
    }
}

mod test_utils {
    use postgres::{Client, Error, NoTls};

    pub struct User {
        pub id: String,
        pub name: String,
        pub trusted: bool,
    }

    pub struct Restaurant {
        pub id: String,
        pub name: String,
    }

    pub fn create_user(user: &User, client: &mut Client) -> () {
        client.execute(
            "INSERT into \"user\" (id, name, trusted) VALUES ($1, $2, $3)",
            &[&user.id, &user.name, &user.trusted],
        ).expect("SQL execute failed while creating user");
    }

    pub fn create_restaurant(restaurant: &Restaurant, client: &mut Client) {
        client.execute(
            "INSERT into \"restaurant\" (id, name) VALUES ($1, $2)",
            &[&restaurant.id, &restaurant.name],
        ).expect("SQL execute failed while creating restaurant");
    }

    pub fn create_rating_by_user_for_restaurant(rating: (String, &User, &Restaurant, String), client: &mut Client) {
        client.execute("insert into restaurant_rating (id, rated_by_user_id, restaurant_id, rating, city) VALUES ($1, $2, $3, $4, 'vancouverbc')",
                       &[
                           &rating.0,
                           &rating.1.id,
                           &rating.2.id,
                           &rating.3
                       ]).expect("SQL execute failed while creating restaurant");
    }
}

