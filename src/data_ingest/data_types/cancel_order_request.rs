#[repr(C, packed)]
pub struct CancelOrderRequest {
    pub message_type: u8,               // Offset 0, Length 1 (Value: 'X')
    pub user_ref_num: u32,              // Offset 1, Length 4 (Existing Order UserRefNum)
    pub quantity: u32,                  // Offset 5, Length 4 (New intended order size, 0 for full cancel)
    pub appendage_length: u16,          // Offset 9, Length 2 (Integer)
}
