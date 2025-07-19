use ethers::providers::{Provider, Http};
use std::sync::OnceLock;

// STATICS
static INFURA_API_KEY: OnceLock<String> = OnceLock::new();


// Getting API Keys
fn get_infura_api_key() -> &'static str {
    INFURA_API_KEY.get_or_init(|| {
        std::env::var("INFURA_API_KEY").expect("INFURA_API_KEY must be set")
    })
}


static SCROLL_MAINNET: OnceLock<Provider<Http>> = OnceLock::new();
static SCROLL_SEPOLIA: OnceLock<Provider<Http>> = OnceLock::new();


pub fn get_scroll_mainnet_provider()-> &'static Provider<Http>{
    SCROLL_MAINNET.get_or_init(||{
        Provider::try_from(
            format!("https://scroll-mainnet.infura.io/v3/{:?}",get_infura_api_key())
        ).expect("MUST USE API_KEY")
    })
}

pub fn get_scroll_sepolia_provider()-> &'static Provider<Http>{
    SCROLL_SEPOLIA.get_or_init(||{
        Provider::try_from(
            format!("https://scroll-sepolia.infura.io/v3/{:?}",get_infura_api_key())
        ).expect("MUST USE API_KEY")
    })
}


