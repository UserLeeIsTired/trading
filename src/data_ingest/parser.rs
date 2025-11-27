use std::u8;


use super::protocol_request::ProtocolRequest;

use crate::data_ingest::data_types::*;
use crate::low_latency_comm::{Sender};

pub struct Parser<T> {
    sender: Sender<T>
}

unsafe impl <T: Send> Send for Parser<T> {}

impl<'a> Parser<ProtocolRequest<'a>> {    
    pub fn new(sender: Sender<ProtocolRequest<'a>>) -> Self {
        Parser {
            sender: sender
        }
    }
    pub fn parse(&self, bytes: &'a[u8]) {
        if bytes.is_empty() {
            eprintln!("Error: Received empty byte slice");
            return;
        }

        let request_type = bytes[0];

        let request = match request_type {
            b'O' => { 
                let reference = unsafe {
                    &*(bytes.as_ptr() as *const EnterOrderRequest)
                };
                ProtocolRequest::EnterOrder(reference)
            },
            b'U' => {
                let reference = unsafe {
                    &*(bytes.as_ptr() as *const ReplaceOrderRequest)
                };
                ProtocolRequest::ReplaceOrder(reference)
            },
            b'X' => {
                let reference = unsafe {
                    &*(bytes.as_ptr() as *const CancelOrderRequest)
                };
                ProtocolRequest::CancelOrder(reference)
            },
            b'M' => {
                let reference = unsafe {
                    &*(bytes.as_ptr() as *const ModifyOrderRequest)
                };
                ProtocolRequest::ModifyOrder(reference)
            },
            b'C' => {
                let reference = unsafe {
                    &*(bytes.as_ptr() as *const MassCancelRequest)
                };
                ProtocolRequest::MassCancel(reference)
            },
            b'D' => {
                let reference = unsafe {
                    &*(bytes.as_ptr() as *const DisableOrderEntryRequest)
                };
                ProtocolRequest::DisableOrderEntry(reference)
            },
            b'E' => {
                let reference = unsafe {
                    &*(bytes.as_ptr() as *const EnableOrderEntryRequest)
                };
                ProtocolRequest::EnableOrderEntry(reference)
            },
            b'Q' => {
                let reference = unsafe {
                    &*(bytes.as_ptr() as *const AccountQueryRequest)
                };
                ProtocolRequest::AccountQuery(reference)
            },
            _ => {
                eprintln!("Error: Unknown request type: {}", request_type as char);
                return;
            }
            };

            let _ = self.sender.send(request);
        }
}