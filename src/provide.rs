use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::proof::{Proof, PublicKey};

/*
 * Provides contract interfaces that are part of the system to
 * provide entropy to the beacon contract.
*/
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WhitelistPublicKeyMsg {
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SubmitEntropyMsg {
    pub proof: Proof,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
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
    pub bounties: Vec<Uint128>,
}
