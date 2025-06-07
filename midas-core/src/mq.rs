use lapin::ConnectionProperties;
use serde::Serialize;
use tokio::sync::Mutex;
use std::sync::Arc;

/// RabbitMQ连接封装结构
#[derive(Debug)]
pub struct MqClient {
    pub channel: Arc<Mutex<lapin::Channel>>,
}

impl MqClient {
    /// 创建新的RabbitMQ连接实例
    pub async fn new(amqp_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = lapin::Connection::connect(amqp_url, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

        Ok(Self {
            channel: Arc::new(Mutex::new(channel)),
        })
    }

    /// 异步发布消息到指定队列
    pub async fn publish<T: Serialize>(&self, queue: &str, data: &T) -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::to_vec(data)?;
        let mut channel = self.channel.lock().await;

        // 确保队列存在
        channel.queue_declare(
            queue,
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
            ConnectionProperties::default(),
        ).await?;

        // 发布消息
        channel.basic_publish(
            "",
            queue,
            lapin::options::BasicPublishOptions::default(),
            payload,
            lapin::types::FieldTable::default(),
        ).await?;
        Ok(())
    }
}