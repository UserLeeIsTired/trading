/*   Only simple cases are considered in the code, the optional appendage is ignored    */

#[repr(C, packed)]
pub struct EnterOrderRequest {
    pub message_type: u8,               // Offset 0, Length 1 (Value: 'O')
    pub user_ref_num: u32,              // Offset 1, Length 4 (UserRefNuma)
    pub side: u8,                       // Offset 5, Length 1 (Alpha: B, S, T, E)
    pub quantity: u32,                  // Offset 6, Length 4 (Integer)
    pub symbol: [u8; 8],                // Offset 10, Length 8 (Alpha)
    pub price: u64,                     // Offset 18, Length 8 (Price)
    pub time_in_force: u8,              // Offset 26, Length 1 (Alpha)
    pub display: u8,                    // Offset 27, Length 1 (Alpha: Y, N, A)
    pub capacity: u8,                   // Offset 28, Length 1 (Alpha: A, P, R, O)
    pub inter_market_sweep_eligibility: u8, // Offset 29, Length 1 (Alpha: Y, N)
    pub cross_type: u8,                 // Offset 30, Length 1 (Alpha: N, O, C, H, S, R, E, A)
    pub cl_ord_id: [u8; 14],            // Offset 31, Length 14 (Alpha)
    pub appendage_length: u16,          // Offset 45, Length 2 (Integer)
}