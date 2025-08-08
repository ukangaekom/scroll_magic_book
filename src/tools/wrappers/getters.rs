use crate::services::getter::market::*;



pub async fn get_price_wrapper(args:&[&str])-> String{
    let mut results = Vec::new();

    for token in args{
        let result = get_price(token).await;
        results.push(result);

    }

    results.join(", ")
    
}



pub async fn get_marketcap_wrapper(args: &[&str])-> String{
    let mut results = Vec::new();

    for token in args{  
        let result = get_marketcap(token).await;
        results.push(result);
    }

    results.join(", ")
    
}