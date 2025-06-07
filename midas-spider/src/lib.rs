use rayon::prelude::*;
use tokio::fs;
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(serde::Deserialize)]
struct EastmoneyResponse {
    // pub rc: i64,
    // pub rt: i64,
    // pub svr: i64,
    // pub lt: i64,
    // pub full: i64,
    // pub dlmkts: String,
    pub data: EastmoneyResponseData,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EastmoneyResponseData {
    pub code: String,
    // pub market: i64,
    // pub name: String,
    // pub decimal: i64,
    // pub dktotal: i64,
    // pub pre_k_price: f64,
    pub klines: Vec<String>,
}

pub async fn fetch_data(index_code: &midas_core::model::IndexCode, client: &reqwest::Client) {
    tracing::info!("fetch {} date -> begin", index_code.code);

    let url = format!(
        "https://push2his.eastmoney.com/api/qt/stock/kline/get?secid={}&fields1=f1%2Cf2%2Cf3%2Cf4%2Cf5%2Cf6&fields2=f51%2Cf52%2Cf53%2Cf54%2Cf55%2Cf56%2Cf57%2Cf58%2Cf59%2Cf60%2Cf61&klt=101&fqt=1&beg=0&end=20500101&lmt=120",
        index_code.secid
    );

    let response = client.get(url).send().await.unwrap();

    let eastmoney_response = response.json::<EastmoneyResponse>().await.unwrap();

    let index_data_list = eastmoney_response
        .data
        .klines
        .par_iter()
        .map(|item| {
            let item_split_vec = item.split(',').collect::<Vec<&str>>();
            let date = item_split_vec[0];
            // let open_point = item_split_vec[1];
            let close_point = item_split_vec[2];
            // let high_point = item_split_vec[3];
            // let low_point = item_split_vec[4];
            // let volume = item_split_vec[5];
            // let amount = item_split_vec[6];
            // let amplitude = item_split_vec[7];
            // let chg_ratio = item_split_vec[8];
            // let chg = item_split_vec[9];
            // let turnover_rate = item_split_vec[10];

            midas_core::model::IndexData {
                date: date.to_string(),
                close_point: close_point.parse().unwrap(),
            }
        })
        .collect::<Vec<midas_core::model::IndexData>>();

    fs::write(
        format!("index-data/{}.json", eastmoney_response.data.code),
        serde_json::to_string_pretty(&index_data_list).unwrap(),
    )
    .await
    .unwrap();

    tracing::info!("fetch {} date <- end", index_code.code);
}

// 修改数据请求函数，从日级改为tick级
pub async fn fetch_tick_data(
    symbol: &str,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    client: &reqwest::Client
) -> Result<Vec<TickData>, DataError> {
    let url = format!(
        "https://api.marketdata.com/v1/tick/{symbol}?start={start}&end={end}",
        symbol = symbol,
        start = start_time.to_rfc3339(),
        end = end_time.to_rfc3339()
    );
    
    // 设置低延迟HTTP客户端
    let client = reqwest::Client::builder()
        .tcp_nodelay(true)  // 禁用Nagle算法
        .pool_max_idle_per_host(0)  // 禁用连接池
        .timeout(Duration::from_millis(500))  // 设置超时
        .build()?;
    
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<TickDataResponse>()
        .await?;
    
    Ok(response.data)
}

pub async fn save_tick_data(tick_data: &[TickData], symbol: &str) -> Result<(), DataError> {
    fs::write(
        format!("tick-data/{}.json", symbol),
        serde_json::to_string_pretty(tick_data)?,
    )
    .await?;
    
    Ok(())
}

#[derive(Error, Debug)]
pub enum DataError {
    #[error("HTTP request failed")]
    RequestError(#[from] reqwest::Error),
    #[error("JSON parsing failed")]
    ParseError(#[from] serde_json::Error),
    #[error("IO operation failed")]
    IoError(#[from] std::io::Error),
}

// 定义Tick数据结构
#[derive(Debug, Deserialize, Serialize)]
pub struct TickData {
    pub timestamp: DateTime<Utc>,
    pub price: f64,
    pub volume: i64,
    pub bid: f64,
    pub ask: f64,
    pub bid_size: i64,
    pub ask_size: i64,
}

#[derive(Debug, Deserialize)]
struct TickDataResponse {
    pub data: Vec<TickData>,
}
