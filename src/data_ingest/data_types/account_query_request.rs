#[repr(C, packed)]
pub struct AccountQueryRequest {
    pub message_type: u8,               // Offset 0, Length 1 (Value: 'Q')
    pub appendage_length: u16,          // Offset 1, Length 2 (Integer)
}