use cosmwasm_schema::{cw_serde, QueryResponses};
use osmosis_std::types::cosmos::base::v1beta1::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    IncrementCounter { },
    DecrementCounter { },
    ResetCounter { count: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    GetCount { user: String },
}