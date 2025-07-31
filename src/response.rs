use crate::agents::processing_agent::process;
use crate::services::getter::market::{get_price, get_marketcap};
use axum::{Json,debug_handler};
use tokio::task;

// use tokio::sync::Mutex;


 
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

    let info = &query.message.clone();

    let reply = task::spawn(async move{
        let process = process(&query.message);
        // response = format!("{:?}", process.as_ref().unwrap());
        // println!("THE NEXT PHASE IN THE FUNCTION \n\n\n\n\n{:?}", process.as_ref().unwrap());
        process.await.unwrap()
        // process.unwrap()
    }).await;

    let reply_2 = process(&info).await.unwrap();


    let price = get_price("BTC").await;

    println!("The price of bitcoin is {}", price);
    println!("{}",reply_2);

    match reply{
        Ok(result) => {
            return Json::from(
        Respond{output:result})


        }
        _ => {
            return Json::from(
        Respond{output:"Error".to_string(),})
    
        }
    } 
}




