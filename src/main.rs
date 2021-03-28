mod message;
mod message_handler;
mod redis_service;

extern crate redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("service started");

    if let Err(error) = redis_service::start_subscription(String::from("order")) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue");
    }

    redis_service::publish_message(message::Message::new(
        message::Order::new("T-Shirt".to_string(), 
        3, 
        24.0)))?;
    redis_service::publish_message(message::Message::new(
        message::Order::new("Sneakers".to_string(), 
        1, 
        230.0)))?;
    redis_service::publish_message(message::Message::new(
        message::Order::new("Milka Bar".to_string(), 
        10, 
        50.0)))?;

    Ok(())
}
