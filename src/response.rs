
use scroll_magic_book::agents::processing_agent::process;

use axum::{Json,extract::{Path, Query},debug_handler};
use std::collections::HashMap;
use reqwest;
use  tokio::task;
use serde::de::Unexpected::Option;


#[derive(Debug, serde::Serialize)]
pub struct Respond{
    output:String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Prompt{
    message:String,
    name:String,
    media:String
}



#[debug_handler]
pub async fn request(Json(query):Json<Prompt>) -> Json<Respond>{

    println!("Successfully Responsed {:?}",query.message);
    println!("Successfully Responsed {:?}",query.media);

    let response = task::spawn(async move{process(&query.message).await.unwrap_or_default()});
    

    match response.await {
        Ok(result) => println!("Got result: {:?}", result),
        Err(e) => eprintln!("Task failed {:?}",e),
    }

    // let result = response.await.unwrap();


    // println!("{:?}",&response);
   

    return Json::from(
        Respond{
            output: "Done".to_string(),
        }
    )
}














// Utilities

#[tokio::main]
pub async fn response(_message:&str) -> String{

    let reply = process(&_message).await.unwrap();

    reply
}




