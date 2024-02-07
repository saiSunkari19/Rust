use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use pyth_sdk_cw::PriceIdentifier;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct State {
    pub price_feed_id: PriceIdentifier,
    pub pyth_contract_addr: Addr,
}


pub const STATE: Item<State> = Item::new("state");



// #[cfg(test)]
// mod test{

//     use super::*;

//     #[test]
//     fn test_state(){
//         let state = State{
//             pyth_contract_addr: Addr::from_str(""),
//             price_feed_id : PriceIdentifier::new("empty new".as_bytes())
//         };
        
//     }
// }