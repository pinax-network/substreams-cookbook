use crate::abi;
use substreams::{log, hex, Hex};
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;
use abi::erc20::events::{Transfer, Approval};
use crate::pb::erc20::types::v1::{TransferEvent, TransferEvents, ApprovalEvent, ApprovalEvents, BalanceOfStorageChange, BalanceOfStorageChanges};

// to be changed as Param
pub const TETHER: [u8; 20] = hex!("dac17f958d2ee523a2206206994597c13d831ec7");
pub const USDC: [u8; 20] = hex!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");

#[substreams::handlers::map]
pub fn map_transfer(block: Block) -> Result<TransferEvents, Error> {
    Ok(TransferEvents {
        events: block
            .events::<Transfer>(&[&TETHER, &USDC])
            .filter_map(|(event, log)| {
                Some(TransferEvent {
                    // contract address
                    address: Hex(log.address()).to_string(),

                    // event payload
                    from: Hex(event.from).to_string(),
                    to: Hex(event.to).to_string(),
                    value: event.value.to_string(),

                    // trace information
                    transaction: Hex(&log.receipt.transaction.hash).to_string(),
                })
            })
            .collect()
    })
}

#[substreams::handlers::map]
pub fn map_approval(block: Block) -> Result<ApprovalEvents, Error> {
    Ok(ApprovalEvents {
        events: block
            .events::<Approval>(&[&TETHER, &USDC])
            .filter_map(|(event, log)| {
                Some(ApprovalEvent {
                    // contract address
                    address: Hex(log.clone().address()).to_string(),

                    // event payload
                    owner: Hex(event.owner).to_string(),
                    spender: Hex(event.spender).to_string(),
                    value: event.value.to_string(),

                    // trace information
                    transaction: Hex(&log.receipt.transaction.hash).to_string(),
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
            let address = Hex(&storage_change.address).to_string();
            if address != "dac17f958d2ee523a2206206994597c13d831ec7" { continue; }

            storage_changes.push(BalanceOfStorageChange {
                // contract address
                address: address.to_string(),

                // storage changes
                owner: Hex(&calls.call.caller).to_string(),
                balance: Hex(&storage_change.new_value).to_string(),

                // trace information
                transaction: Hex(&calls.transaction.hash).to_string(),
            })
        }
    }
    Ok(BalanceOfStorageChanges { storage_changes })
}