use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    //expose environment variables from .env file
    dotenvy::dotenv().expect("Unable to access .env file");

    //set variables from environment variables
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

    //create our database pool
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Can't connect to the database");

    //create our tcp listener
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create tcp listener");

    println!("listening on {}", listener.local_addr().unwrap());

    // compose the routes
    let app = Router::new()
        .route("/", get(|| async { "Up and Running" }))
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/{user_id}", get(get_user).patch(update_user).delete(delete_user),
        )
        .with_state(db_pool);

    //serve the application
    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

async fn get_users(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(UserRow, "SELECT * FROM users ORDER BY user_id")
        .fetch_all(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": rows}).to_string(),
    ))
}

async fn get_user(
    State(db_pool): State<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(UserRow, "SELECT * FROM users WHERE user_id = $1", user_id)
        .fetch_one(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": rows}).to_string(),
    ))
}

async fn create_user(
    State(db_pool): State<PgPool>,
    Json(user): Json<CreateUserReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        CreateUserRow,
        "INSERT INTO users (name, age) VALUES ($1, $2) RETURNING user_id",
        user.name,
        user.age
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        json!({"success": true, "data": row}).to_string(),
    ))
}

async fn update_user(
    State(db_pool): State<PgPool>,
    Path(user_id): Path<i32>,
    Json(user): Json<UpdateUserReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut query = "UPDATE public.users SET user_id = $1".to_owned();

    let mut i = 2;

    if user.name.is_some() {
        query.push_str(&format!(", name = ${i}"));
        i = i + 1;
    };

    if user.age.is_some() {
        query.push_str(&format!(", age = ${i}"));
    };

    query.push_str(&format!(" WHERE user_id = $1"));

    let mut s = sqlx::query(&query).bind(user_id);

    if user.name.is_some() {
        s = s.bind(user.name);
    }

    if user.age.is_some() {
        s = s.bind(user.age);
    }

    s.execute(&db_pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    Ok((StatusCode::OK, json!({"success":true}).to_string()))
}

async fn delete_user(
    State(db_pool): State<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    sqlx::query!("DELETE FROM public.users WHERE user_id = $1", user_id)
        .execute(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((StatusCode::OK, json!({"success":true}).to_string()))
}

#[derive(Serialize)]
struct UserRow {
    user_id: i32,
    name: String,
    age: Option<i32>,
}

#[derive(Deserialize)]
struct CreateUserReq {
    name: String,
    age: Option<i32>,
}

#[derive(Serialize)]
struct CreateUserRow {
    user_id: i32,
}

#[derive(Deserialize)]
struct UpdateUserReq {
    name: Option<String>,
    age: Option<i32>,
}
