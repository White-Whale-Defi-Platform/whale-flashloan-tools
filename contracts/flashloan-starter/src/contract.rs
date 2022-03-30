#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, to_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg};
use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::msg::{CallbackMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, ADMIN, BASE_ASSET, STATE};

use terraswap::asset::{Asset, AssetInfo};
use terraswap::querier::query_balance;

use white_whale::deposit_info::ArbBaseAsset;
use white_whale::ust_vault::msg::ExecuteMsg as VaultMsg;
use white_whale::ust_vault::msg::FlashLoanPayload;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:flashloan-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<Empty>, ContractError> {
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
            asset_info: msg.asset_info.clone(),
        },
    )?;
    // Setup the admin as the creator of the contract
    ADMIN.set(deps, Some(info.sender))?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("base_asset", msg.asset_info.to_string())
        .add_attribute("vault_address", msg.vault_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<Empty>, ContractError> {
    match msg {
        ExecuteMsg::Callback(msg) => _handle_callback(deps, env, info, msg),
        ExecuteMsg::ExecuteCallback { msgs } => handle_callback(env, msgs),
        ExecuteMsg::FlashLoan { amount, msgs } => call_flashloan(deps, env, info, amount, msgs),
        ExecuteMsg::SetAdmin { admin } => {
            let admin_addr = deps.api.addr_validate(&admin)?;
            let previous_admin = ADMIN.get(deps.as_ref())?.unwrap();
            ADMIN.execute_update_admin(deps, info, Some(admin_addr))?;
            Ok(Response::default()
                .add_attribute("previous admin", previous_admin)
                .add_attribute("admin", admin))
        }
        ExecuteMsg::SetVault { vault } => set_vault_addr(deps, info, vault),
    }
}


//----------------------------------------------------------------------------------------
//  EXECUTE FUNCTION HANDLERS
//----------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------
//  PRIVATE FUNCTIONS
//----------------------------------------------------------------------------------------

fn _handle_callback(deps: DepsMut, env: Env, info: MessageInfo, msg: CallbackMsg) -> Result<Response<Empty>, ContractError> {
    // Callback functions can only be called this contract itself
    if info.sender != env.contract.address {
        return Err(ContractError::NotValidCallback {});
    }
    match msg {
        CallbackMsg::AfterSuccessfulExecuteCallback {} => after_successful_exec_callback(deps, env),
        // Possibility to add more callbacks in future.
    }
}
//----------------------------------------------------------------------------------------
//  CALLBACK FUNCTION HANDLERS
//----------------------------------------------------------------------------------------

// After the arb this function returns the funds to the vault.
fn after_successful_exec_callback(deps: DepsMut, env: Env) -> Result<Response<Empty>, ContractError> {
    let state = STATE.load(deps.storage)?;
    let stable_denom = BASE_ASSET.load(deps.storage)?.get_denom()?;
    let stables_in_contract =
        query_balance(&deps.querier, env.contract.address, stable_denom.clone())?;

    // Send asset back to vault
    let repay_asset = Asset {
        info: AssetInfo::NativeToken {
            denom: stable_denom,
        },
        amount: stables_in_contract,
    };

    Ok(Response::new().add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: state.vault_address.to_string(),
        amount: vec![repay_asset.deduct_tax(&deps.querier)?],
    })))
}

fn handle_callback(env: Env, msgs: Vec<CosmosMsg<Empty>>) -> Result<Response<Empty>, ContractError> {
    let callback =
        CallbackMsg::AfterSuccessfulExecuteCallback {}.to_cosmos_msg(&env.contract.address)?;
    Ok(Response::new().add_messages(msgs).add_message(callback))
}


/// Attempt to call a FlashLoan of a specified amount providing the subsequent messages which will have access to this liquidity within the block
fn call_flashloan(
    deps: DepsMut,
    _env: Env,
    _msg_info: MessageInfo,
    amount: Uint128,
    msgs: Vec<CosmosMsg<Empty>>,
) -> Result<Response<Empty>, ContractError> {
    let state = STATE.load(deps.storage)?;
    let deposit_info = BASE_ASSET.load(deps.storage)?;

    // Construct callback msg
    let callback_msg = ExecuteMsg::ExecuteCallback { msgs };
    // Construct payload
    let payload = FlashLoanPayload {
        requested_asset: Asset {
            info: deposit_info.asset_info,
            amount,
        },
        callback: to_binary(&callback_msg)?,
    };

    // Call stablecoin Vault
    Ok(
        Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: state.vault_address.to_string(),
            msg: to_binary(&VaultMsg::FlashLoan { payload })?,
            funds: vec![],
        })),
    )
}

pub fn set_vault_addr(deps: DepsMut, msg_info: MessageInfo, vault_address: String) -> Result<Response<Empty>, ContractError> {
    // Only the admin should be able to call this
    ADMIN.assert_admin(deps.as_ref(), &msg_info.sender)?;

    let mut state = STATE.load(deps.storage)?;
    // Get the old vault
    let previous_vault = state.vault_address.to_string();
    // Store the new vault addr
    state.vault_address = deps.api.addr_validate(&vault_address)?;
    STATE.save(deps.storage, &state)?;
    // Respond and note the previous vault address
    Ok(Response::new()
        .add_attribute("new vault", vault_address)
        .add_attribute("previous vault", previous_vault))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAssetInfo {} => to_binary(&try_query_asset_info(deps)?),
        QueryMsg::GetVault {} => to_binary(&try_query_vault(deps)?),
    }
}

/// query how many times good morning has been called.
pub fn try_query_asset_info(deps: Deps) -> StdResult<ArbBaseAsset> {
    let info: ArbBaseAsset = BASE_ASSET.load(deps.storage)?;
    Ok(info)
}

/// query how many times good morning has been called.
pub fn try_query_vault(deps: Deps) -> StdResult<Addr> {
    let info: State = STATE.load(deps.storage)?;
    Ok(info.vault_address)
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