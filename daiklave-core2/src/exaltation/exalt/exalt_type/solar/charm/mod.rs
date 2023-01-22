/// A builder path for constructing a new Solar Charm.
pub mod builder;

mod ability;
mod id;
mod keyword;
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

pub use id::SolarCharmId;
pub use keyword::SolarCharmKeyword;
pub use ability::SolarCharmAbility;

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCostType, CharmCost},
};

use self::{builder::SolarCharmBuilder};

/// A Solar charm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarCharm {
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    essence_required: NonZeroU8,
    ability: SolarCharmAbility,
    ability_requirement: u8,
    charms_required: HashSet<SolarCharmId>,
    keywords: HashSet<SolarCharmKeyword>,
    costs: HashMap<CharmCostType, NonZeroU8>,
    action_type: CharmActionType,
    duration: String,
}

impl SolarCharm {
    /// Starts building a new Solar Charm.
    pub fn builder(name: String) -> SolarCharmBuilder {
        SolarCharmBuilder {
            name,
            book_reference: None,
            summary: None,
            charms_required: HashSet::new(),
            keywords: HashSet::new(),
            costs: HashMap::new(),
        }
    }

    /// The name of the Charm.
    pub fn name(&self) -> &str {
        self.name.as_str()
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
    pub fn charm_prerequisites(&self) -> impl Iterator<Item = SolarCharmId> + '_ {
        self.charms_required.iter().copied()
    }

    /// Any keywords that the Charm has.
    pub fn keywords(&self) -> impl Iterator<Item = SolarCharmKeyword> + '_ {
        let mut list = self.keywords.iter().copied().collect::<Vec<_>>();
        list.sort();
        list.into_iter()
    }

    /// The costs to use the Charm.
    pub fn costs(&self) -> impl Iterator<Item = CharmCost> + '_ {
        let mut list = self.costs.iter().map(|(cost_type, amount)| CharmCost::new(*cost_type, amount.get())).collect::<Vec<_>>();
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
