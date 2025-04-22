use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    Database::connect(&url).await
}
