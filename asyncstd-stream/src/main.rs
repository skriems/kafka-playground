use std::time::Duration;

use futures::stream::StreamExt;

use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::message::Message;
use rdkafka::producer::FutureRecord;

use serde::{Deserialize, Serialize};

use common::async_std::{create_consumer, create_producer};
use common::utils::setup_logger;
use log::{error, info, warn};

mod cli;

/// https://github.com/fede1024/rust-rdkafka/blob/master/examples/runtime_async_std.rs

#[derive(Debug, Deserialize, Serialize)]
struct Event<'a> {
    kind: &'a str,
    amount: i32,
}

struct Amount(i32);

impl Amount {
    fn new(amount: i32) -> Self {
        Amount(amount)
    }

    fn add(&mut self, amount: i32) {
        self.0 += amount;
    }

    fn sub(&mut self, amount: i32) {
        self.0 -= amount;
    }
}

async fn run(brokers: &str, group_id: &str, input_topic: &str, output_topic: &str) {

    let consumer = create_consumer(&brokers, &group_id);
    let producer = create_producer(&brokers);

    consumer.subscribe(&[&input_topic]).unwrap();
    let mut stream = consumer.stream();

    let mut amount = Amount::new(0);

    loop {
        match stream.next().await {
            Some(Ok(message)) => match message.payload_view::<str>() {
                Some(Ok(string)) => match serde_json::from_str::<Event>(string) {
                    Ok(event) => {
                        info!(
                        "Processing message {}, key: {:?}, topic: {}, partition: {}, {:?}: kind: {}, amount: {}",
                        message.offset(),
                        message.key(),
                        message.topic(),
                        message.partition(),
                        message.timestamp(),
                        event.kind, event.amount
                    );

                        match event.kind {
                            "add" => amount.add(event.amount),
                            "sub" => amount.sub(event.amount),
                            _ => warn!("Unknown event kind: {}", event.kind),
                        }

                        if message.offset() % 10 == 0 {
                            let delivery_status = producer
                                .send::<Vec<u8>, _, _>(
                                    FutureRecord::to(&output_topic).payload(&serde_json::json!({
                                        "amount": amount.0,
                                        "version": message.offset()
                                    }).to_string()),
                                    Duration::from_secs(0),
                                )
                                .await;

                            if let Err((e, _)) = delivery_status {
                                error!("Unable to send message: {}", e);
                            }
                            consumer
                                .commit_message(&message, CommitMode::Async)
                                .unwrap();
                            info!("Committed offset: {}", message.offset());
                        }
                    }

                    Err(parse_error) => {
                        error!("Cannot parse message {:?}: {:?}", string, parse_error);
                    }
                },
                Some(Err(_)) => error!("Message is not utf-8 encoded"),
                None => warn!("Got message without payload"),
            },
            Some(Err(e)) => {
                error!("Error receiving message: {}", e);
            }
            None => {
                warn!("Consumer unexpectedly returned no messages");
            }
        }
    }
}

#[async_std::main]
async fn main() {
    let matches = cli::get_matches();

    setup_logger(true, matches.get_one::<String>("log-conf"));

    let brokers = matches.get_one::<String>("brokers").unwrap();
    let group_id = matches.get_one::<String>("group-id").unwrap();
    let input_topic = matches.get_one::<String>("input-topic").unwrap();
    let output_topic = matches.get_one::<String>("output-topic").unwrap();

    run(brokers, group_id, &input_topic, &output_topic).await;
}
