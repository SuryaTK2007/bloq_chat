use axum::{routing::get, Router};
use std::net::SocketAddr;
#[tokio::main]
async fn main(){
    let app=Router::new().route("/", get(root));
    let addr:SocketAddr="127.0.0.1:3000".parse().unwrap();
    println!("Server running in http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(),app).await.unwrap();
}
async fn root()->&'static str{
    "Hello"
}