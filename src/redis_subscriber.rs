extern crate redis;

use std::error::Error;
use redis::{ControlFlow, PubSubCommands};
use crate::message::Message;
use crate::message_handler;

pub fn subscribe(channel: String) -> Result<(), Box<dyn Error>> {
    let _ = tokio::spawn(async move {
        let client = redis::Client::open("redis://localhost").unwrap();
        
        let mut con = client.get_connection().unwrap();

        let _: () = con.subscribe(&[channel], |msg| {            
            let received: String = msg.get_payload().unwrap();
            let message_obj = serde_json::from_str::<Message>(&received).unwrap();
            
            message_handler::handle(message_obj);

            return ControlFlow::Continue;
        }).unwrap();
    });

    Ok(())
}
