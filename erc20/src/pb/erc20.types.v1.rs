// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, repeated, tag="1")]
    pub approvals: ::prost::alloc::vec::Vec<ApprovalEvent>,
    #[prost(message, repeated, tag="2")]
    pub transfers: ::prost::alloc::vec::Vec<TransferEvent>,
    #[prost(message, repeated, tag="3")]
    pub storage_changes: ::prost::alloc::vec::Vec<BalanceOfStorageChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    /// contract address
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// event payload
    #[prost(string, tag="2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub value: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="5")]
    pub transaction: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub block_index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApprovalEvent {
    /// contract address
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// event payload
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub spender: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub value: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="5")]
    pub transaction: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub block_index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceOfStorageChange {
    /// contract address
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub method: ::prost::alloc::string::String,
    /// storage changes
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub balance: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="6")]
    pub transaction: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
