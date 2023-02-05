use sqlx::PgPool;

use crate::{model::user::RegisterRequest, repository::user::create_user};

#[tokio::test]
async fn test_create_user() {
    use std::env;

    dotenv::dotenv().ok();
    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let user = RegisterRequest {
        name: "test".to_string(),
        password: "lblblb".to_string(),
        email: "orange_cheers2@outlook.com".to_string(),
    };
    let user = create_user(&pool, user).await.unwrap();
    println!("output: {:?}", user);
    assert!(false);
}
