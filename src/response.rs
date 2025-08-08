use crate::agents::{processing_agent::process, report_agent::report_result};
use crate::services::getter::market::{get_price, get_marketcap};
use crate::tools::tools_map::TOOLS;
use crate::tools::utils::extract_tool_params;
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

    let reply = task::block_in_place(||async move{
        let process = process(&query.message).await.unwrap();

        

        if let Some((tool, parameters)) = extract_tool_params(&process.to_string()){

            // println!("{:#?}",extract_tool_params(&process));
            // Convert &Vec<String> -> Vec<&str>
            let params: Vec<&str> = parameters.iter().map(|s| s.as_str()).collect();

            let result = if let Some(func) = TOOLS.get(&tool.as_str()){
                return func(&params).await;
            }else{
                "Tools not found".to_string();
            };

            process


            // TOOLS.get(&tool).map(|func| func(&params).await);
        }else{
            process
        }
        
    }).await;

    // let reply_2 = report_result(&price).await.unwrap();

    // println!("The price of bitcoin is {}", price);
    // println!("{}",reply_2);

    match reply{
        result => {
            let ouput = report_result(&result).await.unwrap();
            return Json::from(
        Respond{output:ouput})


        }
        _ => {
            return Json::from(
        Respond{output:"Error".to_string(),})
    
        }
    } 
}




