use clap::ArgMatches;

use futures::stream::StreamExt;

use rdkafka::message::Message;
use rdkafka::consumer::Consumer;

use lib::async_std::{create_consumer, create_producer};
use log::{error, info, warn};

use serde::{Deserialize, Serialize};
use zeou::events;

/// A Command sent to the woker
#[derive(Debug, Deserialize, Serialize)]
struct Command<'a> {
    /// kind of the command
    command: &'a str,
}

pub async fn process(matches: &ArgMatches) {
    let brokers = matches.get_one::<String>("brokers").unwrap();
    let domains = matches
        .get_many::<String>("domain")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let group_id = matches.get_one::<String>("group-id").unwrap();

    info!("Starting worker on brokers: {}, domains: {:?}, group_id: {}", brokers, domains, group_id);

    let producer = create_producer(&brokers);
    let consumer = create_consumer(&brokers, &group_id);
    consumer.subscribe(&domains).unwrap();

    let mut stream = consumer.stream();

    loop {
        match stream.next().await {
            Some(Ok(message)) => { 
                match message.payload_view::<str>() {
                    Some(Ok(string)) => match serde_json::from_str::<Command>(string) {
                        Ok(deserialized) => {
                            match deserialized.command {
                                "createEvent" => events::process_message(message, &consumer, &producer).await,
                                _ => warn!("Unhandled command: {}", deserialized.command),
                            }
                        },
                        Err(error) => error!("Error deserializing message: {}", error),
                    },
                    Some(Err(utf8_error)) => error!("Error reading message: {}", utf8_error),
                    None => warn!("Warning: no message?"),
                }
            },
            Some(Err(kafka_error)) => error!("Error receiving message: {}", kafka_error),
            None => warn!("Consumer unexpectedly returned no messages"),
        }
    }
}
