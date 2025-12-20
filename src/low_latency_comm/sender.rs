use std::{sync::atomic::Ordering};

use super::spsc::SPSC;
use super::error::{SenderError};

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

    pub fn try_send(&self, item: T) -> Result<(), SenderError> {
        let core_queue = unsafe {
            &(*self.queue)
        };
        let result = core_queue.try_push(item);

        match result {
            Ok(_) => Ok(()),
            Err(_) => unsafe {
                if (&*self.queue).check_other_component_disconnected() {
                    return Err(SenderError::ReceiverDisconnected);
                }else {
                    return Err(SenderError::Full);
                }
            },
        }
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