use std::time::Duration;
use rand::Rng;

use hft_trading::low_latency_comm::{SPSC, Sender, Receiver};
use hft_trading::data_ingest::{Parser};
use hft_trading::data_ingest::ProtocolRequest;



const MESSAGE_1: [u8; 47] = [
    // Type (Offset 0, Length 1): 'O'
    b'O',
    // UserRefNum (Offset 1, Length 4): 1u32 (Little Endian: 01 00 00 00)
    0x01, 0x00, 0x00, 0x00,
    // Side (Offset 5, Length 1): 'B' (Buy)
    b'B',
    // Quantity (Offset 6, Length 4): 100u32 (Little Endian: 64 00 00 00)
    100u32.to_le_bytes()[0], 100u32.to_le_bytes()[1], 100u32.to_le_bytes()[2], 100u32.to_le_bytes()[3],
    // Symbol (Offset 10, Length 8): "GOOG    " (Padded)
    b'G', b'O', b'O', b'G', 0x20, 0x20, 0x20, 0x20,
    // Price (Offset 18, Length 8): 100.50 (Scaled u64: 1005000000000000)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Placeholder for scaled price (100.50)
    // Time In Force (Offset 26, Length 1): '0' (Day)
    b'0',
    // Display (Offset 27, Length 1): 'Y' (Visible)
    b'Y',
    // Capacity (Offset 28, Length 1): 'A' (Agency)
    b'A',
    // InterMarket Sweep Eligibility (Offset 29, Length 1): 'Y' (Eligible)
    b'Y',
    // CrossType (Offset 30, Length 1): 'N' (Continuous Market)
    b'N',
    // ClOrdID (Offset 31, Length 14): "ID_1            " (Padded)
    b'I', b'D', b'_', b'1', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    // Appendage Length (Offset 45, Length 2): 0u16 (No appendage)
    0x00, 0x00,
];

const MESSAGE_2: [u8; 47] = [
    // Type (Offset 0, Length 1): 'O'
    b'O',
    // UserRefNum (Offset 1, Length 4): 2u32
    0x02, 0x00, 0x00, 0x00,
    // Side (Offset 5, Length 1): 'E' (Sell Short Exempt)
    b'E',
    // Quantity (Offset 6, Length 4): 500u32
    500u32.to_le_bytes()[0], 500u32.to_le_bytes()[1], 500u32.to_le_bytes()[2], 500u32.to_le_bytes()[3],
    // Symbol (Offset 10, Length 8): "MSFT    "
    b'M', b'S', b'F', b'T', 0x20, 0x20, 0x20, 0x20,
    // Price (Offset 18, Length 8): 250.75 (Scaled u64)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // Time In Force (Offset 26, Length 1): '3' (IOC - Immediate or Cancel)
    b'3',
    // Display (Offset 27, Length 1): 'N' (Hidden)
    b'N',
    // Capacity (Offset 28, Length 1): 'P' (Principal)
    b'P',
    // InterMarket Sweep Eligibility (Offset 29, Length 1): 'N' (Not Eligible)
    b'N',
    // CrossType (Offset 30, Length 1): 'N' (Continuous Market)
    b'N',
    // ClOrdID (Offset 31, Length 14): "ID_2            "
    b'I', b'D', b'_', b'2', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    // Appendage Length (Offset 45, Length 2): 0u16
    0x00, 0x00,
];

const MESSAGE_3: [u8; 47] = [
    // Type (Offset 0, Length 1): 'O'
    b'O',
    // UserRefNum (Offset 1, Length 4): 3u32
    0x03, 0x00, 0x00, 0x00,
    // Side (Offset 5, Length 1): 'S' (Sell)
    b'S',
    // Quantity (Offset 6, Length 4): 1000u32
    1000u32.to_le_bytes()[0], 1000u32.to_le_bytes()[1], 1000u32.to_le_bytes()[2], 1000u32.to_le_bytes()[3],
    // Symbol (Offset 10, Length 8): "AMZN    "
    b'A', b'M', b'Z', b'N', 0x20, 0x20, 0x20, 0x20,
    // Price (Offset 18, Length 8): 3000.00 (Scaled u64)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // Time In Force (Offset 26, Length 1): '0' (Day) - Cross orders are usually treated as Day
    b'0',
    // Display (Offset 27, Length 1): 'A' (Attributable)
    b'A',
    // Capacity (Offset 28, Length 1): 'R' (Riskless)
    b'R',
    // InterMarket Sweep Eligibility (Offset 29, Length 1): 'N'
    b'N',
    // CrossType (Offset 30, Length 1): 'C' (Closing Cross)
    b'C',
    // ClOrdID (Offset 31, Length 14): "ID_3            "
    b'I', b'D', b'_', b'3', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    // Appendage Length (Offset 45, Length 2): 0u16
    0x00, 0x00,
];


const MESSAGE_4: [u8; 47] = [
    // Type (Offset 0, Length 1): 'O'
    b'O',
    // UserRefNum (Offset 1, Length 4): 4u32
    0x04, 0x00, 0x00, 0x00,
    // Side (Offset 5, Length 1): 'B' (Buy)
    b'B',
    // Quantity (Offset 6, Length 4): 50u32
    50u32.to_le_bytes()[0], 50u32.to_le_bytes()[1], 50u32.to_le_bytes()[2], 50u32.to_le_bytes()[3],
    // Symbol (Offset 10, Length 8): "BABA    "
    b'B', b'A', b'B', b'A', 0x20, 0x20, 0x20, 0x20,
    // Price (Offset 18, Length 8): 10.00 (Scaled u64)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // Time In Force (Offset 26, Length 1): 'E' (After Hours)
    b'E',
    // Display (Offset 27, Length 1): 'Y'
    b'Y',
    // Capacity (Offset 28, Length 1): 'O' (Other)
    b'O',
    // InterMarket Sweep Eligibility (Offset 29, Length 1): 'N'
    b'N',
    // CrossType (Offset 30, Length 1): 'N' (Continuous Market)
    b'N',
    // ClOrdID (Offset 31, Length 14): "ID_4            "
    b'I', b'D', b'_', b'4', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    // Appendage Length (Offset 45, Length 2): 0u16
    0x00, 0x00,
];

const MESSAGE_5_BASE: [u8; 47] = [
    // Type (Offset 0, Length 1): 'O'
    b'O',
    // UserRefNum (Offset 1, Length 4): 5u32
    0x05, 0x00, 0x00, 0x00,
    // Side (Offset 5, Length 1): 'B' (Buy)
    b'B',
    // Quantity (Offset 6, Length 4): 200u32
    200u32.to_le_bytes()[0], 200u32.to_le_bytes()[1], 200u32.to_le_bytes()[2], 200u32.to_le_bytes()[3],
    // Symbol (Offset 10, Length 8): "AAPL    "
    b'A', b'A', b'P', b'L', 0x20, 0x20, 0x20, 0x20,
    // Price (Offset 18, Length 8): 150.00 (Scaled u64)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // Time In Force (Offset 26, Length 1): '0' (Day)
    b'0',
    // Display (Offset 27, Length 1): 'Y'
    b'Y',
    // Capacity (Offset 28, Length 1): 'A' (Agency)
    b'A',
    // InterMarket Sweep Eligibility (Offset 29, Length 1): 'Y'
    b'Y',
    // CrossType (Offset 30, Length 1): 'N' (Continuous Market)
    b'N',
    // ClOrdID (Offset 31, Length 14): "ID_5            "
    b'I', b'D', b'_', b'5', 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    // Appendage Length (Offset 45, Length 2): 9u16 (Length of APPENDAGE_MIN_QTY)
    9u16.to_le_bytes()[0], 9u16.to_le_bytes()[1],
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
                println!("[Receiver] Received");
                match item {
                    ProtocolRequest::EnterOrder(order) => {
                        println!("{:?}", order);
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
