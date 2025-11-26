use super::spsc::SPSC;

pub struct Sender<T> {
    queue: *const SPSC<T>,
}

unsafe impl <T: Send> Send for Sender<T> {}

impl <T> Sender<T> {
    pub fn new(queue_pointer: *const SPSC<T>) -> Self {
        Sender {
            queue: queue_pointer
        }
    }

    pub fn send(&self, item: T) -> Result<(), ()> {
        let core_queue = unsafe {
            &(*self.queue)
        };
        core_queue.try_push(item)
    }
}