use rocketmq_client_rust::core::*;

fn main(){
    
    let a = 1;
    let producer = Producer::new("132","123","172.16.208.204:9876");
    producer.send("ttt", "", "", "");
}