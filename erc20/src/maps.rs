use substreams::Hex;
use substreams::log;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;
use crate::erc20::{TransferEvents};

#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<TransferEvents, Error> {
    let mut events = vec![];
    for trace in block.transaction_traces.clone() {
        log::info!("trace: {:?}", trace);
    }
    Ok(TransferEvents {events})
}