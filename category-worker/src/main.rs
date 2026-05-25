use rdkafka::Message;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::message::BorrowedMessage;
use serde_json::Value;

mod utils;
mod worker;

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
            Ok(mut message) => {
                let topic = message.topic();
                message = {
                    if let Ok((payload, _message)) = validate_received_message(&message) {
                        match topic {
                            "category.create" => worker::create::handler(payload),
                            _ => eprintln!("Topic does not match"),
                        };
                    } else {
                        eprintln!("Falied to process message");
                    }
                    message
                };

                if let Err(err) = consumer.store_offset_from_message(&message) {
                    eprintln!("Error while storing offset: {:?}", err);
                }
            }
        }
    }
}

fn validate_received_message<'a>(
    message: &'a BorrowedMessage,
) -> Result<(Value, &'a BorrowedMessage<'a>), String> {
    let Some(payload) = message.payload() else {
        return Err(String::from("Can't get the message payload"));
    };
    let Ok(parsed_payload) = serde_json::from_slice::<Value>(payload) else {
        return Err(String::from("Can't parse the message"));
    };

    Ok((parsed_payload, message))
}
