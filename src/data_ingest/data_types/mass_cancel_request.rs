#[repr(C, packed)]
pub struct MassCancelRequest {
    pub message_type: u8,               // Offset 0, Length 1 (Value: 'C')
    pub user_ref_num: u32,              // Offset 1, Length 4 (User Reference Number)
    pub firm: [u8; 4],                  // Offset 5, Length 4 (Alpha Firm Identifier)
    pub symbol: [u8; 8],                // Offset 9, Length 8 (Alpha, Space filled if not specified)
    pub appendage_length: u16,          // Offset 17, Length 2 (Integer)
}