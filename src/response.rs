use crate::agents::processing_agent::process;
use axum::{Json,debug_handler};
use tokio::task;

 
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


    let response = task::spawn_blocking(move||{
        let reply= process(&query.message);

        println!("{}", &reply.unwrap())
    }).await.ok();
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


    println!("{:#?}",&response);
   

    return Json::from(
        Respond{
            output: "HELLO".to_string(),
        }
    )
}









// Utilities

pub fn response(_message:&str) -> String{

    let reply = process(&_message).unwrap();

    reply
}




