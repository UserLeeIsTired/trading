// The way the Node is implemented will copy the user_ref_num and the quantity (16 bytes),
// the copy size is small and therefore acceptable compare to zero-copy method.

// For now, the option struct is an overhead that can be further optimized, but for the clarity
// of the code, I will keep the Option<T>

pub struct Node {
    pub user_ref_num: Option<u32>,
    pub quantity: Option<u32>,
    pub price: Option<usize>,
    pub side: Option<char>,
    pub prev_node: Option<usize>,
    pub next_node: Option<usize>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            user_ref_num: None,
            quantity: None,
            price: None,
            side: None,
            prev_node: None,
            next_node: None,
        }
    }

    // clean up the node manually
    pub fn nullify_node(&mut self) {
        self.user_ref_num = None;
        self.quantity = None;
        self.price = None;
        self.side = None;
        self.prev_node = None;
        self.next_node = None;
    }

    // Below are just the getter and setter

    pub fn get_prev(&self) -> Option<usize> {
        self.prev_node
    }

    pub fn get_next(&self) -> Option<usize> {
        self.next_node
    }

    pub fn get_user_ref_num(&self) -> Option<u32> {
        self.user_ref_num
    }

    pub fn get_quantity(&self) -> Option<u32> {
        self.quantity
    }

    pub fn get_price(&self) -> Option<usize> {
        self.price
    }

    pub fn get_side(&self) -> Option<char> {
        self.side
    }

    pub fn set_prev(&mut self, node_ptr: Option<usize>) {
        self.prev_node = node_ptr;
    }

    pub fn set_next(&mut self, node_ptr: Option<usize>) {
        self.next_node = node_ptr;
    }

    pub fn set_user_ref_num(&mut self, user_ref_num: u32) {
        self.user_ref_num = Some(user_ref_num);
    }

    pub fn set_quantity(&mut self, quantity: u32){
        self.quantity = Some(quantity);
    }

    pub fn set_price(&mut self, price: usize) {
        self.price = Some(price);
    }

    pub fn set_side(&mut self, side: char) {
        self.side = Some(side);
    }
}