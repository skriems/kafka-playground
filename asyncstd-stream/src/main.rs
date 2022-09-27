use std::future::Future;
use std::pin::Pin;
use std::process;
use std::time::Duration;

use clap::{App, Arg};
use futures::stream::StreamExt;

use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::AsyncRuntime;

use common::utils::setup_logger;
use common::context::CustomContext;

/// https://github.com/fede1024/rust-rdkafka/blob/master/examples/runtime_async_std.rs

pub struct AsyncStdRuntime;

impl AsyncRuntime for AsyncStdRuntime {
    type Delay = Pin<Box<dyn Future<Output = ()> + Send>>;

    fn spawn<T>(task: T)
    where
        T: Future<Output = ()> + Send + 'static,
    {
        async_std::task::spawn(task);
    }

    fn delay_for(duration: Duration) -> Self::Delay {
        Box::pin(async_std::task::sleep(duration))
    }
}

#[async_std::main]
async fn main() {
    let matches = App::new("async-std runtime example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Demonstrates using rust-rdkafka with a custom async runtime")
        .arg(
            Arg::with_name("brokers")
                .short('b')
                .long("brokers")
                .help("Broker list in kafka format")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::with_name("group-id")
                .short('g')
                .long("group-id")
                .help("Consumer group id")
                .takes_value(true)
                .default_value("async-std-consumer"),
        )
        .arg(
            Arg::with_name("topic")
                .long("topic")
                .help("topic")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("log-conf")
                .long("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .takes_value(true),
        )
        .get_matches();

    setup_logger(true, matches.value_of("log-conf"));

    let brokers = matches.value_of("brokers").unwrap();
    let topic = matches.value_of("topic").unwrap().to_owned();
    let group_id = matches.value_of("group-id").unwrap();

    let producer_context = CustomContext;
    let producer: FutureProducer<CustomContext, AsyncStdRuntime> = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create_with_context(producer_context)
        .expect("Producer creation error");

    let delivery_status = producer
        .send::<Vec<u8>, _, _>(
            FutureRecord::to(&topic).payload("hello from async-std"),
            Duration::from_secs(0),
        )
        .await;

    if let Err((e, _)) = delivery_status {
        eprintln!("unable to send message: {}", e);
        process::exit(1);
    }

    let consumer_context = CustomContext;
    let consumer: StreamConsumer<CustomContext, AsyncStdRuntime> = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set("group.id", group_id)
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(consumer_context)
        .expect("Consumer creation failed");
    consumer.subscribe(&[&topic]).unwrap();

    let mut stream = consumer.stream();

    loop {
        match stream.next().await {
            Some(Ok(message)) => println!(
                "Received message: {}",
                match message.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(_)) => "<invalid utf-8>",
                }
            ),
            Some(Err(e)) => {
                eprintln!("Error receiving message: {}", e);
                process::exit(1);
            }
            None => {
                eprintln!("Consumer unexpectedly returned no messages");
                process::exit(1);
            }
        }
    }
}
