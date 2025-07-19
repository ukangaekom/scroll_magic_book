mod response;
use axum::{
    extract::{Path, Query}, 
    routing::get,
    routing::post,
    Router
};
use tokio::net::TcpListener;




#[tokio::main]
async fn main(){

    // Axum router 
    let router_scroll_magic_book: Router = Router::new().route("/scroll_magic_book",
    post(response::request));


    // Define Ip and Port
    let address: &'static str = "0.0.0.0:6570";
    let listener: TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();


    println!("Listener on {address}\n");



    // Launch the web server
    axum::serve(listener, router_scroll_magic_book).await.unwrap();
}


