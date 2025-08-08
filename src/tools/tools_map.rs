use crate::tools::wrappers::getters::*;
use std::collections::HashMap;
use std::pin::Pin;
use std::future::Future;
use once_cell::sync::Lazy;
use std::sync::Arc;


type AsyncFn = Arc<
    dyn for<'a> Fn(&'a [&'a str]) -> Pin<Box<dyn Future<Output = String> + Send + 'a>>
        + Send
        + Sync,
>;


pub static TOOLS: Lazy<HashMap<&'static str, AsyncFn>> = Lazy::new(||{
    let mut m: HashMap<&'static str, AsyncFn> = HashMap::new();
    m.insert("get_price", Arc::new(|args| Box::pin(get_price_wrapper(args))));
    m.insert("get_marketcap", Arc::new(|args| Box::pin(get_marketcap_wrapper(args))));
    m
});




// pub static TOOLS: phf::Map<&'static str, fn(&[&str]) -> String> = phf_map!{

//     "get_price"=>get_price_wrapper,
//     "get_marketcap"=>get_marketcap_wrapper,

// };