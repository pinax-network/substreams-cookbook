use substreams::errors::Error;
use substreams_entity_change::pb::entity::{entity_change, EntityChanges};
use crate::pb::erc20::types::v1::{TransferEvents, ApprovalEvents, BalanceOfStorageChanges};

#[substreams::handlers::map]
pub fn graph_out(map_transfer: TransferEvents, map_approval: ApprovalEvents, map_balance_of: BalanceOfStorageChanges) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    for event in map_transfer.events {
        let id = format!("{}:{}", event.from, event.transaction);

        entity_changes
            .push_change("transfer", id.as_str(), 0, entity_change::Operation::Create)
            // contract address
            .change("address", event.address)

            // event payload
            .change("from", event.from)
            .change("to", event.to)
            .change("value", event.value)

            // trace information
            .change("transaction", event.transaction);
    }

    for event in map_approval.events {
        let id = format!("{}:{}:{}", event.owner, event.spender, event.transaction);

        entity_changes
            .push_change("approval", id.as_str(), 0, entity_change::Operation::Create)
            // contract address
            .change("address", event.address)

            // event payload
            .change("owner", event.owner)
            .change("spender", event.spender)
            .change("value", event.value)

            // trace information
            .change("transaction", event.transaction);
    }

    for storage_change in map_balance_of.storage_changes {
        let id = format!("{}:{}", storage_change.owner, storage_change.transaction);

        entity_changes
            .push_change("balanceof", id.as_str(), 0, entity_change::Operation::Create)
            // contract address
            .change("address", storage_change.address)
            .change("method", storage_change.method)

            // storage change
            .change("owner", storage_change.owner)
            .change("balance", storage_change.balance)

            // trace information
            .change("transaction", storage_change.transaction);
    }

    Ok(entity_changes)
}