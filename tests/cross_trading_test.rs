use hft_trading::{
    data_ingest::{ProtocolRequest, Parser},
    low_latency_comm::{SPSC},
    price_matcher::PriceMatcher
};


// --- Constants (Required for the test to assert the outcome) ---

// The price index where the trades should occur (Assuming 950.00 maps to 95000)
const CROSS_PRICE_INDEX: usize = 95000; 

// The Price 950.00 represented as Little Endian bytes (as defined in your setup)
const PRICE_950_LE: [u8; 8] = [0x24, 0x89, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const SYMBOL_TSLA: [u8; 8] = [b'T', b'S', b'L', b'A', 0x20, 0x20, 0x20, 0x20]; 


// 1. BUY (300) | 2. SELL (200) | 3. SELL (500) | 4. BUY (700) | 5. SELL (300)
const MESSAGE_1: [u8; 47] = [b'O', 0x01, 0x00, 0x00, 0x00, b'B', 0x2C, 0x01, 0x00, 0x00, SYMBOL_TSLA[0], SYMBOL_TSLA[1], SYMBOL_TSLA[2], SYMBOL_TSLA[3], SYMBOL_TSLA[4], SYMBOL_TSLA[5], SYMBOL_TSLA[6], SYMBOL_TSLA[7], PRICE_950_LE[0], PRICE_950_LE[1], PRICE_950_LE[2], PRICE_950_LE[3], PRICE_950_LE[4], PRICE_950_LE[5], PRICE_950_LE[6], PRICE_950_LE[7], b'0', b'Y', b'A', b'Y', b'N', b'I', b'D', b'_', b'1', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00,];
const MESSAGE_2: [u8; 47] = [b'O', 0x02, 0x00, 0x00, 0x00, b'S', 0xC8, 0x00, 0x00, 0x00, SYMBOL_TSLA[0], SYMBOL_TSLA[1], SYMBOL_TSLA[2], SYMBOL_TSLA[3], SYMBOL_TSLA[4], SYMBOL_TSLA[5], SYMBOL_TSLA[6], SYMBOL_TSLA[7], PRICE_950_LE[0], PRICE_950_LE[1], PRICE_950_LE[2], PRICE_950_LE[3], PRICE_950_LE[4], PRICE_950_LE[5], PRICE_950_LE[6], PRICE_950_LE[7], b'3', b'N', b'P', b'N', b'N', b'I', b'D', b'_', b'2', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00,];
const MESSAGE_3: [u8; 47] = [b'O', 0x03, 0x00, 0x00, 0x00, b'S', 0xF4, 0x01, 0x00, 0x00, SYMBOL_TSLA[0], SYMBOL_TSLA[1], SYMBOL_TSLA[2], SYMBOL_TSLA[3], SYMBOL_TSLA[4], SYMBOL_TSLA[5], SYMBOL_TSLA[6], SYMBOL_TSLA[7], PRICE_950_LE[0], PRICE_950_LE[1], PRICE_950_LE[2], PRICE_950_LE[3], PRICE_950_LE[4], PRICE_950_LE[5], PRICE_950_LE[6], PRICE_950_LE[7], b'0', b'A', b'R', b'N', b'C', b'I', b'D', b'_', b'3', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00,];
const MESSAGE_4: [u8; 47] = [b'O', 0x04, 0x00, 0x00, 0x00, b'B', 0xBC, 0x02, 0x00, 0x00, SYMBOL_TSLA[0], SYMBOL_TSLA[1], SYMBOL_TSLA[2], SYMBOL_TSLA[3], SYMBOL_TSLA[4], SYMBOL_TSLA[5], SYMBOL_TSLA[6], SYMBOL_TSLA[7], PRICE_950_LE[0], PRICE_950_LE[1], PRICE_950_LE[2], PRICE_950_LE[3], PRICE_950_LE[4], PRICE_950_LE[5], PRICE_950_LE[6], PRICE_950_LE[7], b'E', b'Y', b'O', b'N', b'N', b'I', b'D', b'_', b'4', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00,];
const MESSAGE_5: [u8; 47] = [b'O', 0x05, 0x00, 0x00, 0x00, b'S', 0x2C, 0x01, 0x00, 0x00, SYMBOL_TSLA[0], SYMBOL_TSLA[1], SYMBOL_TSLA[2], SYMBOL_TSLA[3], SYMBOL_TSLA[4], SYMBOL_TSLA[5], SYMBOL_TSLA[6], SYMBOL_TSLA[7], PRICE_950_LE[0], PRICE_950_LE[1], PRICE_950_LE[2], PRICE_950_LE[3], PRICE_950_LE[4], PRICE_950_LE[5], PRICE_950_LE[6], PRICE_950_LE[7], b'0', b'Y', b'A', b'Y', b'N', b'I', b'D', b'_', b'1', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00,];

fn get_messages() -> Vec<&'static [u8]>{
    vec![&MESSAGE_1, &MESSAGE_2, &MESSAGE_3, &MESSAGE_4, &MESSAGE_5]
}


// --- THE TEST BLOCK ---

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test_cross_order_clearance_spsc() {
        // --- Setup ---
        let (done_tx, done_rx) = mpsc::channel();
        let queue = SPSC::<ProtocolRequest>::new();
        let (tx, rx) = queue.split();
        let messages_data = get_messages();
        
        // --- Producer Thread (Parses messages and sends them to the queue) ---
        let producer_thread = thread::spawn(move || {
            let parser = Parser::new(tx);
            for &message in messages_data.iter() {
                parser.parse(message); 
            }
            // Sender drops here, signaling stream completion.
        });

        // --- Consumer Thread (Runs the matching engine logic) ---
        let consumer_thread = thread::spawn(move || {
            let mut price_matcher = PriceMatcher::new();

            loop {
                match rx.recv() {
                    Ok(item) => {
                        if let ProtocolRequest::EnterOrder(order) = item {
                            // Convert price to the index (u64 -> usize)
                            let price_index: usize = order.price.try_into().unwrap_or(0); 

                            if order.side == b'B' {
                                price_matcher.add_bid_order(order.user_ref_num, order.quantity, price_index);
                            } else {
                                price_matcher.add_ask_order(order.user_ref_num, order.quantity, price_index);
                            }
                            price_matcher.process_order(); 
                        }
                    },
                    Err(_) => {
                        println!("Exit");
                        break
                    }, // Exit when SPSC channel is closed and empty
                }
            }
            
            // Return the final state of the matcher for assertion
            done_tx.send(price_matcher).expect("Failed to send final matcher state.");
        });

        // --- Assertion ---
        producer_thread.join().unwrap();
        let final_matcher = done_rx.recv().expect("Failed to retrieve final matcher state.");
        
        // Total Buy (1000) matched Total Sell (1000) at CROSS_PRICE_INDEX.
        
        // 1. Assert the market is cleared (BBO spread exists)
        assert!(
            final_matcher.max_bid < final_matcher.min_ask,
            "Market was not cleared! Max Bid: {}, Min Ask: {}",
            final_matcher.max_bid, final_matcher.min_ask
        );

        // 2. Assert the cross price level is entirely empty
        assert!(
            final_matcher.bids[CROSS_PRICE_INDEX].0.is_none(),
            "Bid side at cross price {} should be empty after trade.",
            CROSS_PRICE_INDEX
        );
        
        assert!(
            final_matcher.asks[CROSS_PRICE_INDEX].0.is_none(),
            "Ask side at cross price {} should be empty after trade.",
            CROSS_PRICE_INDEX
        );

        println!("Test Passed: Full 1000 share cross-trade successfully executed and book cleared at index {}.", CROSS_PRICE_INDEX);
    }
}