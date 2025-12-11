use crate::data_structure::{Node, Slab};
use crate::low_latency_comm::Receiver;
use crate::data_ingest::ProtocolRequest;

// Let assume the all stock price is in the range [0, 2500.00] due to device limitation

const ORDER_BOOK_CAPACITY: usize = 250000; // 2500 * 100 due to the floating point
const SLAB_SIZE: usize = 10_000_000;


// slab, where the orders are holded, inside bid ask, there are only 2 fields, representing the
// index of (head, tail)

pub struct PriceMatcher {
    slab: Slab<Node>,
    bids: Vec<(Option<usize>, Option<usize>)>,
    asks: Vec<(Option<usize>, Option<usize>)>,
    max_bid: usize,
    min_ask: usize
}

impl PriceMatcher {
    pub fn new() -> Self {      
        PriceMatcher {
            slab: Slab::new(SLAB_SIZE),
            bids: vec![(None, None); ORDER_BOOK_CAPACITY + 1],
            asks: vec![(None, None); ORDER_BOOK_CAPACITY + 1],
            max_bid: 0,
            min_ask: ORDER_BOOK_CAPACITY,
        }
    }

    pub fn add_bid_order(&mut self, price: usize, user_ref_num: u32, quantity: u32) {
        if price > self.max_bid {
            self.max_bid = price;
        }
        
        let new_tail = self.slab.append_list(user_ref_num, quantity, self.bids[price].1);

        if self.bids[price].0.is_none() {
            self.bids[price].0 = Some(new_tail);
        }

        self.bids[price].1 = Some(new_tail);

    }

    pub fn add_ask_order(&mut self, price: usize, user_ref_num: u32, quantity: u32) {
        if price < self.min_ask {
            self.min_ask = price;
        }
        
        let new_tail = self.slab.append_list(user_ref_num, quantity, self.asks[price].1);

        if self.asks[price].0.is_none() {
            self.asks[price].0 = Some(new_tail);
        }

        self.asks[price].1 = Some(new_tail);

    }

    fn process_order() {
        
    }
}






