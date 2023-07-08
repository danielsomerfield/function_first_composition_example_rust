extern crate functionfirst;

mod testutils;

#[cfg(test)]
mod integration_tests {
    use reqwest::get;
    use reqwest::StatusCode;
    use serde_json::Value;
    use sqlx::postgres::PgPoolOptions;
    use testcontainers::{clients, RunnableImage};
    use testcontainers::images::postgres::Postgres;

    use testutils::test_utils::{create_rating_by_user_for_restaurant, create_restaurant, create_user, Restaurant, User};

    use crate::testutils;

    #[tokio::test]
    #[ignore]
    async fn the_restaurant_endpoint_ranks_by_recommendations() {
        let docker = clients::Cli::default();
        let postgres = docker.run(
            RunnableImage::from(Postgres::default())
                .with_volume(("./db", "/docker-entrypoint-initdb.d"))
        );

        postgres.start();

        let pool = PgPoolOptions::new().connect(
            format!("postgresql://postgres:postgres@localhost:{}/postgres",
                    postgres.get_host_port_ipv4(5432)).as_str()
        ).await.expect("Could not get DB pool");

        let users = [
            User { id: "user1".to_string(), name: "User 1".to_string(), trusted: true },
            User { id: "user2".to_string(), name: "User 2".to_string(), trusted: false },
            User { id: "user3".to_string(), name: "User 3".to_string(), trusted: false },
        ];

        for user in &users {
            create_user(&user, &pool).await;
        }

        let restaurants = [
            Restaurant { id: "cafegloucesterid".to_string(), name: "Cafe Gloucester".to_string() },
            Restaurant { id: "burgerkingid".to_string(), name: "Burger King".to_string() },
        ];

        for restaurant in &restaurants {
            create_restaurant(&restaurant, &pool).await
        }

        let ratings = [
            ("rating1".to_string(), &users[0], &restaurants[0], "EXCELLENT".to_string()),
            ("rating2".to_string(), &users[1], &restaurants[0], "TERRIBLE".to_string()),
            ("rating3".to_string(), &users[2], &restaurants[0], "AVERAGE".to_string()),
            ("rating4".to_string(), &users[2], &restaurants[1], "ABOVE_AVERAGE".to_string()),
        ];

        for rating in ratings {
            create_rating_by_user_for_restaurant(rating, &pool).await
        }

        let server = functionfirst::start().await;

        // Hit the restaurant endpoint
        let response = get("http://localhost:3000/vancouverbc/restaurants/recommended").await
            .expect("HTTP GET failed.");

        assert_eq!(StatusCode::OK, response.status());

        let body = response
            .json::<Value>().await
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

