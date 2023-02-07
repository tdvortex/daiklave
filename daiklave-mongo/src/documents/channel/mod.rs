mod new;
mod versions;
use serde::{Serialize, Deserialize};
pub use new::NewChannel;
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