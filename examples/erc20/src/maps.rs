use crate::abi;
use substreams::{hex, Hex};
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;
use abi::erc20::events::{Transfer, Approval};
use crate::pb::erc20::types::v1::{TransferEvent, TransferEvents, ApprovalEvent, ApprovalEvents, BalanceOfStorageChange, BalanceOfStorageChanges};

// Token contract addresses
pub const TETHER: [u8; 20] = hex!("dac17f958d2ee523a2206206994597c13d831ec7");
pub const USDC: [u8; 20] = hex!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");
pub const ADDRESSES: &[&[u8]] = &[&TETHER, &USDC];

#[substreams::handlers::map]
pub fn map_transfer(block: Block) -> Result<TransferEvents, Error> {
    Ok(TransferEvents {
        events: block
            .events::<Transfer>(&ADDRESSES)
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
pub fn map_approval(block: Block) -> Result<ApprovalEvents, Error> {
    Ok(ApprovalEvents {
        events: block
            .events::<Approval>(&ADDRESSES)
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
pub fn map_balance_of(block: Block) -> Result<BalanceOfStorageChanges, Error> {
    let mut storage_changes = vec![];

    // ETH calls
    for calls in block.calls() {
        // Storage changes
        for storage_change in &calls.call.storage_changes {
            let address = Hex::encode(&storage_change.address);
            if address != "dac17f958d2ee523a2206206994597c13d831ec7" { continue; }

            storage_changes.push(BalanceOfStorageChange {
                // contract address
                address,

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
