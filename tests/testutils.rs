pub mod test_utils {
    use sqlx::{Pool, Postgres};

    pub struct User {
        pub id: String,
        pub name: String,
        pub trusted: bool,
    }

    pub struct Restaurant {
        pub id: String,
        pub name: String,
    }

    pub async fn create_user(user: &User, pool: &Pool<Postgres>) -> () {
        sqlx::query("INSERT into \"user\" (id, name, trusted) VALUES ($1, $2, $3)")
            .bind(&user.id)
            .bind(&user.name)
            .bind(user.trusted)
            .execute(pool)
            .await
            .expect("Failed to create user");
    }

    pub async fn create_restaurant(restaurant: &Restaurant, pool: &Pool<Postgres>) {
        sqlx::query("INSERT into \"restaurant\" (id, name) VALUES ($1, $2)")
            .bind(&restaurant.id)
            .bind(&restaurant.name)
            .execute(pool)
            .await
            .expect("Failed to create user");
    }

    pub async fn create_rating_by_user_for_restaurant(
        rating: (String, &User, &Restaurant, String),
        pool: &Pool<Postgres>,
    ) {
        sqlx::query("insert into restaurant_rating (id, rated_by_user_id, restaurant_id, rating, city) VALUES ($1, $2, $3, $4, 'vancouverbc')")
            .bind(&rating.0)
            .bind(&rating.1.id)
            .bind(&rating.2.id)
            .bind(&rating.3)
            .execute(pool)
            .await
            .expect("SQL execute failed while creating restaurant");
    }
}
