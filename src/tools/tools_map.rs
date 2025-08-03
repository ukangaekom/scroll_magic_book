use phf::phf_map;
use crate::services::getter;


pub static TOOLS: phf::Map<&'static str, fn(&[&str])> = phf::phf_map!{

    "get_price"=>get_price,
    "get_marketcap"=>get_marketcap


}