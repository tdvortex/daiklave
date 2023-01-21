mod id;
mod keyword;

use std::collections::HashSet;

pub use id::MartialArtsCharmId;
pub use keyword::MartialArtsCharmKeyword;

use serde::{Serialize, Deserialize};

use crate::{book_reference::BookReference, charms::{CharmCost, CharmActionType}};

use super::MartialArtsStyleId;

/// A Martial Arts charm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MartialArtsCharm {
    style: MartialArtsStyleId,
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    essence_required: u8,
    ability_required: u8,
    charms_required: HashSet<MartialArtsCharmId>,
    keywords: HashSet<MartialArtsCharmKeyword>,
    costs: Vec<CharmCost>,
    action_type: CharmActionType,
    duration: String,
}

impl MartialArtsCharm {
    /// The Martial Arts style associated with the Charm.
    pub fn style(&self) -> MartialArtsStyleId {
        self.style
    }

    /// The book reference for the Charm, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// The name of the Charm.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// A brief summary of the Charm.
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    /// The full Charm description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// The minimum Essence level required to use the Charm.
    pub fn essence_required(&self) -> u8 {
        self.essence_required
    }

    /// The minimum dots in the applicable Martial Arts ability to use the
    /// Charm.
    pub fn ability_required(&self) -> u8 {
        self.ability_required
    }

    /// An iterator of the Martial Arts Charm Ids for any prerequisite Charms.
    pub fn charms_required(&self) -> impl Iterator<Item = MartialArtsCharmId> + '_ {
        self.charms_required.iter().copied()
    }

    /// An iterator of the Charm keywords associated with
    /// this Charm.
    pub fn keywords(&self) -> impl Iterator<Item = MartialArtsCharmKeyword> + '_ {
        self.keywords.iter().copied()
    }

    /// An iterator of the Charm costs (motes, willpower, etc) associated with
    /// this Charm.
    pub fn costs(&self) -> impl Iterator<Item = CharmCost> + '_ {
        self.costs.iter().copied()
    }
}