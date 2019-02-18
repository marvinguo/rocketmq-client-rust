use rocketmq_client_rust::core::*;
use std::{thread, time};

fn main() {
    let consumer = PushConsumer::new("consumer_test_group", "consumer1", "172.16.208.204:9876");
    consumer.subscribe("test", "*");
    consumer.start();
    thread::sleep(time::Duration::from_secs(100000000));
}