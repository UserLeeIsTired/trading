use std::{sync::atomic::Ordering};

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

impl <T> Drop for Sender<T> {
    fn drop(&mut self) {
        let core_queue = unsafe{ &*self.queue };
        let ref_count = core_queue.reference_count.fetch_sub(1, Ordering::Release);
        
        if ref_count == 1 {
            let raw_ptr = self.queue as *mut SPSC<T>;
            let _ = unsafe { Box::from_raw(raw_ptr) };
        }
    }
}