use midas_core::event::system::EventSystem;
use midas_spider::kafka_integration::start_kafka_consumer;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let event_system = Arc::new(EventSystem::new());
    
    // 启动事件分发
    let es_clone = event_system.clone();
    tokio::spawn(async move {
        es_clone.run().await;
    });

    // 启动Kafka消费者
    tokio::spawn(start_kafka_consumer(event_system.clone()));

    // 保持主线程运行
    tokio::signal::ctrl_c().await.unwrap();
}