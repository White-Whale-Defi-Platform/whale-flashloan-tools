use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, Empty, StdResult, Uint128, WasmMsg,
};
use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use terraswap::asset::{AssetInfo};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub vault_address: String,
    pub asset_info: AssetInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Example ExecuteMsg with no Inputs
    GoodMorning {},
    // Attempt to call a FlashLoan of a specified amount providing the subsequent messages which will have access to this liquidity within the block
    FlashLoan {
        amount: Uint128,
        msgs: Vec<CosmosMsg<Empty>>,
    },
    ExecuteCallback {
        msgs: Vec<CosmosMsg<Empty>>,
    },
    Callback(CallbackMsg),
}

// Modified from
// https://github.com/CosmWasm/cosmwasm-plus/blob/v0.2.3/packages/cw20/src/receiver.rs#L15
impl CallbackMsg {
    pub fn to_cosmos_msg<T: Clone + fmt::Debug + PartialEq + JsonSchema>(
        &self,
        contract_addr: &Addr,
    ) -> StdResult<CosmosMsg<T>> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: String::from(contract_addr),
            msg: to_binary(&ExecuteMsg::Callback(self.clone()))?,
            funds: vec![],
        }))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CallbackMsg {
    AfterSuccessfulExecuteCallback {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GmCountResponse {
    pub count: i32,
}