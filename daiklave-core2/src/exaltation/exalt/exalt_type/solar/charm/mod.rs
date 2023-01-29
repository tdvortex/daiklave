/// A builder path for constructing a new Solar Charm.
pub mod builder;

mod ability;
mod add;
mod keyword;
mod name;
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

pub use ability::SolarCharmAbility;
pub use add::AddSolarCharm;
pub use keyword::SolarCharmKeyword;
pub use name::SolarCharmName;

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost, CharmCostType},
};

use self::builder::SolarCharmBuilder;

/// A Solar charm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarCharm {
    book_reference: Option<BookReference>,
    summary: Option<String>,
    description: String,
    essence_required: NonZeroU8,
    ability: SolarCharmAbility,
    ability_requirement: u8,
    charms_required: HashSet<String>,
    keywords: HashSet<SolarCharmKeyword>,
    costs: HashMap<CharmCostType, NonZeroU8>,
    action_type: CharmActionType,
    duration: String,
}

impl<'source> SolarCharm {
    /// Starts building a new Solar Charm.
    pub fn builder(name: impl Into<SolarCharmName>) -> SolarCharmBuilder {
        SolarCharmBuilder {
            name: name.into(),
            book_reference: None,
            summary: None,
            charms_required: HashSet::new(),
            keywords: HashSet::new(),
            costs: HashMap::new(),
        }
    }

    /// The book reference of the Charm, if any
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// A short summary of the Charm if provided
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    /// The full Charm text description
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// The Essence requirement for the Charm
    pub fn essence_required(&self) -> NonZeroU8 {
        self.essence_required
    }

    /// The ability associated with the charm, and its minimum rating
    pub fn ability_requirement(&self) -> (SolarCharmAbility, u8) {
        (self.ability, self.ability_requirement)
    }

    /// The Ids of Charms which are prerequisites for this Charm
    pub fn charm_prerequisites(&'source self) -> impl Iterator<Item = &'source str> + '_ {
        self.charms_required.iter().map(|s| s.as_str())
    }

    /// Any keywords that the Charm has.
    pub fn keywords(&self) -> impl Iterator<Item = SolarCharmKeyword> + '_ {
        let mut list = self.keywords.iter().copied().collect::<Vec<_>>();
        list.sort();
        list.into_iter()
    }

    /// The costs to use the Charm.
    pub fn costs(&self) -> impl Iterator<Item = CharmCost> + '_ {
        let mut list = self
            .costs
            .iter()
            .map(|(cost_type, amount)| CharmCost::new(*cost_type, amount.get()))
            .collect::<Vec<_>>();
        list.sort();
        list.into_iter()
    }

    /// The action required to use the Charm.
    pub fn action_type(&self) -> CharmActionType {
        self.action_type
    }

    /// The duration of the Charm's effects.
    pub fn duration(&self) -> &str {
        self.duration.as_str()
    }
}
