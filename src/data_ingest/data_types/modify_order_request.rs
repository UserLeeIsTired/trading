#[repr(C, packed)]
pub struct ModifyOrderRequest {
    pub message_type: u8,               // Offset 0, Length 1 (Value: 'M')
    pub user_ref_num: u32,              // Offset 1, Length 4 (Existing Order UserRefNum)
    pub side: u8,                       // Offset 5, Length 1 (New side: B, S, T, E)
    pub quantity: u32,                  // Offset 6, Length 4 (New intended order size)
    pub appendage_length: u16,          // Offset 10, Length 2 (Integer)
}