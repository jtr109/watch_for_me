use anyhow::Result;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use soup::prelude::*;
use std::io::Write;
use teloxide_core::{
    payloads::SendMessage,
    requests::{JsonRequest, Request},
    Bot,
};
use tokio::time::{sleep, Duration};

const URL: &str = "https://www.dongao.com/zckjs/zkz/202107063478415.shtml";
const TELEGRAM_TOKEN_KEY: &str = "TELEGRAM_TOKEN";
const TELEGRAM_TO_KEY: &str = "TELEGRAM_TO";
const USER_AGENT: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:90.0) Gecko/20100101 Firefox/90.0";

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();
    let duration = std::env::var("DURATION")
        .unwrap_or("60".to_string())
        .parse::<u64>()
        .expect("failed to parse environment variable DURATION");
    send_message(&format!("开始监控注会信息更新，时间间隔{}秒", duration)).await;
    loop {
        let doc = match get_document().await {
            Ok(d) => d,
            Err(e) => {
                log::warn!("{}", e);
                sleep(Duration::from_secs(duration)).await;
                continue;
            }
        };
        let status = get_status_of_js(&doc);
        if status != "暂未开通" {
            break;
        }
        log::info!(
            "the status is still \"not opened\", will try again in {} seconds.",
            duration
        );
        sleep(Duration::from_secs(duration)).await;
    }
    send_message(&format!("注会信息有更新，请前往查看 {}", URL)).await;
    Ok(())
}

fn init_logger() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

async fn get_document() -> Result<String> {
    let client = reqwest::Client::new();
    Ok(client
        .get(URL)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?
        .text()
        .await?)
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
