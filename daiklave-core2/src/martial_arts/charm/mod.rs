mod add;
/// A builder path for constructing a Martial Arts charm.
pub mod builder;
mod keyword;
mod name;

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

pub use add::AddMartialArtsCharm;
pub use keyword::MartialArtsCharmKeyword;
pub use name::MartialArtsCharmName;

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost, CharmCostType},
};

use self::builder::MartialArtsCharmBuilder;

/// A Martial Arts charm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MartialArtsCharm {
    style: String,
    book_reference: Option<BookReference>,
    summary: Option<String>,
    description: String,
    mastery: Option<String>,
    terrestrial: Option<String>,
    enlightenment: Option<String>,
    essence_required: NonZeroU8,
    ability_required: NonZeroU8,
    charms_required: HashSet<String>,
    keywords: HashSet<MartialArtsCharmKeyword>,
    costs: HashMap<CharmCostType, NonZeroU8>,
    action_type: CharmActionType,
    duration: String,
}

impl<'source> MartialArtsCharm {
    /// Starts a builder to create a new Martial Arts Charm.
    pub fn builder(charm_name: String, style_name: String) -> MartialArtsCharmBuilder {
        MartialArtsCharmBuilder {
            name: charm_name,
            style: style_name,
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
    pub fn style(&self) -> &str {
        self.style.as_str()
    }

    /// The book reference for the Charm, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// A brief summary of the Charm.
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    /// The full Charm description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// The description of any Mastery effects on the Charm.
    pub fn mastery(&self) -> Option<&str> {
        self.mastery.as_deref()
    }

    /// The description of any Terrestrial limitations on the Charm.
    pub fn terrestrial(&self) -> Option<&str> {
        self.terrestrial.as_deref()
    }

    /// The description of any Enlightenment enhancements on the Charm.
    pub fn enlightenment(&self) -> Option<&str> {
        self.enlightenment.as_deref()
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

    /// An iterator of the Martial Arts Charm names for any prerequisite Charms.
    pub fn charms_required(&'source self) -> impl Iterator<Item = &'source str> + '_ {
        self.charms_required.iter().map(|s| s.as_str())
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
