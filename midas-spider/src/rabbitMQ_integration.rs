use lapin::{Connection, ConnectionProperties, ConsumerDelegate, Message, options::*, types::FieldTable};
use midas_core::event::{Event, system::EventSystem};
use serde_json::from_slice;
use std::sync::Arc;

pub async fn start_rabbitmq_consumer(event_system: Arc<EventSystem>) {
    let conn = Connection::connect("amqp://localhost:5672", ConnectionProperties::default())
        .await
        .expect("RabbitMQ connection failed");
    let channel = conn.create_channel().await.expect("Channel creation failed");

    channel.queue_declare(
        "market-data",
        QueueDeclareOptions::default(),
        FieldTable::default(),
    ).await.expect("Queue declaration failed");

    let mut consumer = channel
        .basic_consume(
            "market-data",
            "midas-consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Consume start failed");

    while let Some(delivery) = consumer.next().await {
        if let Ok((_, msg)) = delivery {
            match from_slice::<Event>(&msg.data) {
                Ok(event) => {
                    event_system.get_sender().send(event).await
                        .expect("Failed to send event");
                    channel.basic_ack(msg.delivery_tag, BasicAckOptions::default())
                        .await.expect("Ack failed");
                },
                Err(e) => {
                    eprintln!("Message deserialization error: {}", e);
                    channel.basic_nack(msg.delivery_tag, BasicNackOptions::default())
                        .await.expect("Nack failed");
                }
            }
        }
    }
}