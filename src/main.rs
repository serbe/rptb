use std::time::Duration;

use dotenv::dotenv;
use tokio::runtime::Runtime;
use tokio::time::delay_for;
// use rutel::bot::;
// use serde_json::{from_value, Value, from_slice};
use rutel::bot::{Bot, GetMe, GetUpdates, SendMessage};
use rutel::types::{ChatID};
// use bytes::BufMut;

use crate::error::Result;

mod error;

async fn run() -> Result<()> {
    let token = dotenv::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN is not set");
    let mut b = Bot::new(&token);
    let gm = GetMe::new();
    let msg = b.get_me(&gm).await?;
    log::info!("msg : {:?}", msg);
    let sm = SendMessage::new(ChatID::from(-1001102759484i64), "test".to_string());
    log::info!("sm : {:?}", sm);
    let msg = b.send_message(&sm).await?;
    log::info!("msg : {:?}", msg);
    let gu = GetUpdates::new();
    let msg = b.get_updates(&gu).await?;
    log::info!("msg : {:?}", msg);
    delay_for(Duration::from_millis(100)).await;
    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let mut rt = Runtime::new().unwrap();

    rt.block_on(async { run().await.unwrap() });
}
