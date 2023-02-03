mod add;
pub use add::AddEclipseCharm;

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost},
};

use super::{inner::SpiritCharmInner, SpiritCharmKeyword};

/// A Spirit charm with the Eclipse keyword that may be purchased by an Eclipse
/// caste Solar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EclipseCharm(pub(crate) SpiritCharmInner);

impl EclipseCharm {
    /// The book reference for the Eclipse Charm.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference
    }

    /// A short summary of the Charm's effect.
    pub fn summary(&self) -> Option<&str> {
        self.0.summary.as_deref()
    }

    /// The full description of the Charm's effect.
    pub fn description(&self) -> &str {
        self.0.description.as_str()
    }

    /// The Essence required to learn the Charm.
    pub fn essence_required(&self) -> u8 {
        self.0.essence_required.get()
    }

    /// The keywords of the Charm.
    pub fn keywords(&self) -> impl Iterator<Item = SpiritCharmKeyword> + '_ {
        self.0.keywords()
    }

    /// The activation costs for using the Charm.
    pub fn costs(&self) -> impl Iterator<Item = CharmCost> + '_ {
        self.0.costs()
    }

    /// The action required to activate the Charm.
    pub fn action_type(&self) -> CharmActionType {
        self.0.action_type
    }

    /// How long the Charm's effects last.
    pub fn duration(&self) -> &str {
        self.0.duration.as_str()
    }
}
