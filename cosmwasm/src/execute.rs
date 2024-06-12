use crate::error::ContractError;
use crate::error::ContractError::{InaccurateFunds, InvalidTaker, NoOfferFound, Unauthorized};
use crate::msg::{ExecuteMsg, Offer};
use crate::state::{FULFILLED_OFFERS, OFFERS, OFFER_ID_COUNTER};
use cosmwasm_std::{BankMsg, Binary, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response};
use osmosis_std::types::cosmos::authz::v1beta1::MsgExec;
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
use osmosis_std::types::cosmos::base::v1beta1::Coin as Coin2;
use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContract;

pub fn increment_counter(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Get User 
    let user = info.sender.to_string();

    // Initialize or load the user's counter
    let count = match USER_COUNT.may_load(deps.storage, &user)? {
        Some(counter) => counter + 1,
        None => {
            // If the counter doesn't exist in storage, set its initial value to 1
            1
        }
    };

    USER_COUNT.save(deps.storage, &count, &user)?;

    Ok(Response::new()
        .add_attribute("user", &user)
        .add_attribute("count", &count.to_string()))
}
pub fn decrement_counter(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Get User 
    let user = info.sender.to_string();

    // Initialize or load the user's counter
    let count = match USER_COUNT.may_load(deps.storage, &user)? {
        Some(counter) => counter - 1,
        None => {
            // If the counter doesn't exist in storage, set its initial value to 1
            1
        }
    };

    USER_COUNT.save(deps.storage, &count, &user)?;

    Ok(Response::new()
        .add_attribute("user", &user)
        .add_attribute("count", &count.to_string()))
}

pub fn reset_counter(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    count: u64,
) -> Result<Response, ContractError> {
    // Get User 
    let user = info.sender.to_string();

    USER_COUNT.save(deps.storage, &count, &user)?;

    Ok(Response::new()
        .add_attribute("user", &user)
        .add_attribute("count", &count.to_string())
    )
}