use cosmwasm_std::{Addr, Deps, StdResult};

pub fn validate_address(deps: Deps, addr: &str) -> StdResult<Addr> {
    deps.api.addr_validate(addr)
}
