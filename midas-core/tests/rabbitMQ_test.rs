// RabbitMQ消息发送模块单元测试

use midas_core::{message_queue::publish_tick_data, mq::setup_message_queue}; // 新增RabbitMQ连接与发布函数导入
use lapin::{ConnectionProperties, ConsumerDelegate, options::BasicConsumeOptions}; // RabbitMQ消费者依赖
use tokio::sync::oneshot; // 用于测试同步

#[tokio::test]
async fn test_rabbitmq_message_publish() {
    // 步骤1：初始化RabbitMQ连接
    let conn = setup_message_queue("amqp://localhost:5672").await.expect("连接RabbitMQ失败");
    let channel = conn.create_channel().await.expect("创建Channel失败");

    // 步骤2：声明测试队列（避免因队列不存在导致发送失败）
    channel.queue_declare(
        "test_queue",
        Default::default(),
        Default::default(),
        ConnectionProperties::default(),
    ).await.expect("声明队列失败");

    // 步骤3：发送测试消息
    let test_msg = "test_message"; // 待发送的消息内容
    let result = publish_tick_data(&channel, "test_queue", test_msg).await;
    assert!(result.is_ok(), "消息发布失败");

    // 步骤4：消费消息验证（同步等待5秒）
    let (tx, rx) = oneshot::channel();
    let mut consumer = channel
        .basic_consume(
            "test_queue",
            "test_consumer",
            BasicConsumeOptions::default(),
            ConnectionProperties::default(),
        )
        .await
        .expect("创建消费者失败");

    tokio::spawn(async move {
        if let Some(delivery) = consumer.next().await {
            let (_, delivery) = delivery.expect("接收消息失败");
            let received_msg = String::from_utf8_lossy(&delivery.data);
            tx.send(received_msg).expect("发送验证结果失败");
        }
    });

    let received_msg = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        rx,
    ).await.expect("等待消息超时").expect("未接收到消息");

    // 步骤5：断言消息一致性
    assert_eq!(received_msg, test_msg, "发送与接收消息内容不一致");
}