use crate::{abi};
use substreams::Hex;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;
use abi::erc20::events::{Transfer, Approval};
use crate::pb::erc20::types::v1::{TransferEvent, TransferEvents, ApprovalEvent, ApprovalEvents, BalanceOfStorageChange, BalanceOfStorageChanges};

#[substreams::handlers::map]
pub fn map_transfer(params: String, block: Block) -> Result<TransferEvents, Error> {
    let token_address = Hex::decode(params).unwrap();
    Ok(TransferEvents {
        events: block
            .events::<Transfer>(&[&token_address])
            .filter_map(|(event, log)| {
                Some(TransferEvent {
                    // contract address
                    address: Hex::encode(log.address()),

                    // event payload
                    from: Hex::encode(event.from),
                    to: Hex::encode(event.to),
                    value: event.value.to_string(),

                    // trace information
                    transaction: Hex::encode(&log.receipt.transaction.hash),
                })
            })
            .collect()
    })
}

#[substreams::handlers::map]
pub fn map_approval(params: String, block: Block) -> Result<ApprovalEvents, Error> {
    let token_address = Hex::decode(params).unwrap();
    Ok(ApprovalEvents {
        events: block
            .events::<Approval>(&[&token_address])
            .filter_map(|(event, log)| {
                Some(ApprovalEvent {
                    // contract address
                    address: Hex::encode(log.clone().address()),

                    // event payload
                    owner: Hex::encode(event.owner),
                    spender: Hex::encode(event.spender),
                    value: event.value.to_string(),

                    // trace information
                    transaction: Hex::encode(&log.receipt.transaction.hash),
                })
            })
            .collect()
    })
}

#[substreams::handlers::map]
pub fn map_balance_of(params: String, block: Block) -> Result<BalanceOfStorageChanges, Error> {
    let mut storage_changes = vec![];
    let token_address = Hex::decode(params).unwrap();

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

        // Filter by token address
        if !calls.call.address.eq(&token_address) { continue; }

        // Storage changes
        for storage_change in &calls.call.storage_changes {
            storage_changes.push(BalanceOfStorageChange {
                // contract address
                address: Hex::encode(&storage_change.address),

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
