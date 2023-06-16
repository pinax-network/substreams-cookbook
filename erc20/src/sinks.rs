
use substreams::errors::Error;
use substreams_entity_change::pb::entity::{entity_change, EntityChanges};

use crate::erc20::{TransferEvents};

#[substreams::handlers::map]
pub fn graph_out(map_transfers: TransferEvents) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    for transfer in map_transfers.events {
        entity_changes
            .push_change("transfer", transfer.from.as_str(), 0, entity_change::Operation::Create);
    }

    Ok(entity_changes)
}