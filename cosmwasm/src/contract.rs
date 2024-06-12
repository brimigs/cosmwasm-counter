use crate::error::ContractError;
use crate::execute::{fulfill_offer, make_offer, provide_taker};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_all_offers, query_fulfilled_offers};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::IncrementCounter { } => increment_counter(deps, env, info),
        ExecuteMsg::DecrementCounter { } => decrement_counter(deps, env, info),
        ExecuteMsg::ResetCounter { count: u64 } => reset_counter(deps, env, info, count),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, StdError> {
    pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        }
    }
}

