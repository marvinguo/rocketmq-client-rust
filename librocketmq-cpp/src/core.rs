use std::mem::drop;
use std::ffi::CString;
use crate::bindings::*;

pub struct PushConsumer {
    consumer: *mut CPushConsumer,
}

impl PushConsumer {
    pub fn new(group_id: &str, instance_name: &str, name_server: &str) -> PushConsumer {
        unsafe {
            let consumer_ptr = CreatePushConsumer(CString::new(group_id).unwrap().as_ptr());
            SetPushConsumerInstanceName(consumer_ptr, CString::new(instance_name).unwrap().as_ptr());
            SetPushConsumerNameServerAddress(consumer_ptr, CString::new(name_server).unwrap().as_ptr());
            PushConsumer { consumer: consumer_ptr }
        }
    }

    pub fn start(&self) {
        unsafe {
            StartPushConsumer(self.consumer);
        }
    }

    pub fn subscribe(&self, topic: &str, expression: &str) {
        unsafe {
            Subscribe(self.consumer, CString::new(topic).unwrap().as_ptr(), CString::new(expression).unwrap().as_ptr());
            RegisterMessageCallback(self.consumer, doConsumeMessage);
        }
    }
}

impl Drop for PushConsumer {
    fn drop(&mut self) {
        unsafe {
            ShutdownPushConsumer(self.consumer);
            DestroyPushConsumer(self.consumer);
        }
    }
}

pub struct Producer {
    producer: *mut CProducer,
}

impl Producer {
    pub fn new(group_id: &str, instance_name: &str, name_server: &str) -> Producer {
        unsafe {
            let producer_ptr = CreateProducer(CString::new(group_id).unwrap().as_ptr());
            SetProducerInstanceName(producer_ptr, CString::new(instance_name).unwrap().as_ptr());
            SetProducerNameServerAddress(producer_ptr, CString::new(name_server).unwrap().as_ptr());
            Producer { producer: producer_ptr }
        }
    }

    pub fn start(&self) {
        unsafe {
            StartProducer(self.producer);
        }
    }

    pub fn send(&self, topic: &str, body: &str, tags: &str, keys: &str) {
        unsafe {
            let message_ptr = CreateMessage(CString::new(topic).unwrap().as_ptr());
            SetMessageBody(message_ptr, CString::new(body).unwrap().as_ptr());
            SetMessageTags(message_ptr, CString::new(tags).unwrap().as_ptr());
            SetMessageKeys(message_ptr, CString::new(keys).unwrap().as_ptr());

            let mut message_result = CSendResult { sendStatus: 0, msgId: [0; 256usize], offset: 0 };
            SendMessageSync(self.producer, message_ptr, &mut message_result);
        }
    }
}

impl Drop for Producer {
    fn drop(&mut self) {
        unsafe {
            ShutdownProducer(self.producer);
            DestroyProducer(self.producer);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::*;

    #[test]
    fn test_producer() {
        println!("1321");

        let x = 100;
        let producer = Producer::new("132", "123", "172.16.208.204:9876");
        println!("1321");
        producer.send("ttt", "", "", "");
    }
}

