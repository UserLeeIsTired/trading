use std::ptr;

// The way the Node is implemented will copy the user_ref_num and the quantity (16 bytes),
// the copy size is small and therefore acceptable compare to zero-copy method.

pub struct Node {
    pub user_ref_num: u32,
    pub quantity: u32,
    pub prev_node: *mut Node,
    pub next_node: *mut Node,
}

impl Node {
    pub fn new(user_ref_num: u32, quantity: u32) -> Self {
        Node {
            user_ref_num,
            quantity,
            prev_node: ptr::null_mut(),
            next_node: ptr::null_mut(),
        }
    }

    // clean up the node like manually
    pub fn nullify_links(&mut self) {
        self.prev_node = ptr::null_mut();
        self.next_node = ptr::null_mut();
    }
}

pub struct LinkedList {
    pub size: usize,
    pub first_node: *mut Node,
    pub last_node: *mut Node,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            size: 0,
            first_node: ptr::null_mut(),
            last_node: ptr::null_mut(),
        }
    }

    // Let the LinkedList itself to remove the node, while it receive a node pointer as input
    // the node should be stored in a hashmap[(transaction_id, user_ref_num)] -> *mut Node

    pub fn remove_node_ptr(&mut self, node_ptr: *mut Node) {
        
        // handle invalid input for safety reason

        if node_ptr.is_null() {
            return;
        }

        
        let node_ref = unsafe { &mut *node_ptr };
        let prev_ptr = node_ref.prev_node;
        let next_ptr = node_ref.next_node;
        
        // when the current deleting node is the first node
        
        if node_ptr == self.first_node {
            self.first_node = next_ptr;
        }
        
        // when the current deleting node is the last node

        if node_ptr == self.last_node {
            self.last_node = prev_ptr;
        }

        if !prev_ptr.is_null() {
            unsafe { (*prev_ptr).next_node = next_ptr };
        }

        if !next_ptr.is_null() {
            unsafe { (*next_ptr).prev_node = prev_ptr };
        }

        // remove the current node and reduce the size of the list

        node_ref.nullify_links();

        self.size -= 1;
        
    }
    
    pub fn push_back(&mut self, new_node_ptr: *mut Node) {
        // when the list just got created
        
        if self.last_node.is_null() {
            self.first_node = new_node_ptr;
            self.last_node = new_node_ptr;
        } else {

            // take the node pointer and append

            unsafe {
                (*self.last_node).next_node = new_node_ptr;
                (*new_node_ptr).prev_node = self.last_node;
            }
            self.last_node = new_node_ptr;
        }
        self.size += 1;
    }
}