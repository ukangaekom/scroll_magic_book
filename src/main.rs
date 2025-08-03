// Defining Modules
mod response;
mod agents;
mod services;

use response::request;
use axum::{
    extract::{Path, Query}, 
    routing::get,
    routing::post,
    Router
};
use tokio::net::TcpListener;
use std::env;





#[tokio::main]
async fn main(){
    dotenv_flow::dotenv_flow().ok();


    // Axum router 
    let router_scroll_magic_book: Router = Router::new().route("/scroll_magic_book",
    post(request));


    // Define Ip and Port
    let address: &'static str = "0.0.0.0:6570";
    let listener: TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();


    println!("Listener on {address}\n");


    // let output = agents::processing_agent::process("Hello How are you").await.unwrap();

    // println!("{}",output);
    // Launch the web server
    axum::serve(listener, router_scroll_magic_book).await.unwrap();
}




#[cfg(test)]
mod scroll_magic_book_test{
    #[test]
    fn test_request(){
        super::request()
    }

}


