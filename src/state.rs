
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// Admin address is stored here
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const OPERATOR: Item<Addr> = Item::new("operator");
