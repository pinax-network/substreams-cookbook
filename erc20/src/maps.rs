use substreams::Hex;
use substreams::log;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;
use crate::erc20::{TransferEvent,TransferEvents};

#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<TransferEvents, Error> {
    let mut events = vec![];
    log::info!("events: {:?}", events);
    for trace in block.transaction_traces.clone() {
        for calls in trace.calls {
            let address = Hex(calls.address).to_string();
            log::info!("call.address: {:?}", address);
            events.push(TransferEvent{
                from: address.clone(),
                to: address,
            });
        }
    }
    Ok(TransferEvents {events})
}