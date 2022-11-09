use cosmwasm_std::{Addr, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    beacon::{CalculateFeeQuery, RequestEntropyMsg, UpdateConfigMsg},
    provide::{
        ActiveRequestsQuery, AdminReturnDepositMsg, BeaconConfigQuery, KeyStatusQuery,
        LastEntropyQuery, ReclaimDepositMsg, SubmitEntropyMsg, WhitelistPublicKeyMsg,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    WhitelistPublicKey(WhitelistPublicKeyMsg),
    ReclaimDeposit(ReclaimDepositMsg),
    SubmitEntropy(SubmitEntropyMsg),
    RequestEntropy(RequestEntropyMsg),
    UpdateConfig(UpdateConfigMsg),
    AdminReturnDeposit(AdminReturnDepositMsg),
    UpdateGasPrice(Decimal),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    KeyStatus(KeyStatusQuery),
    LastEntropy(LastEntropyQuery),
    ActiveRequests(ActiveRequestsQuery),
    BeaconConfig(BeaconConfigQuery),
    CalculateFee(CalculateFeeQuery),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub whitelist_deposit_amt: Uint128,
    pub refund_increment_amt: Uint128,
    pub key_activation_delay: u64,
    pub protocol_fee: u64,
    pub submitter_share: u64,
    pub native_denom: String,
    pub whitelisted_keys: Vec<(Addr, crate::proof::PublicKey)>,
    pub belief_gas_price: Decimal,
    pub permissioned: bool,
    pub test_mode: bool,
}
