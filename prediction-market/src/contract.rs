use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};
use pyth_sdk_cw::query_price_feed;

use crate::{
    msg::{FetchPriceResponse, InstantiateMsg, QueryMsg},
    state::{State, STATE},
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        pyth_contract_addr: deps.api.addr_validate(msg.pyth_contract_addr.as_ref())?,
        price_feed_id: msg.price_feed_id,
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("price_id", format!("{}", msg.price_feed_id)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FetchPrice {} => to_json_binary(&query_fetch_price(deps, env)?),
        QueryMsg::FetchUpdateFee { vaas } => to_json_binary(""),
        QueryMsg::FetchValidTimePeriod => to_json_binary(""),
    }
}

fn query_fetch_price(deps: Deps, env: Env) -> StdResult<FetchPriceResponse> {
    let state = STATE.load(deps.storage)?;

    let price_feed_response: pyth_sdk_cw::PriceFeedResponse =
        query_price_feed(&deps.querier, state.pyth_contract_addr, state.price_feed_id)?;

    let price_feed = price_feed_response.price_feed;

    // Get the current price and confidence interaval from the price feed
    let current_price = price_feed
        .get_price_no_older_than(env.block.time.seconds() as i64, 60)
        .ok_or_else(|| StdError::not_found("Current price not available"))?;

    // Get expoenentially-weighted moving average price and confidence level

    let ema_price = price_feed
        .get_ema_price_no_older_than(env.block.time.seconds() as i64, 60)
        .ok_or_else(|| StdError::not_found("EMA price is not avilable"))?;

    Ok(FetchPriceResponse {
        current_price,
        ema_price,
    })
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{coins, Addr};
    use pyth_sdk_cw::PriceIdentifier;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    // https://docs.pyth.network/documentation/pythnet-price-feeds/cosmwasm
    const PYTH_CONTRACT_ADDR: &str = "inj12j43nf2f0qumnt2zrrmpvnsqgzndxefujlvr08";
    const PRICE_FEED_ID: &str = "4ea5bb4d2f5900cc2e97ba534240950740b4d3b89fe712a94a7304fd2fd92702"; // AKT
    #[test]
    fn test_instantiate_msg() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &coins(100, "earth"));

        let msg = InstantiateMsg {
            price_feed_id: PriceIdentifier::from_hex(PRICE_FEED_ID).unwrap(),
            pyth_contract_addr: Addr::unchecked(PYTH_CONTRACT_ADDR).to_string(),
        };

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        dbg!(res);
    }

    #[test]
    fn test_query_price() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &coins(100, "earth"));

        let msg = InstantiateMsg {
            price_feed_id: PriceIdentifier::from_hex(PRICE_FEED_ID).unwrap(),
            pyth_contract_addr: Addr::unchecked(PYTH_CONTRACT_ADDR).to_string(),
        };

        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = QueryMsg::FetchPrice {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        dbg!(res);
    }
}
