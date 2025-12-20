pub enum SPSCError {
    Full,
    Empty,
}

pub enum ReceiverError {
    Empty,
    SenderDisconnected,
}

pub enum SenderError {
    Full,
    ReceiverDisconnected,
}

