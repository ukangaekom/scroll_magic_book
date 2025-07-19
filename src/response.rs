use axum::{Json,extract::{Path, Query},debug_handler};
use std::collections::HashMap;
use reqwest;


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

   

    return Json::from(
        Respond{
            output: query.message.to_string(),
        }
    )
}




