#[allow(unused_imports)]
use schemars::JsonSchema;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[cfg(feature = "ecvrf")]
pub use ecvrf_rs::{Proof, PublicKey};

/// A PublicKey is a 64 character representation of a 32 byte key in hexadecimal.
#[cfg(not(feature = "ecvrf"))]
pub type PublicKey = String;

/// A Proof contains the PublicKey of the signer, the message used to generate the proof, and the proof itself.
#[cfg(not(feature = "ecvrf"))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Proof {
    pub signer: PublicKey,
    pub message: String,
    pub proof: String,
}
