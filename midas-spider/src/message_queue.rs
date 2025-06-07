use lapin::{Connection, ConnectionProperties};
use tokio_amqp::LapinTokioExt;

pub async fn setup_message_queue() -> Result<Connection, Box<dyn std::error::Error>> {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default().with_tokio(),
    )
    .await?;
    
    Ok(conn)
}


pub async fn publish_tick_data(
    conn: &Connection,
    queue_name: &str,
    tick_data: &TickData,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = conn.create_channel().await?;
    
    channel
        .queue_declare(
            queue_name,
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;
    
    let payload = serde_json::to_vec(tick_data)?;
    
    channel
        .basic_publish(
            "",
            queue_name,
            lapin::options::BasicPublishOptions::default(),
            payload,
            lapin::basic::BasicProperties::default(),
        )
        .await?;
    
    Ok(())
}