mod v0;

use serde::{Serialize, Deserialize};
pub use v0::{CampaignV0};

/// The current version of the Campaign document.
pub type CampaignCurrent = CampaignV0;

/// A version tag for the campaign struct to use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CampaignVersion {
    /// Version zero
    V0,
}

