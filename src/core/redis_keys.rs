pub fn queue_list() -> String {
    String::from("queues")
}

pub fn _queue(queue_name: &str) -> String {
    format!("queue::{}", queue_name)
}

pub fn message(id: &str) -> String {
    format!("message::{}", id)
}

pub fn queued(queue_name: &str) -> String {
    format!("queue::{}::queued", queue_name)
}

pub fn processing() -> String {
    String::from("message::processing")
}

pub fn complete() -> String {
    String::from("message::complete")
}

pub fn failed() -> String {
    String::from("message::failed")
}
