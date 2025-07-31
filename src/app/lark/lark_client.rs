use serde::Deserialize;
use anyhow::Result;
use serde_json::json;
use crate::app::config::config;

#[derive(Debug)]
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Response {
    code: i32,
    msg: String,
}

// https://open.larksuite.com/document/client-docs/bot-v3/add-custom-bot#f62e72d5
pub async fn send_msg (msg: &str, webook: &str) -> Result<String> {

    // 构建JSON请求体
    // "request example, <at user_id=\"all\">所有人</at> "
    let request_body = json!({
        "msg_type":"text",
        "content":{"text":msg}});

    let url = format!("{}{}", "https://open.larksuite.com/open-apis/bot/v2/hook/", webook);
    let client = reqwest::Client::new();
    let res = client.post(url).json(&request_body).send().await?;

    let resp: Response = serde_json::from_str(&res.text().await?)?;
    info!("send_msg {}", msg);
    Ok(resp.msg)
}

pub async fn send_msg_1w (msg: &String) -> Result<String> {
    let webhook = config().settings.lark_bot_webhook_1w.clone();
    send_msg(msg,&webhook).await
}

pub async fn send_msg_1d (msg: &String) -> Result<String> {
    let webhook = config().settings.lark_bot_webhook_1d.clone();
    send_msg(msg, &webhook).await
}

pub async fn send_msg_4h (msg: &String) -> Result<String> {
    let webhook = config().settings.lark_bot_webhook_4h.clone();
    send_msg(msg, &webhook).await
}

pub async fn send_msg_by_interval (msg: &String, interval: &str) -> Result<String> {
    if interval == "1w" {
        return send_msg_1w(msg).await;
    } else if interval == "1d" {
        return send_msg_1d(msg).await;
    } else if interval == "4h" {
        return send_msg_4h(msg).await;
    } else {
        let webhook = config().settings.lark_bot_webhook.clone();
        return send_msg(msg, &webhook).await
    }
}
