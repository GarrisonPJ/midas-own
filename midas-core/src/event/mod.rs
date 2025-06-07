use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    TickData {
        symbol: String,
        price: f64,
        volume: f64,
        timestamp: u64,
    },
    OrderBookUpdate {
        symbol: String,
        bids: Vec<(f64, f64)>,
        asks: Vec<(f64, f64)>,
    },
    Trade {
        symbol: String,
        price: f64,
        quantity: f64,
        side: OrderSide,
    },
    StrategySignal {
        symbol: String,
        signal: SignalType,
        strength: f64,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide { Buy, Sell }

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub enum SignalType { EnterLong, EnterShort, Exit }

pub mod stream {
    pub use super::super::stream::*;
}

// 为Event实现流转换
impl Event {
    pub fn into_stream(self) -> DataStream<Self> {
        let (stream, tx) = DataStream::new(1024);
        tokio::spawn(async move {
            tx.send(self).await.unwrap();
        });
        stream
    }
}

// 添加网络延迟计算任务
pub async fn calculate_network_latency() -> f64 {
    // 使用Ping测量网络延迟
    let start = std::time::Instant::now();
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    start.elapsed().as_secs_f64() * 1000.0
}

// 添加拓扑结构分析任务
pub async fn analyze_topology() -> Vec<String> {
    // 模拟获取网络拓扑信息
    vec!["node1", "node2", "node3"]
}

// 集成到事件处理流程
pub async fn process_event(event: Event) {
    let latency = calculate_network_latency().await;
    let topology = analyze_topology().await;
    
    // 根据网络状况优化事件处理
    tokio::spawn(async move {
        match event {
            Event::TickData { symbol, price, volume, timestamp } => {
                // 使用SIMD优化价格计算
                let avg_price = price * volume;
                println!("Processed tick: {} at {} with volume {}", symbol, avg_price, volume);
            }
            Event::OrderBookUpdate { symbol, bids, asks } => {
                // 优化订单簿更新处理
                let spread = bids[0].0 - asks[0].0;
                println!("Processed order book update: {} with spread {}", symbol, spread);
            }
            _ => {}
        }
    });
}