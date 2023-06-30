use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;
use std::thread;

pub struct LinkedBlockingQueue<T> {
    queue: Arc<(Mutex<VecDeque<T>>, Condvar)>,
}

impl<T> LinkedBlockingQueue<T> {
    pub fn new() -> Self {
        LinkedBlockingQueue {
            queue: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    pub fn put(&self, val: T) {
        let (lock, cvar) = &*self.queue;
        let mut queue = lock.lock().unwrap();
        queue.push_back(val);
        cvar.notify_one();
    }

    pub fn take(&self) -> T {
        let (lock, cvar) = &*self.queue;
        let mut queue = lock.lock().unwrap();
        while queue.is_empty() {
            queue = cvar.wait(queue).unwrap();
        }
        queue.pop_front().unwrap()
    }
}

