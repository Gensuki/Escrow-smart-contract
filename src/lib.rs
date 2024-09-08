use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo, StdResult, Response, Binary};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

pub mod contract;
pub mod error;
pub mod msg;
pub mod state;
pub mod helpers;

#[cfg(target_arch = "wasm32")]
#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, _msg: InstantiateMsg) -> StdResult<Response> {
    contract::instantiate(deps, _env, info, _msg)
}

#[cfg(target_arch = "wasm32")]
#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    contract::execute(deps, _env, info, msg)
}

#[cfg(target_arch = "wasm32")]
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    contract::query(deps, _env, msg)
}
