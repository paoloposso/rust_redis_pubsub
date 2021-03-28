use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel: String,
    pub payload: Order
}

impl Message {
    pub fn new(payload: Order) -> Message {
        Message {
            id: Message::generate_id(),
            channel: String::from("order"),
            payload
        }
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    description: String,
    quantity: u16,
    total_price: f32
}

impl Order {
    pub fn new(description: String, quantity: u16, total_price: f32) -> Order {
        Order { description, quantity, total_price }
    }
}
