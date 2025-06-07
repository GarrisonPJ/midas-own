use midas_spider::{fetch_tick_data, TickData};
use midas_spider::message_queue::{setup_message_queue, publish_tick_data};
use chrono::{Utc, TimeDelta};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化Rayon线程池
    midas_core::init_rayon_pool();
    
    // 设置消息队列连接
    let conn = setup_message_queue().await?;
    
    // 创建HTTP客户端
    let client = Client::new();
    
    // 获取最近1小时的tick数据
    let end_time = Utc::now();
    let start_time = end_time - TimeDelta::hours(1);
    
    let tick_data = fetch_tick_data("AAPL", start_time, end_time, &client).await?;
    
    // 发布每条tick数据到消息队列
    for tick in tick_data {
        publish_tick_data(&conn, "tick_data", &tick).await?;
    }
    
    Ok(())
}
