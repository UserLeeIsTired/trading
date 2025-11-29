use std::fmt;

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

// For debug testing only

impl fmt::Debug for EnterOrderRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format_alpha = |bytes: &[u8]| -> String {
            let len = bytes.iter().position(|&b| b == 0x20 || b == 0x00).unwrap_or(bytes.len());
            String::from_utf8_lossy(&bytes[..len]).into_owned()
        };

        let byte_to_char = |byte: u8| -> char { byte as char };
        let msg_type_char = byte_to_char(self.message_type);
        let user_ref_num = self.user_ref_num;
        let side_char = byte_to_char(self.side);
        let quantity = self.quantity;
        let symbol_string = format_alpha(&self.symbol);
        let price = self.price;
        let tif_char = byte_to_char(self.time_in_force);
        let display_char = byte_to_char(self.display);
        let capacity_char = byte_to_char(self.capacity);
        let ime_eligibility_char = byte_to_char(self.inter_market_sweep_eligibility);
        let cross_type_char = byte_to_char(self.cross_type);
        let cl_ord_id_string = format_alpha(&self.cl_ord_id);
        let appendage_length = self.appendage_length;

        f.debug_struct("EnterOrderRequest")
            .field("type", &msg_type_char)
            .field("user_ref_num", &user_ref_num)
            .field("side", &side_char)
            .field("quantity", &quantity)
            .field("symbol", &symbol_string)
            .field("price", &price)
            .field("time_in_force", &tif_char)
            .field("display", &display_char)
            .field("capacity", &capacity_char)
            .field("ime_eligibility", &ime_eligibility_char)
            .field("cross_type", &cross_type_char)
            .field("cl_ord_id", &cl_ord_id_string)
            .field("appendage_length", &appendage_length)
            .finish()
    }
}