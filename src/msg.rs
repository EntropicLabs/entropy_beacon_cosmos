use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    beacon::RequestEntropyMsg,
    provide::{KeyStatusQuery, LastEntropyQuery, SubmitEntropyMsg, WhitelistPublicKeyMsg, ActiveRequestsQuery},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    WhitelistPublicKey(WhitelistPublicKeyMsg),
    SubmitEntropy(SubmitEntropyMsg),
    RequestEntropy(RequestEntropyMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    KeyStatus(KeyStatusQuery),
    LastEntropy(LastEntropyQuery),
    ActiveRequests(ActiveRequestsQuery),
}
