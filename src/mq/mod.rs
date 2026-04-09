use lapin::{Channel, Connection, ConnectionProperties, options::QueueDeclareOptions, types::{AMQPValue, FieldTable}};

pub async fn create_channel(amqp_url: &str) -> Channel
{
    let conn = Connection::connect(amqp_url, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ");
    let channel = conn.create_channel()
        .await
        .expect("Failed to create channel");

    channel.queue_declare(
     "score_logs".into(), 
    QueueDeclareOptions{durable: true, ..QueueDeclareOptions::default()}, 
    FieldTable::default())
        .await
        .expect("Failed to declare queue");

    channel
}