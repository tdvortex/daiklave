mod create;
mod versions;
use serde::{Serialize, Deserialize};
pub use create::CreateChannel;
pub use versions::{ChannelV0, ChannelVersion, ChannelCurrent};

/// A versioned Channel document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "channel")]
#[serde(rename_all = "camelCase")]
#[serde(tag = "version")]
pub enum ChannelDocument {
    /// Version zero
    V0(ChannelV0),
}

impl From<ChannelCurrent> for ChannelDocument {
    fn from(value: ChannelCurrent) -> Self {
        Self::V0(value)
    }
}

impl From<ChannelDocument> for ChannelCurrent {
    fn from(value: ChannelDocument) -> Self {
        match value {
            ChannelDocument::V0(value) => value,
        }
    }
}