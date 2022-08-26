use cosmwasm_std::{Addr, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::proof::{Proof, PublicKey};

/*
 * Provides contract interfaces that are part of the system to
 * provide entropy to the beacon contract.
*/
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WhitelistPublicKeyMsg {
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ReclaimDepositMsg {
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SubmitEntropyMsg {
    pub proof: Proof,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AdminReturnDepositMsg {
    pub key: PublicKey,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct KeyStatusQuery {
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct KeyStatusResponse {
    /// Is the key whitelisted?
    pub whitelisted: bool,
    /// Is the key active?
    pub active: bool,
    /// What height the key will become active / has been activated at.
    pub activation_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LastEntropyQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LastEntropyResponse {
    pub entropy: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ActiveRequestsQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ActiveRequestsResponse {
    pub requests: Vec<ActiveRequestInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BeaconConfigQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct BeaconConfigResponse {
    pub whitelist_deposit_amt: Uint128,
    pub key_activation_delay: u64,
    pub protocol_fee: u64,
    pub submitter_share: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ActiveRequestInfo {
    ///How much gas the requester has provisioned for their callback transaction.
    pub callback_gas_limit: u64,
    ///The address to send the callback message to.
    pub callback_address: Addr,
    ///The address that we received the request from.
    pub submitter: Addr,
    ///The block that the request was received on.
    pub submitted_block_height: u64,
    ///The amount of tokens left after subtracting the requested gas.
    pub submitted_bounty_amount: Uint128,
}

