extern crate redis;

use redis::{Client, Commands, ControlFlow, PubSubCommands};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

trait AppState {
    fn client(&self) -> &Arc<Client>;
}

struct Ctx {
    pub client: Arc<Client>,
}

impl Ctx {
    fn new() -> Ctx {
        let client = Client::open("redis://localhost/").unwrap();
        Ctx {
            client: Arc::new(client),
        }
    }
}

impl AppState for Ctx {
    fn client(&self) -> &Arc<Client> {
        &self.client
    }
}

fn subscribe(state: &impl AppState) -> thread::JoinHandle<()> {
    let client = Arc::clone(state.client());
    thread::spawn(move || {
        let mut conn = client.get_connection().unwrap();

        conn.subscribe(&["boo"], |msg| {
            let ch = msg.get_channel_name();
            let payload: String = msg.get_payload().unwrap();
            match payload.as_ref() {
                "10" => ControlFlow::Break(()),
                a => {
                    println!("Channel '{}' received '{}'.", ch, a);
                    ControlFlow::Continue
                }
            }
        }).unwrap();
    })
}

fn publish(state: &impl AppState) {
    let client = Arc::clone(state.client());
    thread::spawn(move || {
        let mut conn = client.get_connection().unwrap();

        for x in 0..11 {
            thread::sleep(Duration::from_millis(500));
            println!("Publish {} to boo.", x);
            let _: () = conn.publish("boo", x).unwrap();
        }
    });
}

fn main() {
    let ctx = Ctx::new();
    let handle = subscribe(&ctx);
    // publish(&ctx);
    handle.join().unwrap();
}
