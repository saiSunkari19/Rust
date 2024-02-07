use pyth_sdk_cw::{Price, PriceIdentifier};
use std::str;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;
use cosmwasm_std::Coin;
use std::time::Duration;

#[cw_serde]
pub struct MigrationMsg {}

#[cw_serde]
pub struct InstantiateMsg {
    pub price_feed_id: PriceIdentifier,
    pub pyth_contract_addr: String,
}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(FetchPriceResponse)]
    FetchPrice {},

    #[returns(Coin)]
    FetchUpdateFee { vaas: Vec<Binary> },

    #[returns(Duration)]
    FetchValidTimePeriod,
}

#[cw_serde]
pub struct FetchPriceResponse {
    pub current_price: Price,
    pub ema_price: Price,
}
