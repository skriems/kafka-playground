use clap::{App, Arg, ArgMatches};

pub fn get_matches() -> ArgMatches {
    App::new("async-std runtime example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Demonstrates using rust-rdkafka with async-std")
        .arg(
            Arg::with_name("brokers")
                .help("Broker list in kafka format")
                .short('b')
                .long("brokers")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::with_name("group-id")
                .help("Consumer group id")
                .short('g')
                .long("group-id")
                .takes_value(true)
                .default_value("async-std"),
        )
        .arg(
            Arg::with_name("input-topic")
                .help("topic to consume from")
                .long("input-topic")
                .short('i')
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output-topic")
                .help("topic to send events to")
                .long("output-topic")
                .short('o')
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .long("log-conf")
                .takes_value(true),
        )
        .get_matches()
}
