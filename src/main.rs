use anyhow::Result;
use soup::prelude::*;
use teloxide_core::{
    payloads::SendMessage,
    requests::{JsonRequest, Request},
    Bot,
};
use tokio::time::{sleep, Duration};

const URL: &str = "https://www.dongao.com/zckjs/zkz/202107063478415.shtml";
const TELEGRAM_TOKEN_KEY: &str = "TELEGRAM_TOKEN";
const TELEGRAM_TO_KEY: &str = "TELEGRAM_TO";

#[tokio::main]
async fn main() {
    loop {
        let status = get_status_of_js(&get_document().await.expect("unable to get document"));
        if status != "暂未开通" {
            break;
        }
        sleep(Duration::from_secs(1)).await;
    }
    send_message(&format!("注会信息有更新，请前往查看 {}", URL)).await;
}

async fn get_document() -> Result<String> {
    Ok(reqwest::get(URL).await.expect("cannot").text().await?)
}

fn get_status_of_js(doc: &str) -> String {
    let soup = Soup::new(&doc);
    let js_text = "江　苏";
    let js_strong_node = soup
        .tag("strong")
        .find_all()
        .find(|strong| strong.text().trim() == js_text)
        .expect("cannot find a strong node contains 江苏");
    let js_tr_node = js_strong_node
        .parents()
        .nth(1)
        .expect("2nd parent of 江苏 strong node does not exist");
    let status = js_tr_node
        .children()
        .nth(2)
        .expect("the tr node contains 江苏 does not have a 3rd children node")
        .children()
        .nth(1)
        .expect("the td node contains 江苏 does not have a 2nd children node") // span
        .text();
    status.trim().to_string()
}

async fn send_message(message: &str) {
    let bot = Bot::new(std::env::var(TELEGRAM_TOKEN_KEY).expect(&format!(
        "failed to get environment variable {}",
        TELEGRAM_TOKEN_KEY
    )));
    let payload = SendMessage::new(
        std::env::var(TELEGRAM_TO_KEY).expect(&format!(
            "failed to get environment variable {}",
            TELEGRAM_TO_KEY,
        )),
        message,
    );
    JsonRequest::new(bot, payload)
        .send()
        .await
        .expect("failed to send message");
}
