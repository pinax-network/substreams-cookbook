use crate::{abi};
use substreams::Hex;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;
use abi::erc20::events::{Transfer, Approval};
use crate::pb::erc20::types::v1::{TransferEvent, TransferEvents, ApprovalEvent, ApprovalEvents, BalanceOfStorageChange, BalanceOfStorageChanges};

#[substreams::handlers::map]
pub fn map_transfer(block: Block) -> Result<TransferEvents, Error> {
    let mut events = vec![];

    for log in block.logs() {
        if !Transfer::match_log(log.log) { continue; } // no data
        let event = Transfer::decode(log.log).unwrap();

        events.push(TransferEvent {
            // contract address
            address: Hex::encode(log.address()),

            // event payload
            from: Hex::encode(event.from),
            to: Hex::encode(event.to),
            value: event.value.to_string(),

            // trace information
            transaction: Hex::encode(&log.receipt.transaction.hash),
        })
    }
    Ok(TransferEvents{events})
}

#[substreams::handlers::map]
pub fn map_approval(block: Block) -> Result<ApprovalEvents, Error> {
    let mut events = vec![];

    for log in block.logs() {
        if !Approval::match_log(log.log) { continue; } // no data
        let event = Approval::decode(log.log).unwrap();

        events.push(ApprovalEvent {
            // contract address
            address: Hex::encode(log.address()),

            // event payload
            owner: Hex::encode(event.owner),
            spender: Hex::encode(event.spender),
            value: event.value.to_string(),

            // trace information
            transaction: Hex::encode(&log.receipt.transaction.hash),
        })
    }
    Ok(ApprovalEvents{events})
}

#[substreams::handlers::map]
pub fn map_balance_of(block: Block) -> Result<BalanceOfStorageChanges, Error> {
    let mut storage_changes = vec![];

    // ETH calls
    for calls in block.calls() {
        // filter by calls containing 36 bytes of raw data
        let input = calls.call.clone().input;
        if input.len() < 36 { continue; } // skip if not 36 bytes

        // filter by method selector
        // 0xa9059cbb => transfer(address,uint256)
        // 0x23b872dd => transferFrom(address,address,uint256)
        let method = Hex::encode(&input[0..4]);
        if !["a9059cbb", "23b872dd"].contains(&method.as_str()) { continue; }

        // Storage changes
        for storage_change in &calls.call.storage_changes {
            storage_changes.push(BalanceOfStorageChange {
                // contract address
                address: Hex::encode(&storage_change.address),
                method: method.to_string(),

                // storage changes
                owner: Hex::encode(&calls.call.caller),
                balance: Hex::encode(&storage_change.new_value),

                // trace information
                transaction: Hex::encode(&calls.transaction.hash),
            })
        }
    }
    Ok(BalanceOfStorageChanges { storage_changes })
}
