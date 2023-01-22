/// A builder path for constructing a Martial Arts charm.
pub mod builder;
mod id;
mod keyword;

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

pub use id::MartialArtsCharmId;
pub use keyword::MartialArtsCharmKeyword;

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost, CharmCostType},
};

use self::builder::MartialArtsCharmBuilder;

use super::MartialArtsStyleId;

/// A Martial Arts charm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MartialArtsCharm {
    style: MartialArtsStyleId,
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    mastery: Option<String>,
    terrestrial: Option<String>,
    essence_required: NonZeroU8,
    ability_required: NonZeroU8,
    charms_required: HashSet<MartialArtsCharmId>,
    keywords: HashSet<MartialArtsCharmKeyword>,
    costs: HashMap<CharmCostType, NonZeroU8>,
    action_type: CharmActionType,
    duration: String,
}

impl MartialArtsCharm {
    /// Starts a builder to create a new Martial Arts Charm.
    pub fn builder(name: String, style: MartialArtsStyleId) -> MartialArtsCharmBuilder {
        MartialArtsCharmBuilder {
            name,
            style,
            book_reference: None,
            charms_required: HashSet::new(),
            mastery: None,
            terrestrial: None,
            enlightenment: None,
            keywords: HashSet::new(),
            costs: HashMap::new(),
            summary: None,
        }
    }

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
        self.essence_required.get()
    }

    /// The minimum dots in the applicable Martial Arts ability to use the
    /// Charm.
    pub fn ability_required(&self) -> u8 {
        self.ability_required.get()
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
        self.costs
            .iter()
            .map(|(cost_type, amount)| CharmCost::new(*cost_type, amount.get()))
    }
}
