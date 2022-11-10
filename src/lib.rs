/// Functions for interacting with the Beacon as a consumer of entropy.
pub mod beacon;
/// Structs describing the ExecuteMsg, QueryMsg, and InstantiateMsg of the Beacon.
pub mod msg;
/// Convinience structs for representing Entropy proofs with and without the [ecvrf] feature.
pub mod proof;
/// Functions for interacting with the Beacon as a provider of entropy.
pub mod provide;
/// Re-export of common types for entropy consumers.
pub use beacon::{EntropyCallbackMsg, EntropyRequest, CalculateFeeQuery};
pub use msg::QueryMsg as BeaconQueryMsg;
/// Re-export of common types for entropy providers.
pub use provide::{BeaconConfigQuery, BeaconConfigResponse};