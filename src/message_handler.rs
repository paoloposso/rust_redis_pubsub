use crate::message::Message;

pub fn handle(order: Message) {
    println!("{:?}", order);
}