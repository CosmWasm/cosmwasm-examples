use cosmwasm_std::Uint64;
use cw_storage_plus::Item;

pub const PING_COUNT: Item<Uint64> = Item::new("ping_count");
