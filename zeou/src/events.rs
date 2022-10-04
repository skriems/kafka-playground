use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use lib::async_std::AsyncStdRuntime;
use lib::context::CustomContext;
use log::{error, info, warn};

#[derive(Debug, Deserialize, Serialize)]
struct Command<'a> {
    kind: &'a str,
}

pub async fn process_message(
    message: BorrowedMessage<'_>,
    consumer: &StreamConsumer<CustomContext, AsyncStdRuntime>,
    producer: &FutureProducer<CustomContext, AsyncStdRuntime>,
) {
    match message.payload_view::<str>() {
        Some(Ok(string)) => match serde_json::from_str::<Command>(string) {
            Ok(command) => {
                info!(
                    "Processing message {}, key: {:?}, topic: {}, partition: {}, {:?}: kind: {}",
                    message.offset(),
                    message.key(),
                    message.topic(),
                    message.partition(),
                    message.timestamp(),
                    command.kind
                );

                // match event.kind {
                //     "add" => amount.add(event.amount),
                //     "sub" => amount.sub(event.amount),
                //     _ => warn!("Unknown event kind: {}", event.kind),
                // }

                let delivery_status = producer
                    .send::<Vec<u8>, _, _>(
                        FutureRecord::to("events-processed").payload(
                            &serde_json::json!({
                                "amount": 0,
                                "version": 0
                            })
                            .to_string(),
                        ),
                        Duration::from_secs(0),
                    )
                    .await;

                if let Err((error, _)) = delivery_status {
                    error!("Unable to send message: {}", error);
                }

                consumer
                    .commit_message(&message, CommitMode::Async)
                    .unwrap();
                info!("Committed offset: {}", message.offset());
            }
            Err(parse_error) => error!("Cannot parse message {:?}: {:?}", string, parse_error),
        },
        Some(Err(_)) => error!("Message is not utf-8 encoded"),
        None => warn!("Got message without payload"),
    }
}
