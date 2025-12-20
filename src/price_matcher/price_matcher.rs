// To maintain readability of the code, most of the abstraction will be retained
// e.g. the getter and setter function
// In real world application, such abstraction will introduce overhead and
// should be eliminated to provide even better performance.

use crate::data_structure::{Node, Slab};

// Let assume the all stock price is in the range [0, 2500.00] due to device limitation

const ORDER_BOOK_CAPACITY: usize = 250000; // 2500 * 100 due to the floating point
const SLAB_SIZE: usize = 10_000_000;

// slab, where the orders are holded, inside bid ask, there are only 2 fields, representing the
// index of (head, tail)
pub struct PriceMatcher {
    pub slab: Slab<Node>,
    pub bids: Vec<(Option<usize>, Option<usize>)>,
    pub asks: Vec<(Option<usize>, Option<usize>)>,
    pub max_bid: usize,
    pub min_ask: usize
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

    pub fn add_bid_order(
        &mut self, 
        user_ref_num: u32, 
        quantity: u32,
        price: usize, 
    ) {
        if price > self.max_bid {
            self.max_bid = price;
        }
        
        let new_tail = self.slab.append_list(user_ref_num, quantity, price, 'B', self.bids[price].1);

        if self.bids[price].0.is_none() {
            self.bids[price].0 = Some(new_tail);
        }

        self.bids[price].1 = Some(new_tail);

    }

    pub fn add_ask_order(
        &mut self, 
        user_ref_num: u32, 
        quantity: u32,
        price: usize, 
    ) {
        if price < self.min_ask {
            self.min_ask = price;
        }
        
        let new_tail = self.slab.append_list(user_ref_num, quantity, price, 'S', self.asks[price].1);

        if self.asks[price].0.is_none() {
            self.asks[price].0 = Some(new_tail);
        }

        self.asks[price].1 = Some(new_tail);

    }

    pub fn cancel_order(
        &mut self, 
        user_ref_num: u32
    ) {

        let target_node = self.slab.get_node_by_user_ref_num(user_ref_num);

        let meta = target_node
            .map(|(_, n)| (n.get_price().unwrap(), n.get_side().unwrap(), n.get_prev(), n.get_next()));
        
        if let Some((price, side, prev_ptr, next_ptr)) = meta {
            let (idx, node)= target_node.unwrap();
            
            let book =  if side == 'B' { &mut self.bids } else { &mut self.asks };
            let (ref mut head, ref mut tail) = book[price];

            if let Some(head_idx) = *head {
                if std::ptr::eq(self.slab.get_node(head_idx), node) {
                    *head = next_ptr;
                }
            }

            if let Some(tail_idx) = *tail {
                if std::ptr::eq(self.slab.get_node(tail_idx), node) {
                    *tail = prev_ptr;
                }
            }
            
            self.slab.unlink_node(idx);
        }
    }

    pub fn update_order(
        &mut self,
        user_ref_num: u32,
        new_quantity: u32,
        new_price: usize,
        new_side: char,
    ) {
        let node = self.slab.get_mut_node_by_user_ref_num(user_ref_num);
        
        if let Some((_, node)) = node {
            let (old_quantity, 
                old_price, 
                old_side) = 
                (node.get_quantity(), 
                node.get_price(), 
                node.get_side()
            );

            if old_quantity.unwrap() < new_quantity 
            || old_price.unwrap() != new_price 
            || old_side.unwrap() != new_side {
                self.cancel_order(user_ref_num);
                match new_side {
                    'B' => self.add_bid_order(user_ref_num, new_quantity, new_price),
                    'S' => self.add_ask_order(user_ref_num, new_quantity, new_price),
                    _ => ()
                }
            }else {
                node.set_quantity(new_quantity);
            }
        }
    }

    fn consume_node(
        &mut self, 
        bid_index: Option<usize>, 
        ask_index: Option<usize>
    ) -> (Option<usize>, Option<usize>) {
    
        let (mut next_bid_index, mut next_ask_index) = (bid_index, ask_index);

        // user_ref is used to send confirmation but ignored for now
        let (_bid_user_ref, bid_quantity) = {
            let node = self.slab.get_mut_node(bid_index.unwrap());
            (node.get_user_ref_num(), node.get_quantity())
        };

        let (_ask_user_ref, ask_quantity) = {
            let node = self.slab.get_mut_node(ask_index.unwrap());
            (node.get_user_ref_num(), node.get_quantity())
        };

        if bid_quantity.unwrap() > ask_quantity.unwrap() {
        
            next_ask_index = self.slab
            .get_mut_node(ask_index.unwrap())
            .get_next();

            self.slab.get_mut_node(bid_index.unwrap())
            .set_quantity(
            bid_quantity.unwrap() - ask_quantity.unwrap()
            );

            self.slab.unlink_node(ask_index.unwrap());
            
            // temp
            // println!("{} successfully brought {} stocks", _bid_user_ref.unwrap(), ask_quantity.unwrap());
            // println!("{} successfully sold {} stocks", _ask_user_ref.unwrap(), ask_quantity.unwrap());

        } else if bid_quantity.unwrap() < ask_quantity.unwrap() {

            next_bid_index = self.slab
            .get_mut_node(bid_index.unwrap())
            .get_next();

            self.slab.get_mut_node(ask_index.unwrap())
            .set_quantity(
            ask_quantity.unwrap() - bid_quantity.unwrap()
            );

            self.slab.unlink_node(bid_index.unwrap());

            // temp
            // println!("{} successfully brought {} stocks", _bid_user_ref.unwrap(), bid_quantity.unwrap());
            // println!("{} successfully sold {} stocks", _ask_user_ref.unwrap(), bid_quantity.unwrap());

        } else if bid_quantity.unwrap() == ask_quantity.unwrap() {
            next_bid_index = self.slab
            .get_mut_node(bid_index.unwrap())
            .get_next();
            
            next_ask_index = self.slab
            .get_mut_node(ask_index.unwrap())
            .get_next();

            self.slab.unlink_node(bid_index.unwrap());
            self.slab.unlink_node(ask_index.unwrap());
            
            // temp
            // println!("{} successfully brought {} stocks", _bid_user_ref.unwrap(), ask_quantity.unwrap());
            // println!("{} successfully sold {} stocks", _ask_user_ref.unwrap(), ask_quantity.unwrap());
        }

        // TODO: Trading confirmation

        (next_bid_index, next_ask_index)
    }

    pub fn process_order(&mut self) {
        while self.max_bid >= self.min_ask {
            
            while self.max_bid > 0 && self.max_bid >= self.min_ask && self.bids[self.max_bid].0.is_none() {
                self.max_bid -= 1;
            }

            while self.min_ask < ORDER_BOOK_CAPACITY && self.min_ask <= self.max_bid && self.asks[self.min_ask].0.is_none() {
                self.min_ask += 1;
            }

            if self.max_bid < self.min_ask {
                break;
            }

            let (next_bid_node
                , next_ask_node) 
            = self.consume_node(self.bids[self.max_bid].0, self.asks[self.min_ask].0);

            if next_bid_node.is_none() {
                self.bids[self.max_bid].1 = None;
            }

            if next_ask_node.is_none() {
                self.asks[self.min_ask].1 = None;
            }

            self.bids[self.max_bid].0 = next_bid_node;
            self.asks[self.min_ask].0 = next_ask_node;


        }
    }
}






