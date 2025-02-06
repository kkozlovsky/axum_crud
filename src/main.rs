use axum::{
    routing::{get},
    Router,
};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    //expose environment variables from .env file
    dotenvy::dotenv().expect("Unable to access .env file");

    //set variables from environment variables
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());

    //create our tcp listener
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create tcp listener");

    println!("listening on {}", listener.local_addr().unwrap());

    // compose the routes
    let app = Router::new().route("/", get(|| async { "Up and Running" }));

    //serve the application
    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
