use std::mem::drop;
use std::ffi::CString;
use crate::bindings::*;

pub struct Consumer {
    consumer: *mut CPushConsumer,
}

impl Consumer {
    fn new(group_id: &str, instance_name: &str, name_server: &str) -> Consumer {
        unsafe {
            let consumer_ptr = CreatePushConsumer(CString::new(group_id).unwrap().as_ptr());
            SetPushConsumerInstanceName(consumer_ptr, CString::new(instance_name).unwrap().as_ptr());
            SetPushConsumerNameServerAddress(consumer_ptr, CString::new(name_server).unwrap().as_ptr());
            Consumer { consumer: consumer_ptr }
        }
    }
}

impl Drop for Consumer {
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
    fn new(group_id: &str, instance_name: &str, name_server: &str) -> Producer {
        unsafe {
            let producer_ptr = CreateProducer(CString::new(group_id).unwrap().as_ptr());
            SetProducerInstanceName(producer_ptr, CString::new(instance_name).unwrap().as_ptr());
            SetProducerNameServerAddress(producer_ptr, CString::new(name_server).unwrap().as_ptr());
            Producer { producer: producer_ptr }
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

pub struct Message {
    message: *mut CMessage,
}

impl Message {
    fn new() {}
}