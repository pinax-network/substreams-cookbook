use substreams::errors::Error;
use substreams_entity_change::pb::entity::{entity_change, EntityChanges};
use crate::pb::erc20::types::v1::{TransferEvents, ApprovalEvents};

#[substreams::handlers::map]
pub fn graph_out(map_transfers: TransferEvents, map_approvals: ApprovalEvents) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    for event in map_transfers.events {
        let id = format!("{}-{}", event.transaction_hash, event.index);

        entity_changes
            .push_change("transfer", id.as_str(), 0, entity_change::Operation::Create)
            // contract address
            .change("address", event.address)

            // event payload
            .change("from", event.from)
            .change("to", event.to)
            .change("value", event.value)

            // trace information
            .change("transaction_hash", event.transaction_hash)
            .change("gas_used", event.gas_used)
            .change("begin_ordinal", event.begin_ordinal)
            .change("end_ordinal", event.end_ordinal)
            .change("index", event.index.to_string())
            .change("sender", event.sender)
            .change("interacted_with", event.interacted_with);
    }

    for event in map_approvals.events {
        let id = format!("{}-{}", event.transaction_hash, event.index);

        entity_changes
            .push_change("approval", id.as_str(), 0, entity_change::Operation::Create)
            // contract address
            .change("address", event.address)

            // event payload
            .change("owner", event.owner)
            .change("spender", event.spender)
            .change("value", event.value)

            // trace information
            .change("transaction_hash", event.transaction_hash)
            .change("gas_used", event.gas_used)
            .change("begin_ordinal", event.begin_ordinal)
            .change("end_ordinal", event.end_ordinal)
            .change("index", event.index.to_string())
            .change("sender", event.sender)
            .change("interacted_with", event.interacted_with);
    }

    Ok(entity_changes)
}