use clap::{Arg, Command, ArgMatches};

pub fn get_matches() -> ArgMatches {
    Command::new("async-std runtime example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Demonstrates using rust-rdkafka with async-std")
        .arg(
            Arg::new("brokers")
                .help("Broker list in kafka format")
                .short('b')
                .long("brokers")
                .default_value("localhost:9092")
        )
        .arg(
            Arg::new("group-id")
                .help("Consumer group id")
                .short('g')
                .long("group-id")
                .default_value("async-std")
        )
        .arg(
            Arg::new("input-topic")
                .help("topic to consume from")
                .long("input-topic")
                .short('i')
                .required(true)
        )
        .arg(
            Arg::new("output-topic")
                .help("topic to send events to")
                .long("output-topic")
                .short('o')
                .required(true),
        )
        .arg(
            Arg::new("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .long("log-conf")
        )
        .get_matches()
}
