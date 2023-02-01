mod add;
pub use add::AddSpiritOnlyCharm;

use super::inner::SpiritCharmInner;

/// A Spirit Charm not usable by an Eclipse Caste Solar.
pub struct SpiritOnlyCharm(pub(crate) SpiritCharmInner);
