use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;
use rdkafka::util::AsyncRuntime;

use crate::context::CustomContext;

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

pub fn create_consumer(
    brokers: &str,
    group_id: &str,
) -> StreamConsumer<CustomContext, AsyncStdRuntime> {
    let context = CustomContext;
    let consumer: StreamConsumer<CustomContext, AsyncStdRuntime> = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set("group.id", group_id)
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");
    consumer
}


pub fn create_producer(
    brokers: &str,

) -> FutureProducer<CustomContext, AsyncStdRuntime> {
    let context = CustomContext;
    let producer: FutureProducer<CustomContext, AsyncStdRuntime> = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create_with_context(context)
        .expect("Producer creation error");
    producer
}
