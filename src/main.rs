mod db;
mod entity;

use anyhow::Result;

use axum::{routing::get, Router};

use db::init_db;
use entity::user;
use sea_orm::{EntityTrait, Set};

#[tokio::main]
async fn main() -> Result<()> {
    let db = init_db().await?;

    let now = chrono::Utc::now();

    let new_user = user::ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        name: Set("owo".into()),
        email: Set("uwu@example.com".into()),
        verified: Set(false),
        password: Set("hashed_pw".into()),
        verification_token: Set(Some("token123".into())),
        token_expires_at: Set(Some((now + chrono::Duration::hours(24)).into())),
        role: Set(user::UserRole::User),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    };
    let insert_res = user::Entity::insert(new_user).exec(&db).await?;
    println!("Inserted user id: {}", insert_res.last_insert_id);

    let app = Router::new().route("/", get(|| async { "Hello, Axum + SeaORM!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
