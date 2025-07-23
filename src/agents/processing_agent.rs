use genai::chat::printer::print_chat_stream;
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use std::sync::OnceLock;


static PROCESS_SYSTEM_CONFIGURATION: OnceLock<String> = OnceLock::new();
static CLIENT: OnceLock<genai::Client> = OnceLock::new();


pub fn get_process_system_configuration() -> &'static String {
    PROCESS_SYSTEM_CONFIGURATION.get_or_init(||{
        include_str!("../../knowledge/scroll_magic_book.txt").to_string()
    })

}

pub fn get_client() -> &'static genai::Client {
    CLIENT.get_or_init(||{
        Client::default()
    })
}

#[tokio::main]
pub async fn process(_text:&str) -> Option<std::string::String> {

    let client = get_client();

    let chat_req: ChatRequest = ChatRequest::new(vec![
        ChatMessage::system(get_process_system_configuration()),
        ChatMessage::user(_text.to_string())
    ]);

    let model: &str = "gemini-2.0-flash";
    // let model: &str = "gemini-1.5-flash-latest";

    // let chat_res = client.exec_chat_stream(model, chat_req, None).await;
    let chat_res = client.exec_chat(model, chat_req, None).await;
    
    // let routing_response = match print_chat_stream(chat_res.expect("REASON"),  None).await {
    // let routing_response = match print_chat_stream(chat_res.expect("REASON"), None).await{

    //     Ok(response) => {
    //         return Some(response);
    //     },

    //     Err(e) => {
    //         return Some(e.to_string());
    //     }
    // };

    chat_res.expect("REASON").content_text_into_string()
    
}





