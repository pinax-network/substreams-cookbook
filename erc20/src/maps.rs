use crate::abi;
use substreams::{log, hex, Hex};
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;
use crate::pb::erc20::types::v1::{TransferEvent, TransferEvents, ApprovalEvent, ApprovalEvents};

// to be changed as Param
pub const TETHER: [u8; 20] = hex!("dac17f958d2ee523a2206206994597c13d831ec7");
pub const USDC: [u8; 20] = hex!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");

#[substreams::handlers::map]
pub fn map_transfers(params: String, block: Block) -> Result<TransferEvents, Error> {
    use abi::erc20::events::Transfer;

    // TO-DO : parse params to allow custom contract addresses

    Ok(TransferEvents {
        events: block
            .events::<Transfer>(&[&TETHER, &USDC])
            .filter_map(|(event, log)| {
                let address = Hex(log.clone().address()).to_string();
                let transaction = log.clone().receipt.transaction;
                let transaction_hash = Hex(&transaction.hash).to_string();

                log::info!("TransferEvent: {:}", transaction_hash);
                Some(TransferEvent {
                    // contract address
                    address,

                    // event payload
                    from: Hex(event.from).to_string(),
                    to: Hex(event.to).to_string(),
                    value: event.value.to_string(),

                    // trace information
                    transaction_hash,
                    gas_used: transaction.gas_used,
                    begin_ordinal: transaction.begin_ordinal,
                    end_ordinal: transaction.end_ordinal,
                    index: transaction.index,
                    sender: Hex(&transaction.from).to_string(),
                    interacted_with: Hex(&transaction.to).to_string(),
                })
            })
            .collect()
    })
}

#[substreams::handlers::map]
pub fn map_approvals(params: String, block: Block) -> Result<ApprovalEvents, Error> {
    use abi::erc20::events::Approval;

    // TO-DO : parse params to allow custom contract addresses

    Ok(ApprovalEvents {
        events: block
            .events::<Approval>(&[&TETHER, &USDC])
            .filter_map(|(event, log)| {
                let address = Hex(log.clone().address()).to_string();
                let transaction = log.clone().receipt.transaction;
                let transaction_hash = Hex(&transaction.hash).to_string();

                log::info!("ApprovalEvent: {:}", transaction_hash);
                Some(ApprovalEvent {
                    // contract address
                    address,

                    // event payload
                    owner: Hex(event.owner).to_string(),
                    spender: Hex(event.spender).to_string(),
                    value: event.value.to_string(),

                    // trace information
                    transaction_hash,
                    gas_used: transaction.gas_used,
                    begin_ordinal: transaction.begin_ordinal,
                    end_ordinal: transaction.end_ordinal,
                    index: transaction.index,
                    sender: Hex(&transaction.from).to_string(),
                    interacted_with: Hex(&transaction.to).to_string(),
                })
            })
            .collect()
    })
}