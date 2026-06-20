use sea_orm::{Database, DatabaseConnection};

pub async fn connect() -> DatabaseConnection {

    // load env
    dotenv::dotenv().ok();

    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be filed");

    Database::connect(&url).await.unwrap()
}