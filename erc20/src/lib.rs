use substreams::Hex;
use substreams::log;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<TransferEvents, Error> {
    for trace in block.transaction_traces.clone() {
        log::info!("trace: {:?}", trace);
    }

    Ok(TransferEvents {})
}