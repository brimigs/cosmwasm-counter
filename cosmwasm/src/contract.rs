use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw_storage_plus::Map;
use cosmwasm_schema::{cw_serde, QueryResponses};\

/// CosmWasm Contract State Definition 
pub const USER_COUNT: Map<String, u64> = Map::new("count");

/// CosmWasm Contract Messages
#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Increment,
    Decrement,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    GetUserCount { user: String },
}

/// CosmWasm Contract Entry Points
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment => {
            USER_COUNT.update(deps.storage, &info.sender.to_string(), count {
                Ok(count.unwrap_or_default() + 1)
            })?;
            Ok(Response::new().add_attribute("execute", "increment"))
        },
        ExecuteMsg::Decrement => {
            USER_COUNT.update(deps.storage, &info.sender.to_string(), count {
                let mut count = count.unwrap_or_default();
                if count > 0 {
                    count -= 1;
                }
                Ok(count)
            })?;
            Ok(Response::new().add_attribute("execute", "decrement"))
        },
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    info: QueryMsg,
) -> StdResult<Binary> {
    let count = USER_COUNT.may_load(deps.storage, &info.user)?.unwrap_or_default();
    Ok(to_binary(&count)?)
}
