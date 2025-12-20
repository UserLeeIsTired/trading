use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

use super::{receiver::Receiver, sender::Sender};
use super::error::SPSCError;

const BUFFER_CAPACITY: usize = 16384;
const MASK: usize = BUFFER_CAPACITY - 1;

// The spsc queue is a thread safety, only one consumer and one producer

pub struct SPSC<T> {
    // head and tail index is like a critical section, due to data racing, we must use atomic
    buffer: [UnsafeCell<MaybeUninit<T>>; BUFFER_CAPACITY],
    // added a reference count, when both consumer and producer are dropped, the queue should also be dropped
    pub reference_count: AtomicUsize,
    head_index: AtomicUsize,
    tail_index: AtomicUsize,
}

// Since the queue may move between thread, it needs to have Send and Sync
unsafe impl <T: Send> Send for SPSC<T> {}
unsafe impl <T: Send> Sync for SPSC<T> {}

impl <T> SPSC<T> {
    pub fn new() -> Self {
        // The rust compile cannot guarantee generic T array is initialized 
        // MaybeUninit should be used to initialize the buffer array
        let buffer: [UnsafeCell<MaybeUninit<T>>; BUFFER_CAPACITY] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        SPSC { buffer, reference_count: AtomicUsize::new(2), head_index: AtomicUsize::new(0), tail_index: AtomicUsize::new(0) }
    }
    
    // pushing item into the buffer
    pub fn try_push(&self, item: T) -> Result<(), SPSCError> {
        
        // get the current index, store item, update index to index += 1

        let tail = self.tail_index.load(Ordering::Relaxed);
        let next_index = (tail + 1) & MASK;    

        if next_index == self.head_index.load(Ordering::Acquire) {
            return Err(SPSCError::Full);
        }

        unsafe {
            let that_buffer = self.buffer[tail].get();
            (*that_buffer).write(item);
        }

        self.tail_index.store(next_index, Ordering::Release);

        Ok(())
    }

    // pop item from the buffer

    pub fn try_pop(&self) -> Result<T, SPSCError> {

        // get the current index, get item, update index to index += 1

        let head = self.head_index.load(Ordering::Relaxed);
        let next_index = (head + 1) & MASK;

        if head == self.tail_index.load(Ordering::Acquire) {
            return Err(SPSCError::Empty);
        }

        unsafe {
            let that_buffer = self.buffer[head].get();
            let item = (*that_buffer).assume_init_read();
            self.head_index.store(next_index, Ordering::Release);
            Ok(item)
        }
    }


    // split the SPSC queue into sender and receiver

    pub fn split(self) -> (Sender<T>, Receiver<T>) {
        // heap allocation here is acceptable since it is a one time set-up
        let core_queue = Box::into_raw(Box::new(self));
        let sender = Sender::new(core_queue);
        let receiver = Receiver::new(core_queue);

        (sender, receiver)
    }


    // return true if one of them is disconnected, otherwise false
    pub fn check_other_component_disconnected(&self) -> bool {
        let connected_component = self.reference_count.load(Ordering::Relaxed);
        connected_component < 2
    }
}