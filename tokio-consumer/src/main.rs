use clap::{Command, Arg};
use log::{info, warn};

use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::message::Message;
use rdkafka::util::get_rdkafka_version;

use lib::utils::setup_logger;
use lib::context::CustomContext;

/// https://github.com/fede1024/rust-rdkafka/blob/master/examples/simple_consumer.rs

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume(brokers: &str, group_id: &str, topics: &str) {
    let context = CustomContext;

    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&topics])
        .expect("Can't subscribe to specified topics");

    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                // if let Some(headers) = m.headers() {
                //     for header in headers.iter() {
                //         info!("  Header {:#?}: {:?}", header.key, header.value);
                //     }
                // }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}

#[tokio::main]
async fn main() {
    let matches = Command::new("tokio-consumer")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Simple command line consumer")
        .arg(
            Arg::new("brokers")
                .short('b')
                .long("brokers")
                .help("Broker list in kafka format")
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::new("group-id")
                .short('g')
                .long("group-id")
                .help("Consumer group id")
                .default_value("example_consumer_group_id"),
        )
        .arg(
            Arg::new("log-conf")
                .long("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
        )
        .arg(
            Arg::new("topics")
                .short('t')
                .long("topics")
                .help("Topic list")
                .required(true),
        )
        .get_matches();

    setup_logger(true, matches.get_one::<String>("log-conf"));

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let topics = matches.get_one::<String>("topics").unwrap();
    let brokers = matches.get_one::<String>("brokers").unwrap();
    let group_id = matches.get_one::<String>("group-id").unwrap();

    consume(brokers, group_id, &topics).await
}
