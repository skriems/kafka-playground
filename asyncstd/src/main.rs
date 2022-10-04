use clap::ArgMatches;

use futures::stream::StreamExt;

use rdkafka::message::Message;
use rdkafka::consumer::Consumer;

use lib::async_std::{create_consumer, create_producer};
use lib::utils::setup_logger;
use log::{error, info, warn};

use serde::{Deserialize, Serialize};
use zeou::events;

mod cli;

/// https://github.com/fede1024/rust-rdkafka/blob/master/examples/runtime_async_std.rs

/// A Command sent to the woker
#[derive(Debug, Deserialize, Serialize)]
struct Command<'a> {
    /// kind of the command
    kind: &'a str,
}

async fn process(matches: &ArgMatches) {
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
                        Ok(command) => {
                            match command.kind {
                                "createEvent" => events::process_message(message, &consumer, &producer).await,
                                _ => warn!("Unknown command: {}", command.kind),
                            }
                        },
                        Err(error) => error!("Error deserializing message: {}", error),
                    },
                    Some(Err(error)) => error!("Error reading message: {}", error),
                    None => warn!("Error: no message?"),
                }
            },
            Some(Err(error)) => error!("Error receiving message: {}", error),
            None => warn!("Consumer unexpectedly returned no messages"),
        }
    }
}

#[async_std::main]
async fn main() {
    let matches = cli::get_matches();

    match matches.subcommand() {
        Some(("process", sub_matches)) => {
            setup_logger(true, sub_matches.get_one::<String>("log-conf"));
            process(sub_matches).await;
        }
        _ => {
            println!("not implemented");
        }
    }
}
