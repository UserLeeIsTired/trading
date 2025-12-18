use std::collections::HashMap;

use super::node::Node;


// using a one time allocation method to create the slab, the arena (nodes) can be stored in the heap
// Since the allocation part is where the overhead occur, only changing data is much faster operation,
// and also allow better CPU cache compare to a linked list

pub struct Slab<T> {
    arena: Vec<T>,
    available_slot: Vec<usize>,
    hashmap: HashMap<u32, usize>
}

impl Slab<Node> {
    // create the arena and available slot with iteration method
    pub fn new(slab_size: usize) -> Self {
        Slab {
            arena: (0..slab_size).map(|_| { Node::new() }).collect(),
            available_slot: (0..slab_size).collect(),
            hashmap: HashMap::new(),
        }
    }

    // this function receive the last node, then update the last node and return the new last index
    pub fn append_list(&mut self, user_ref_num: u32, quantity: u32, node_ptr: Option<usize>) -> usize {

        // this always assume there are available slot, otherwise, God bless you
        let available_index = self.available_slot.pop().unwrap();
        
        if let Some(node_ptr) = node_ptr {
            // get the current node first
            let node = &mut self.arena[node_ptr];
            node.set_next(Some(available_index));
            
            // now get the new node to prevent rule violation
            let available_node = &mut self.arena[available_index];
            available_node.set_prev(Some(node_ptr));
            available_node.insert_detail(user_ref_num, quantity);

        }else {
            // now get the new node to prevent rule violation
            let available_node = &mut self.arena[available_index];
            available_node.insert_detail(user_ref_num, quantity);
        }


        // hashmap for O(1) look up if order is being modified or cancelled later
        self.hashmap.insert(user_ref_num, available_index);

        available_index
    }   


    // try to update prev, next node if they exist, and finally initalize the node
    pub fn unlink_node(&mut self, node_ptr: usize) {

        let user_ref_num = *(&self.arena[node_ptr].user_ref_num);
        
        // borrow immutable reference to take the ptr
        let (prev_ptr, next_ptr) = {
            let node = &self.arena[node_ptr];
            (node.get_prev(), node.get_next())
        };

        // if prev_node is Some, set next ptr
        if let Some(ptr) = prev_ptr {
            let prev_node = &mut self.arena[ptr];
            prev_node.set_next(next_ptr);
        }

        // if next_node is Some, set prev ptr
        if let Some(ptr) = next_ptr {
            let next_node = &mut self.arena[ptr];
            next_node.set_prev(prev_ptr);
        }
        
        // initialize
        let node = &mut self.arena[node_ptr];
        node.nullify_node();

        if let Some(user_ref_num) = user_ref_num {
            let _ = self.hashmap.remove(&user_ref_num);
        }
        // after the node is initialized, it is now available for reusing
        self.available_slot.push(node_ptr);

    }

    pub fn unlink_by_user_ref_num(&mut self, user_ref_num: u32) {
        
        let some_node_ptr = self.hashmap.get(&user_ref_num);

        if let Some(&node_ptr) = some_node_ptr {
            // borrow immutable reference to take the ptr
            let (prev_ptr, next_ptr) = {
                let node = &self.arena[node_ptr];
                (node.get_prev(), node.get_next())
            };

            // if prev_node is Some, set next ptr
            if let Some(ptr) = prev_ptr {
                let prev_node = &mut self.arena[ptr];
                prev_node.set_next(next_ptr);
            }

            // if next_node is Some, set prev ptr
            if let Some(ptr) = next_ptr {
                let next_node = &mut self.arena[ptr];
                next_node.set_prev(prev_ptr);
            }
            
            // initialize
            let node = &mut self.arena[node_ptr];
            node.nullify_node();

            // after the node is initialized, it is now available for reusing
            self.available_slot.push(node_ptr);

        }

    }

    pub fn get_mut_node(&mut self, node_ptr: usize) -> &mut Node {
        &mut self.arena[node_ptr]
    }
}