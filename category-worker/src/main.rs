use rdkafka::Message;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use serde_json::Value;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = utils::get_config();
    let consumer: StreamConsumer = config.set("group.id", "rust_example_group_1").create()?;

    consumer.subscribe(&["category.create"])?;

    loop {
        match consumer.recv().await {
            Err(err) => {
                eprintln!("Kafka error: {}", err);
            }
            Ok(message) => {
                let Some(payload) = message.payload() else {
                    if let Err(err) = consumer.store_offset_from_message(&message) {
                        eprintln!("Error while storing offset: {:?}", err);
                    }
                    continue;
                };
                // remove unwrap
                let parsed_payload: Value =
                    serde_json::from_str(std::str::from_utf8(payload).unwrap())?;

                eprintln!("New message: {}", parsed_payload["orderid"]);
                if let Err(err) = consumer.store_offset_from_message(&message) {
                    eprintln!("Error while storing offset: {:?}", err);
                }
            }
        }
    }
}
