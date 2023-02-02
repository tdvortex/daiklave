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

/// A Charm associated with a Martial Arts style.
pub struct MartialArtsCharm<'source> {
    pub(crate) name: &'source str,
    pub(crate) style_name: &'source str,
    pub(crate) details: &'source MartialArtsCharmDetails,
}

impl<'source> MartialArtsCharm<'source> {
    /// The name of the Charm.
    pub fn name(&self) -> &'source str {
        self.name
    }

    /// The Martial Arts style associated with the Charm.
    pub fn style(&self) -> &'source str {
        self.style_name
    }

    /// The book reference for the Charm, if any.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.details.book_reference
    }

    /// A brief summary of the Charm.
    pub fn summary(&self) -> Option<&'source str> {
        self.details.summary.as_deref()
    }

    /// The full Charm description.
    pub fn description(&self) -> &'source str {
        &self.details.description
    }

    /// The description of any Mastery effects on the Charm.
    pub fn mastery(&self) -> Option<&'source str> {
        self.details.mastery.as_deref()
    }

    /// The description of any Terrestrial limitations on the Charm.
    pub fn terrestrial(&self) -> Option<&'source str> {
        self.details.terrestrial.as_deref()
    }

    /// The description of any Enlightenment enhancements on the Charm.
    pub fn enlightenment(&self) -> Option<&'source str> {
        self.details.enlightenment.as_deref()
    }

    /// The minimum Essence level required to use the Charm.
    pub fn essence_required(&self) -> u8 {
        self.details.essence_required.get()
    }

    /// The minimum dots in the applicable Martial Arts ability to use the
    /// Charm.
    pub fn ability_required(&self) -> u8 {
        self.details.ability_required.get()
    }

    /// An iterator of the Martial Arts Charm names for any prerequisite Charms.
    pub fn charms_required(&'source self) -> impl Iterator<Item = &'source str> + '_ {
        self.details.charms_required.iter().map(|s| s.as_str())
    }

    /// An iterator of the Charm keywords associated with
    /// this Charm.
    pub fn keywords(&self) -> impl Iterator<Item = MartialArtsCharmKeyword> + '_ {
        self.details.keywords.iter().copied()
    }

    /// An iterator of the Charm costs (motes, willpower, etc) associated with
    /// this Charm.
    pub fn costs(&self) -> impl Iterator<Item = CharmCost> + '_ {
        self.details
            .costs
            .iter()
            .map(|(cost_type, amount)| CharmCost::new(*cost_type, amount.get()))
    }
}



#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MartialArtsCharmDetails {
    pub book_reference: Option<BookReference>,
    pub summary: Option<String>,
    pub description: String,
    pub mastery: Option<String>,
    pub terrestrial: Option<String>,
    pub enlightenment: Option<String>,
    pub essence_required: NonZeroU8,
    pub ability_required: NonZeroU8,
    pub charms_required: HashSet<MartialArtsCharmName>,
    pub keywords: HashSet<MartialArtsCharmKeyword>,
    pub costs: HashMap<CharmCostType, NonZeroU8>,
    pub action_type: CharmActionType,
    pub duration: String,
}