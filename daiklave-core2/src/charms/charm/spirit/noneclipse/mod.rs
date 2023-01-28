mod add;
pub use add::AddNonEclipseCharm;

use super::inner::SpiritCharmInner;

/// A Spirit Charm not usable by an Eclipse Caste Solar.
pub struct NonEclipseCharm(pub(crate) SpiritCharmInner);
