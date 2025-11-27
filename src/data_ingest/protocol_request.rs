use crate::data_ingest::data_types::*;

pub enum ProtocolRequest<'a> {
    EnterOrder(&'a EnterOrderRequest),
    ReplaceOrder(&'a ReplaceOrderRequest),
    CancelOrder(&'a CancelOrderRequest),
    ModifyOrder(&'a ModifyOrderRequest),
    MassCancel(&'a MassCancelRequest),
    DisableOrderEntry(&'a DisableOrderEntryRequest),
    EnableOrderEntry(&'a EnableOrderEntryRequest),
    AccountQuery(&'a AccountQueryRequest),
}