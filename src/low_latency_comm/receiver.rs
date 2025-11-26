use super::spsc::{SPSC};

pub struct Receiver<T> {
    queue: *const SPSC<T>,
}

unsafe impl <T: Send> Send for Receiver<T> {}

impl <T> Receiver<T> {
     pub fn new(queue_pointer: *const SPSC<T>) -> Self {
        Receiver {
            queue: queue_pointer
        }
    }

    pub fn recv(&self) -> Result<T, ()> {
        let core_queue = unsafe {
            &(*self.queue)
        };

        core_queue.try_pop()
    }
}