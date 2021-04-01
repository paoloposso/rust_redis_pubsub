# rust_redis_pubsub

Example of publish and subscription using Redis and Rusg Language.

## Dependencies

[dependencies]
tokio = { version = "*", features = ["full"] }
redis = "*"
uuid = { version = "0.8.2", features = ["v4"] }
serde_json = "*"
serde = {version = "*", features = ["derive"]}

* tokio is used in this case to create an async main method and to access the mutithread features.
* In the case of serde, whose features are used to serialize and deserialize objects, it's important to add the _derive_ features, otherwise it will not be possible to add the Serialize and Deserialize traits on the Derive attribute, in this case, on Message struct, as follows:

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel: String,
    pub payload: Order
}

When you don't add the _derive_ feature, the following error is shown: _cannot find derive macro `Serialize` in this scope_.