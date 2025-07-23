use crate::agents::processing_agent::process;
use axum::{Json,debug_handler};
use tokio::task;
// use tokio::sync::Mutex;

use std::sync::{Arc, Mutex};
use std::thread::{spawn};

 
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


    let reply = task::spawn_blocking(move||{
        let process = process(&query.message);
        // response = format!("{:?}", process.as_ref().unwrap());
        // println!("THE NEXT PHASE IN THE FUNCTION \n\n\n\n\n{:?}", process.as_ref().unwrap());
        process.unwrap()
        // process.unwrap()
    }).await;

    match reply{
        ref result => {
            return Json::from(
        Respond{output:format!("{:#?}",reply) })


        }
        _ => {
            return Json::from(
        Respond{output:"Error".to_string(),})
    
        }
    }


    

    // println!("Hey {:?}",reply);
    // let response = task::spawn(move||{
    //     let reply = process(&query.message);

    //     println!("Ouput:{}",&reply.unwrap());
    //     })();
    

    // match response.await {
    //     Ok(result) => println!("Got result: {:?}", result),
    //     Err(e) => eprintln!("Task failed {:?}",e),
    // }

    // let result = response.await.unwrap();
    // let reply = response(&query.message);


    // println!("{:#?}",&response);
   

    
}









// Utilities

pub fn response(_message:&str) -> String{

    // let reply = process(&_message).await.unwrap();

    // reply
    "Okay".to_string()
}




