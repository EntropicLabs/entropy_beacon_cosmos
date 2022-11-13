use cosmwasm_std::{Addr, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::proof::{Proof, PublicKey};

pub const MAX_PAGINATION_LIMIT: u32 = 30;
pub const DEFAULT_PAGINATION_LIMIT: u32 = 10;

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
    pub request_ids: Vec<Uint128>,
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
pub struct ActiveRequestsQuery {
    pub start_after: Option<Uint128>,
    pub limit: Option<u32>,
}

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
    ///The amount of tokens that must be deposited to whitelist a new public key.
    pub whitelist_deposit_amt: Uint128,
    ///The amount of the deposit that unlocks with each submission of entropy.
    pub refund_increment_amt: Uint128,
    ///The time, in blocks, before a whitelisted public key can be used to submit entropy.
    pub key_activation_delay: u64,
    ///The fee that the protocol contract charges on top of the requested gas fees.
    pub protocol_fee: u64,
    ///The share of the protocol fee that is distributed to the wallet submitting entropy.
    pub submitter_share: Decimal,
    ///The native currency of the target chain.
    pub native_denom: String,
    ///Whether or not the contract is paused.
    pub paused: bool,
    ///Whether or not the contract is in permissioned mode.
    pub permissioned: bool,
    ///Whether or not the contract is in test mode.
    pub test_mode: bool,
    ///Whether or not callback subsidization is enabled.
    pub subsidize_callbacks: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ActiveRequestInfo {
    ///The ID of the request
    pub id: Uint128,
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