use substreams::errors::Error;
use substreams_entity_change::pb::entity::{entity_change, EntityChanges};
use crate::pb::erc20::types::v1::{TransferEvents};

#[substreams::handlers::map]
pub fn graph_out(map_transfers: TransferEvents) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    for transfer in map_transfers.events {
        let id = format!("{}-{}", transfer.transaction_hash, transfer.index);

        entity_changes
            .push_change("transfer", id.as_str(), 0, entity_change::Operation::Create)
            .change("address", transfer.address)
            .change("from", transfer.from)
            .change("to", transfer.to)
            .change("value", transfer.value)
            .change("transaction_hash", transfer.transaction_hash)
            .change("gas_used", transfer.gas_used)
            .change("begin_ordinal", transfer.begin_ordinal)
            .change("end_ordinal", transfer.end_ordinal)
            .change("index", transfer.index.to_string())
            .change("sender", transfer.sender)
            .change("interacted_with", transfer.interacted_with);
    }

    Ok(entity_changes)
}