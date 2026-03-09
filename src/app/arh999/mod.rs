
use anyhow::{Ok, Result, bail};
use serde::Deserialize;
use crate::app::lark::lark_client;


#[derive(Debug)]
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Response {
    code: i32,
    msg: String,
    data:Vec<DataPoint>,
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct DataPoint (
    f64,
    f32,
    f32,
    f32,
    f32,
);

pub async fn get_index () -> Result<String> {
    let body = reqwest::get("https://dncapi.flink1.com/api/v2/index/arh999?code=bitcoin")
    .await?
    .text()
    .await?;

    let resp: Response = serde_json::from_str(&body)?;
    if resp.code == 200 {
        let last = match resp.data.last() {
            Some(v) => v,
            None => bail!("arh999 data is empty"),
        };

        if last.1 < 0.8 {
            let msg = format!("BTC ahr999指数:{}, 最新价格:{}, 指数增长估值:{}, 200日定投成本:{}", last.1, last.2, last.3, last.4);
            let msg = lark_client::send_msg_by_interval(&msg, "1d").await?;
            if msg != "success" {
                error!("send_msg resp {}", msg);
            }
        }
        Ok(String::from("success"))
    } else {
        Ok(resp.msg)
    }
}