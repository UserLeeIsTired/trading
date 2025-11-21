use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

const BUFFER_CAPACITY: usize = 4096;

// TODO: Add comment
pub struct SPSC<T> {
    buffer: [UnsafeCell<MaybeUninit<T>>; BUFFER_CAPACITY],
    head_index: AtomicUsize,
    tail_index: AtomicUsize,
}

unsafe impl <T: Send> Send for SPSC<T> {}
unsafe impl <T: Send> Sync for SPSC<T> {}

impl <T> SPSC<T> {
    pub fn new() -> Self {
        let buffer: [UnsafeCell<MaybeUninit<T>>; BUFFER_CAPACITY] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        SPSC { buffer, head_index: AtomicUsize::new(0), tail_index: AtomicUsize::new(0) }
    }
    
    pub fn try_push(&self, item: T) -> Result<(), ()> {
        let tail = self.tail_index.load(Ordering::Relaxed);
        let next_index = (tail + 1) % BUFFER_CAPACITY;    

        if next_index == self.head_index.load(Ordering::Acquire) {
            return Err(());
        }

        unsafe {
            let that_buffer = self.buffer[next_index].get();
            (*that_buffer).write(item);
        }

        self.tail_index.store(next_index, Ordering::Release);

        Ok(())
    }

    pub fn try_pop(&self) -> Result<T, ()> {
        let head = self.head_index.load(Ordering::Relaxed);
        let next_index = (head + 1) % BUFFER_CAPACITY;

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
}