mod v0;
use serde::{Serialize, Deserialize};
pub use v0::ChannelV0;

/// The current version of the Channel document.
pub type ChannelCurrent = ChannelV0;

/// A version tag for the channel struct to use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChannelVersion {
    /// Version zero
    V0,
}