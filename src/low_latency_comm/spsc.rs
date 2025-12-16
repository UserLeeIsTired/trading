use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

use super::{receiver::Receiver, sender::Sender};

const BUFFER_CAPACITY: usize = 16384;

// The spsc queue is a thread safety, only one consumer and one producer

pub struct SPSC<T> {
    // head and tail index is like a critical section, due to data racing, we must use atomic
    buffer: [UnsafeCell<MaybeUninit<T>>; BUFFER_CAPACITY],
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

        SPSC { buffer, head_index: AtomicUsize::new(0), tail_index: AtomicUsize::new(0) }
    }
    
    // pushing item into the buffer
    pub fn try_push(&self, item: T) -> Result<(), ()> {
        
        // get the current index, store item, update index to index += 1

        let tail = self.tail_index.load(Ordering::Relaxed);
        let next_index = (tail + 1) & (BUFFER_CAPACITY - 1);    

        if next_index == self.head_index.load(Ordering::Acquire) {
            return Err(());
        }

        unsafe {
            let that_buffer = self.buffer[tail].get();
            (*that_buffer).write(item);
        }

        self.tail_index.store(next_index, Ordering::Release);

        Ok(())
    }

    // pop item from the buffer

    pub fn try_pop(&self) -> Result<T, ()> {

        // get the current index, get item, update index to index += 1

        let head = self.head_index.load(Ordering::Relaxed);
        let next_index = (head + 1) & (BUFFER_CAPACITY - 1);

        if head == self.tail_index.load(Ordering::Acquire) {
            return Err(());
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
}