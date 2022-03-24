#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::msg::{GmCountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:flashloan-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Use CW2 to set the contract version, this is needed for migrations
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State {
        vault_address: deps.api.addr_validate(&msg.vault_address)?,
    };

    // Store the initial config
    STATE.save(deps.storage, &state)?;
    BASE_ASSET.save(
        deps.storage,
        &ArbBaseAsset {
            asset_info: msg.asset_info,
        },
    )?;
    // Setup the admin as the creator of the contract
    ADMIN.set(deps, Some(info.sender))?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("base_asset", msg.asset_info)
        .add_attribute("vault_address", msg.vault_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::GoodMorning {} => try_good_morning(deps),
    }
}

/// attempt to say good_morning. A simple example of how you can handle an ExecuteMsg with no input.
pub fn try_good_morning(deps: DepsMut) -> Result<Response, ContractError> {
    // Rework the count example from cw-template to be a count of how many times GoodMorning has been called
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_good_morning"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

/// query how many times good morning has been called.
fn query_count(deps: Deps) -> StdResult<GmCountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GmCountResponse { count: state.count })
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    // use cosmwasm_std::{coins, from_binary};

    // This is only here if you wish to have unit tests in the contract file.
    // If you do not. Remove this. Another way to store tests is by creating a 'testing' module 
    // which houses all the tests.
}