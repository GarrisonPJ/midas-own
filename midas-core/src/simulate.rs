
use lapin::ConnectionProperties;
use serde_json;
use tokio::task;
use midas_core::mq::MqClient;

/// 异步发送消息到RabbitMQ
pub async fn send_to_rabbitmq(amqp_url: &str, queue: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化RabbitMQ连接（实际生产建议复用连接）
    let mq_client = MqClient::new(amqp_url).await?;
    mq_client.publish(queue, message).await
}

/// 深度分析完成后发送结果到RabbitMQ
pub async fn depth_analysis_and_send(amqp_url: &str, queue: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = depth_analysis(data).await?;
    send_to_rabbitmq(amqp_url, queue, &result).await
}

/// 价差分析完成后发送结果到RabbitMQ
pub async fn spread_analysis_and_send(amqp_url: &str, queue: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = spread_analysis(data).await?;
    send_to_rabbitmq(amqp_url, queue, &result).await
}

/// 流动性分析完成后发送结果到RabbitMQ
pub async fn liquidity_analysis_and_send(amqp_url: &str, queue: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = liquidity_analysis(data).await?;
    send_to_rabbitmq(amqp_url, queue, &result).await
}

/// 订单流分析完成后发送结果到RabbitMQ
pub async fn orderflow_analysis_and_send(amqp_url: &str, queue: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = orderflow_analysis(data).await?;
    send_to_rabbitmq(amqp_url, queue, &result).await
}
