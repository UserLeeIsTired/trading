use hft_trading::low_latency_comm::{SPSC};
use hft_trading::data_ingest::{Parser};
use hft_trading::data_ingest::ProtocolRequest;
use hft_trading::price_matcher::{PriceMatcher};


// --- Constants for Order Protocol Message Generation ---

const PRICE_950_LE: [u8; 8] = [0x24, 0x89, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
const SYMBOL_TSLA: [u8; 8] = [b'T', b'S', b'L', b'A', 0x20, 0x20, 0x20, 0x20]; // "TSLA    "


// 1. BUY Order (300 Shares) - Total Buy: 300
// UserRefNum: 1
const MESSAGE_1: [u8; 47] = [
    // Type: 'O'
    b'O',
    // UserRefNum: 1u32
    0x01, 0x00, 0x00, 0x00,
    // Side: 'B' (Buy)
    b'B',
    // Quantity: 300u32 (Little Endian: 2C 01 00 00)
    0x2C, 0x01, 0x00, 0x00, 
    // Symbol: "TSLA    "
    SYMBOL_TSLA[0], SYMBOL_TSLA[1], SYMBOL_TSLA[2], SYMBOL_TSLA[3], SYMBOL_TSLA[4], SYMBOL_TSLA[5], SYMBOL_TSLA[6], SYMBOL_TSLA[7],
    // Price: 950.00
    PRICE_950_LE[0], PRICE_950_LE[1], PRICE_950_LE[2], PRICE_950_LE[3], PRICE_950_LE[4], PRICE_950_LE[5], PRICE_950_LE[6], PRICE_950_LE[7],
    // TIF, Display, Capacity, etc. (Original values retained)
    b'0', b'Y', b'A', b'Y', b'N', 
    b'I', b'D', b'_', b'1', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x00, 0x00,
];

// 2. SELL Order (200 Shares) - Total Sell: 200
// UserRefNum: 2
const MESSAGE_2: [u8; 47] = [
    // Type: 'O'
    b'O',
    // UserRefNum: 2u32
    0x02, 0x00, 0x00, 0x00,
    // Side: 'S' (Sell) - Changed from 'E'
    b'S',
    // Quantity: 200u32 (Little Endian: C8 00 00 00)
    0xC8, 0x00, 0x00, 0x00, 
    // Symbol: "TSLA    "
    SYMBOL_TSLA[0], SYMBOL_TSLA[1], SYMBOL_TSLA[2], SYMBOL_TSLA[3], SYMBOL_TSLA[4], SYMBOL_TSLA[5], SYMBOL_TSLA[6], SYMBOL_TSLA[7],
    // Price: 950.00
    PRICE_950_LE[0], PRICE_950_LE[1], PRICE_950_LE[2], PRICE_950_LE[3], PRICE_950_LE[4], PRICE_950_LE[5], PRICE_950_LE[6], PRICE_950_LE[7],
    // TIF, Display, Capacity, etc.
    b'3', b'N', b'P', b'N', b'N', 
    b'I', b'D', b'_', b'2', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x00, 0x00,
];

// 3. SELL Order (500 Shares) - Total Sell: 700
// UserRefNum: 3
const MESSAGE_3: [u8; 47] = [
    // Type: 'O'
    b'O',
    // UserRefNum: 3u32
    0x03, 0x00, 0x00, 0x00,
    // Side: 'S' (Sell)
    b'S',
    // Quantity: 500u32 (Little Endian: F4 01 00 00) - Changed from 1000
    0xF4, 0x01, 0x00, 0x00, 
    // Symbol: "TSLA    "
    SYMBOL_TSLA[0], SYMBOL_TSLA[1], SYMBOL_TSLA[2], SYMBOL_TSLA[3], SYMBOL_TSLA[4], SYMBOL_TSLA[5], SYMBOL_TSLA[6], SYMBOL_TSLA[7],
    // Price: 950.00
    PRICE_950_LE[0], PRICE_950_LE[1], PRICE_950_LE[2], PRICE_950_LE[3], PRICE_950_LE[4], PRICE_950_LE[5], PRICE_950_LE[6], PRICE_950_LE[7],
    // TIF, Display, Capacity, etc.
    b'0', b'A', b'R', b'N', b'C', 
    b'I', b'D', b'_', b'3', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x00, 0x00,
];

// 4. BUY Order (700 Shares) - Total Buy: 1000
// UserRefNum: 4
const MESSAGE_4: [u8; 47] = [
    // Type: 'O'
    b'O',
    // UserRefNum: 4u32
    0x04, 0x00, 0x00, 0x00,
    // Side: 'B' (Buy)
    b'B',
    // Quantity: 700u32 (Little Endian: BC 02 00 00)
    0xBC, 0x02, 0x00, 0x00, 
    // Symbol: "TSLA    "
    SYMBOL_TSLA[0], SYMBOL_TSLA[1], SYMBOL_TSLA[2], SYMBOL_TSLA[3], SYMBOL_TSLA[4], SYMBOL_TSLA[5], SYMBOL_TSLA[6], SYMBOL_TSLA[7],
    // Price: 950.00
    PRICE_950_LE[0], PRICE_950_LE[1], PRICE_950_LE[2], PRICE_950_LE[3], PRICE_950_LE[4], PRICE_950_LE[5], PRICE_950_LE[6], PRICE_950_LE[7],
    // TIF, Display, Capacity, etc.
    b'E', b'Y', b'O', b'N', b'N', 
    b'I', b'D', b'_', b'4', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x00, 0x00,
];

const MESSAGE_5: [u8; 47] = [
    // Type: 'O'
    b'O',
    // UserRefNum: 1u32
    0x05, 0x00, 0x00, 0x00,
    // Side: 'S' (Sell)
    b'S',
    // Quantity: 300u32 (Little Endian: 2C 01 00 00)
    0x2C, 0x01, 0x00, 0x00, 
    // Symbol: "TSLA    "
    SYMBOL_TSLA[0], SYMBOL_TSLA[1], SYMBOL_TSLA[2], SYMBOL_TSLA[3], SYMBOL_TSLA[4], SYMBOL_TSLA[5], SYMBOL_TSLA[6], SYMBOL_TSLA[7],
    // Price: 950.00
    PRICE_950_LE[0], PRICE_950_LE[1], PRICE_950_LE[2], PRICE_950_LE[3], PRICE_950_LE[4], PRICE_950_LE[5], PRICE_950_LE[6], PRICE_950_LE[7],
    // TIF, Display, Capacity, etc. (Original values retained)
    b'0', b'Y', b'A', b'Y', b'N', 
    b'I', b'D', b'_', b'1', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x00, 0x00,
];


// --- Array of all generated messages (for inspection) ---
// Note: MESSAGE_5 must be handled separately due to its different size (56 bytes)
fn get_messages() -> Vec<&'static [u8]>{
    let mut messages: Vec<&[u8]> = Vec::new();

    // Copy the fixed-size messages (47 bytes)
    messages.push(&MESSAGE_1);
    messages.push(&MESSAGE_2);
    messages.push(&MESSAGE_3);
    messages.push(&MESSAGE_4);
    messages.push(&MESSAGE_5);

    messages
}

fn main() {
    // 1. Initialize the single Queue instance.
    let queue = SPSC::<ProtocolRequest>::new();
    // 2. Use the split() method to break the queue into the two handles.
    // The (tx, rx) convention is standard for Sender and Receiver.
    // This consumes the 'queue' variable and strategically leaks the memory.
    
    let (tx, rx) = queue.split();
    let messages = get_messages();
    let parser = Parser::new(tx);

    let mut price_matcher = PriceMatcher::new();

    // --- Thread 1: Sender (The Producer) ---
    // The 'move' keyword transfers ownership of the 'tx' handle to the new thread.

    let producer_thread = std::thread::spawn(move || {
        for &message in messages.iter() {
            parser.parse(message);
        }
    });

    // --- Thread 2: Receiver (The Consumer) ---
    // The 'move' keyword transfers ownership of the 'rx' handle to the new thread.
    let consumer_thread = std::thread::spawn(move || {
        loop {
            match rx.recv() {
            Ok(item) => {
                match item {
                    ProtocolRequest::EnterOrder(order) => {
                        if order.side == b'B' {
                            price_matcher.add_bid_order( 
                                order.user_ref_num, 
                                order.quantity,
                                order.price as usize
                            );
                        }else {
                            price_matcher.add_ask_order(
                                order.user_ref_num, 
                                order.quantity,
                                order.price as usize
                            );
                        }
                        price_matcher.process_order();
                    }
                    _ => continue,
                }
                
                },
                Err(()) => continue,
            }
        }
    });

    // Wait for the threads to finish execution.
    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}
