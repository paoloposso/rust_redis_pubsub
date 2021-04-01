extern crate redis;

use std::error::Error;
use redis::{Commands};
use crate::message::Message;

pub fn publish_message(message: Message) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://localhost/")?;
    let mut con = client.get_connection()?;

    let json = serde_json::to_string(&message)?;

    con.publish(message.channel, json)?;

    Ok(())
}