use lapin::{Channel, Connection, ConnectionProperties, options::QueueDeclareOptions, types::{AMQPValue, FieldTable}};

pub async fn create_channel(amqp_url: &str) -> Channel
{
    let conn = Connection::connect(amqp_url, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ");
    let channel = conn.create_channel()
        .await
        .expect("Failed to create channel");

    let mut args = FieldTable::default();  
    args.insert("x-queue-type".into(), AMQPValue::LongString("quorum".into()));  
    channel.queue_declare(
     "score_logs".into(), 
    QueueDeclareOptions{durable: true, ..QueueDeclareOptions::default()}, 
    args)
        .await
        .expect("Failed to declare queue");

    channel
}