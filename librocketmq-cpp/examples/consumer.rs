use rocketmq_client_rust::core::*;

fn main() {
    let consumer = PushConsumer::new("producer_test_group", "producer1", "172.16.208.204:9876");
    consumer.subscribe("test", "*");
    consumer.start();
}