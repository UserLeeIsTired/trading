const MESSAGE_QUEUE_SIZE: usize = 10000;


pub struct BroadCast {
    // T: (user_ref_num, quantity, price, request_type)
    message_queue: Vec<(u32, u32, u32, char)>
}

impl BroadCast {
    
    pub fn new() -> Self {
        BroadCast {
            message_queue: Vec::with_capacity(MESSAGE_QUEUE_SIZE),
        }
    }

    // append item 
    pub fn add_successful_order(&mut self, 
        user_ref_num: u32, 
        quantity: u32, 
        price: u32, 
        request_type: char
    ) {
        self.message_queue.push((user_ref_num, quantity, price, request_type));
    }  

    // system out info
    pub fn print_info(&mut self) {
        let message = self.message_queue.pop();
        
        if let Some(message) = message {
            let action = if message.3 == 'S' {"brought"} else {"sold"};
            println!("{} successfully {} {} @ price {}", message.0, action, message.1, message.2);
        } 
         
    }

    // to be implemented
    pub fn send_confirmation(&self) {
        unimplemented!();
    }

}