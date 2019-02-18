use rocketmq_client_rust::core::*;

fn main() {
    let producer = Producer::new("producer_test_group", "producer1", "172.16.208.204:9876");
    producer.start();
    producer.send("test", "abcdefg", "", "");
}