use crate::pb::erc20::types::v1::Block as Erc20Block;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Erc20Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for event in block.transfers {
        let id = format!("{}-{}", event.block_index, event.transaction);

        tables
            .create_row("Transfer", id)
            // contract address
            .set("address", event.address)
            // event payload
            .set("from", event.from)
            .set("to", event.to)
            .set("value", event.value)
            // trace information
            .set("transaction", event.transaction)
            .set_bigint("block_num", &block_num)
            .set_bigint("timestamp", &timestamp);
    }

    for event in block.approvals {
        let id = format!("{}-{}", event.block_index, event.transaction);

        tables
            .create_row("Approval", id)
            // contract address
            .set("address", event.address)
            // event payload
            .set("owner", event.owner)
            .set("spender", event.spender)
            .set("value", event.value)
            // trace information
            .set("transaction", event.transaction)
            .set_bigint("block_num", &block_num)
            .set_bigint("timestamp", &timestamp);
    }

    for storage_change in block.storage_changes {
        let id = format!("{}:{}", storage_change.address, storage_change.owner);

        tables
            .create_row("BalanceOf", id)
            // contract address
            .set("address", storage_change.address)
            .set("method", storage_change.method)
            // storage change
            .set("owner", storage_change.owner)
            .set("balance", storage_change.balance)
            // trace information
            .set("transaction", storage_change.transaction)
            .set_bigint("block_num", &block_num)
            .set_bigint("timestamp", &timestamp);
    }

    Ok(tables.to_entity_changes())
}
