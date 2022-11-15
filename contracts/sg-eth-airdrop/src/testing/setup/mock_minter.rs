use crate::error::ContractError;
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, StdResult, Timestamp,
};
use sg_std::Response;
use vending_factory::msg::VendingMinterCreateMsg;
use vending_minter::msg::{ExecuteMsg, QueryMsg};

use cosmwasm_schema::cw_serde;
#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub base_token_uri: String,
    pub num_tokens: u32,
    pub per_address_limit: u32,
    pub sg721_address: String,
    pub sg721_code_id: u64,
    pub start_time: Timestamp,
    pub mint_price: Coin,
    pub whitelist: Option<String>,
    pub factory: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: VendingMinterCreateMsg,
) -> Result<Response, ContractError> {
    let res = Response::new();
    Ok(res)
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(ContractError::CollectionWhitelistMinterNotSet {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config()),
        _ => to_binary("invalid"),
    }
}

fn query_config() -> ConfigResponse {
    println!("config is being returned hhere");
    ConfigResponse {
        admin: "some_admin".to_string(),
        whitelist: Some("contract2".to_string()),
        base_token_uri: "some_uri".to_string(),
        num_tokens: 5,
        per_address_limit: 5,
        sg721_address: "some_sg721_address".to_string(),
        sg721_code_id: 4,
        start_time: Timestamp::from_seconds(30),
        mint_price: Coin::new(1000, "ustars"),
        factory: "some_factory".to_string(),
    }
}
