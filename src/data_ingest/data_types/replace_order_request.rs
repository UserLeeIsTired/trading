#[repr(C, packed)]
pub struct ReplaceOrderRequest {
    pub message_type: u8,               // Offset 0, Length 1 (Value: 'U')
    pub orig_user_ref_num: u32,         // Offset 1, Length 4 (Existing Order UserRefNum)
    pub new_user_ref_num: u32,          // Offset 5, Length 4 (Replacement Order UserRefNum)
    pub quantity: u32,                  // Offset 9, Length 4 (Integer)
    pub price: u64,                     // Offset 13, Length 8 (Price)
    pub time_in_force: u8,              // Offset 21, Length 1 (Alpha)
    pub display: u8,                    // Offset 22, Length 1 (Alpha: Y, N, A)
    pub inter_market_sweep_eligibility: u8, // Offset 23, Length 1 (Alpha: Y, N)
    pub cl_ord_id: [u8; 14],            // Offset 24, Length 14 (Alpha)
    pub appendage_length: u16,          // Offset 38, Length 2 (Integer)
}