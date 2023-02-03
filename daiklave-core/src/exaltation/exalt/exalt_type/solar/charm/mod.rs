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
pub struct SolarCharm<'source> {
    pub(crate) name: &'source str,
    pub(crate) details: &'source SolarCharmDetails,
}

impl<'source> SolarCharm<'source> {
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

    /// The name of the Charm.
    pub fn name(&self) -> &'source str {
        self.name
    }

    /// The book reference of the Charm, if any
    pub fn book_reference(&self) -> Option<BookReference> {
        self.details.book_reference
    }

    /// A short summary of the Charm if provided
    pub fn summary(&self) -> Option<&str> {
        self.details.summary.as_deref()
    }

    /// The full Charm text description
    pub fn description(&self) -> &str {
        &self.details.description
    }

    /// The Essence requirement for the Charm
    pub fn essence_required(&self) -> NonZeroU8 {
        self.details.essence_required
    }

    /// The ability associated with the charm, and its minimum rating
    pub fn ability_requirement(&self) -> (SolarCharmAbility, u8) {
        (self.details.ability, self.details.ability_requirement)
    }

    /// The Ids of Charms which are prerequisites for this Charm
    pub fn charm_prerequisites(&'source self) -> impl Iterator<Item = &'source str> + '_ {
        self.details.charms_required.iter().map(|s| s.as_str())
    }

    /// Any keywords that the Charm has.
    pub fn keywords(&self) -> impl Iterator<Item = SolarCharmKeyword> + '_ {
        let mut list = self.details.keywords.iter().copied().collect::<Vec<_>>();
        list.sort();
        list.into_iter()
    }

    /// The costs to use the Charm.
    pub fn costs(&self) -> impl Iterator<Item = CharmCost> + '_ {
        let mut list = self
            .details
            .costs
            .iter()
            .map(|(cost_type, amount)| CharmCost::new(*cost_type, amount.get()))
            .collect::<Vec<_>>();
        list.sort();
        list.into_iter()
    }

    /// The action required to use the Charm.
    pub fn action_type(&self) -> CharmActionType {
        self.details.action_type
    }

    /// The duration of the Charm's effects.
    pub fn duration(&self) -> &str {
        &self.details.duration
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SolarCharmDetails {
    pub book_reference: Option<BookReference>,
    pub summary: Option<String>,
    pub description: String,
    pub essence_required: NonZeroU8,
    pub ability: SolarCharmAbility,
    pub ability_requirement: u8,
    pub charms_required: HashSet<String>,
    pub keywords: HashSet<SolarCharmKeyword>,
    pub costs: HashMap<CharmCostType, NonZeroU8>,
    pub action_type: CharmActionType,
    pub duration: String,
}
