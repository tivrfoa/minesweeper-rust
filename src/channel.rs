use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

use crate::linked_blocking_queue::LinkedBlockingQueue;

pub struct Channel<T> {
    queue: LinkedBlockingQueue<T>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        let (sender, receiver) = sync_channel::<T>(0);
        Channel { queue: LinkedBlockingQueue::new() }
    }

    pub fn put(&self, val: T) {
        self.queue.put(val);
    }

    pub fn take(&self) -> T {
        self.queue.take()
    }
}

