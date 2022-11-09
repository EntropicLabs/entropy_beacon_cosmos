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
    pub request_ids: Vec<u128>,
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

#[derive(Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ActiveRequestsQuery {
    pub start_after: Option<u128>,
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
    pub whitelist_deposit_amt: Uint128,
    pub key_activation_delay: u64,
    pub protocol_fee: u64,
    pub submitter_share: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ActiveRequestInfo {
    ///The ID of the request
    pub id: u128,
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
mod serialization {
    use cosmwasm_std::{Addr, Uint128};
    use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

    use super::{ActiveRequestsQuery, ActiveRequestInfo};

    impl Serialize for ActiveRequestsQuery {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("ActiveRequestsQuery", 2)?;
            match &self.start_after {
                Some(n) => state.serialize_field("start_after", &n.to_string())?,
                None => state.serialize_field("start_after", &None::<String>)?,
            };
            state.serialize_field("limit", &self.limit)?;
            state.end()
        }
    }

    impl<'de> Deserialize<'de> for ActiveRequestsQuery {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Deserialize)]
            #[serde(rename_all = "snake_case")]
            struct ActiveRequestsQueryRaw {
                start_after: Option<String>,
                limit: Option<u32>,
            }

            let raw = ActiveRequestsQueryRaw::deserialize(deserializer)?;
            Ok(ActiveRequestsQuery {
                start_after: raw.start_after.map(|n| n.parse().unwrap()),
                limit: raw.limit,
            })
        }
    }

    impl Serialize for ActiveRequestInfo {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("ActiveRequestInfo", 6)?;
            state.serialize_field("id", &self.id.to_string())?;
            state.serialize_field("callback_gas_limit", &self.callback_gas_limit)?;
            state.serialize_field("callback_address", &self.callback_address)?;
            state.serialize_field("submitter", &self.submitter)?;
            state.serialize_field("submitted_block_height", &self.submitted_block_height)?;
            state.serialize_field("submitted_bounty_amount", &self.submitted_bounty_amount)?;
            state.end()
        }
    }

    impl<'de> Deserialize<'de> for ActiveRequestInfo {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Deserialize)]
            #[serde(rename_all = "snake_case")]
            struct ActiveRequestInfoRaw {
                id: String,
                callback_gas_limit: u64,
                callback_address: Addr,
                submitter: Addr,
                submitted_block_height: u64,
                submitted_bounty_amount: Uint128,
            }

            let raw = ActiveRequestInfoRaw::deserialize(deserializer)?;
            Ok(ActiveRequestInfo {
                id: raw.id.parse().unwrap(),
                callback_gas_limit: raw.callback_gas_limit,
                callback_address: raw.callback_address,
                submitter: raw.submitter,
                submitted_block_height: raw.submitted_block_height,
                submitted_bounty_amount: raw.submitted_bounty_amount,
            })
        }
    }
}