#[repr(C, packed)]
pub struct DisableOrderEntryRequest {
    pub message_type: u8,               // Offset 0, Length 1 (Value: 'D')
    pub user_ref_num: u32,              // Offset 1, Length 4 (User Reference Number)
    pub firm: [u8; 4],                  // Offset 5, Length 4 (Alpha Firm Identifier)
    pub appendage_length: u16,          // Offset 9, Length 2 (Integer)
}
