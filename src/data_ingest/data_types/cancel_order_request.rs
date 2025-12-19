use std::fmt;

#[repr(C, packed)]
pub struct CancelOrderRequest {
    pub message_type: u8,               // Offset 0, Length 1 (Value: 'X')
    pub user_ref_num: u32,              // Offset 1, Length 4 (Existing Order UserRefNum)
    pub quantity: u32,                  // Offset 5, Length 4 (New intended order size, 0 for full cancel)
    pub appendage_length: u16,          // Offset 9, Length 2 (Integer)
}


// For debug testing only

impl fmt::Debug for CancelOrderRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let byte_to_char = |byte: u8| -> char { byte as char };
        let msg_type_char = byte_to_char(self.message_type);
        let user_ref_num = self.user_ref_num;
        let quantity = self.quantity;
        let appendage_length = self.appendage_length;

        f.debug_struct("EnterOrderRequest")
            .field("type", &msg_type_char)
            .field("user_ref_num", &user_ref_num)
            .field("quantity", &quantity)
            .field("appendage_length", &appendage_length)
            .finish()
    }
}