use anyhow::Result;
use soup::prelude::*;

const URL: &str = "https://www.dongao.com/zckjs/zkz/202107063478415.shtml";

#[tokio::main]
async fn main() -> Result<()> {
    let status = get_status_of_js(&get_document().await?);
    if status != "暂未开通" {
        println!("The status is {} now!", status);
        std::process::exit(0);
    } else {
        eprintln!("The status it still 暂未开通.");
        std::process::exit(1);
    }
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
    println!("{}", js_strong_node.text());
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
    println!("status: {}", status.trim());
    status.trim().to_string()
}
