mod new;
mod versions;
pub use new::NewCampaign;
use serde::{Serialize, Deserialize};
pub use versions::{CampaignCurrent, CampaignV0};

/// A versioned Campaign document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "campaign")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "version")]
pub enum CampaignDocument {
    /// Version zero.
    V0(CampaignV0),
}

impl From<CampaignCurrent> for CampaignDocument {
    fn from(value: CampaignCurrent) -> Self {
        Self::V0(value)
    }
}

impl From<CampaignDocument> for CampaignCurrent {
    fn from(value: CampaignDocument) -> Self {
        match value {
            CampaignDocument::V0(value) => value,
        }
    }
}