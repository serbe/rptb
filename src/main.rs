use std::time::Duration;

use dotenv::dotenv;
use tokio::runtime::Runtime;
use tokio::time::delay_for;
// use rutel::bot;
use rpc::client::Client;
// use serde_json::{from_value, Value, from_slice};
use rutel::bot::Bot;
// use rutel::types::ChatID;

use crate::error::Result;

mod error;

fn build_uri(token: &str, method: &str) -> String {
    format!("https://api.telegram.org/bot{}/{}", token, method)
}

pub async fn create_request(bot: &Bot, method: &'static str, values: String) -> Result<()> {
    let uri = bot.build_uri(method);
    let client_builder = Client::builder();

    let mut client = client_builder
        .post(&uri)
        .body(values.as_bytes())
        .header("Content-Type", "application/json")
        .build()
        .await?;
    let _response = client.send().await?;
    let body = client.text().await?;
    log::info!("body: {}", body);
    Ok(())
}

async fn run() -> Result<()> {
    let token = dotenv::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN is not set");
    let uri = build_uri(&token, "sendMessage");
    let mut cb = Client::builder()
        .post(&uri)
        .header("Content-Type", "application/json");

    // let mut b = bot::Bot::new(&token);
    // let sm = bot::SendMessage::new(ChatID::from(-1001102759484i64), "test".to_string());
    // let gu = bot::GetUpdates::new();
    // create_request(&b, "getUpdates", gu.to_string()?).await?;

    // let msg = b.get_updates(&gu).await?;
    // let msg = b.create_request("sendMessage", r#"{"chat_id":-1001102759484,"text":"Hello World"}"#.to_string()).await?;
    // log::info!("msg: {:?}", msg);
    delay_for(Duration::from_millis(100)).await;
    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let mut rt = Runtime::new().unwrap();

    rt.block_on(async { run().await.unwrap() });
}
