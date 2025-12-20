use std::{sync::atomic::Ordering};

use super::spsc::{SPSC};
use super::error::{ReceiverError};

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

    pub fn try_recv(&self) -> Result<T, ReceiverError> {
        let core_queue = unsafe {
            &(*self.queue)
        };

        let result = core_queue.try_pop();

        match result {
            Ok(result) => Ok(result),
            Err(_) => {
                unsafe {
                    if (&*self.queue).check_other_component_disconnected() {
                        return Err(ReceiverError::SenderDisconnected);
                    } else {
                        return Err(ReceiverError::Empty);
                    }
                }
            } 
        }
    }
}

impl <T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let core_queue = unsafe{ &*self.queue };
        let ref_count = core_queue.reference_count.fetch_sub(1, Ordering::Release);
        
        if ref_count == 1 {
            let raw_ptr = self.queue as *mut SPSC<T>;
            let _ = unsafe { Box::from_raw(raw_ptr) };
        }
    }
}