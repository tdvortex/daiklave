use serde::{Serialize, Deserialize};

/// Indicates whether motes are spent/committed from peripheral or peripheral
/// pool first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MotePoolName {
    /// Spend/commit peripheral motes first
    Peripheral,
    /// Spend/commit personal motes first
    Personal,
}