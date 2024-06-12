use crate::state::USER_COUNT;
use cosmwasm_std::{Deps, Order, StdError, StdResult};

pub const DEFAULT_LIMIT: u32 = 10;

// Query specific offers that have already been fulfilled
pub fn query_count(deps: Deps user: String) -> Result<u64, StdError> { 
    let count = USER_COUNT.may_load(deps.storage, &user)?;
    Ok(offer)
}