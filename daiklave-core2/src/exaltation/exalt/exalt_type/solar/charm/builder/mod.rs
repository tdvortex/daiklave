mod with_ability_requirement;
mod with_action_type;
mod with_description;
mod with_duration;
mod with_essence_requirement;
pub use with_essence_requirement::SolarCharmBuilderWithEssenceRequirement;

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{book_reference::BookReference, charms::CharmCostType};

use super::SolarCharmKeyword;

/// A builder to construct a new Solar Charm. Required fields are name (already
/// specified), Essence requirement, ability category and required dots,
pub struct SolarCharmBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) charms_required: HashSet<String>,
    pub(crate) keywords: HashSet<SolarCharmKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
}

impl SolarCharmBuilder {
    /// Sets the book reference for the Charm.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets a summary for the evocation.
    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Adds a charm tree prerequisite on other Solar Charms.
    pub fn charm_prerequisite(mut self, prerequisite_name: String) -> Self {
        self.charms_required.insert(prerequisite_name);
        self
    }

    /// Adds a keyword to this Charm.
    pub fn keyword(mut self, keyword: SolarCharmKeyword) -> Self {
        self.keywords.insert(keyword);
        self
    }

    /// Adds a cost to use this Charm.
    pub fn cost(mut self, cost_type: CharmCostType, amount: NonZeroU8) -> Self {
        self.costs
            .entry(cost_type)
            .and_modify(|prior| {
                *prior = (*prior).saturating_add(amount.get());
            })
            .or_insert(amount);

        self
    }

    /// Sets an essence requirement for using this Charm. Maxes out at 5 dots.
    pub fn essence_required(
        self,
        essence_required: NonZeroU8,
    ) -> SolarCharmBuilderWithEssenceRequirement {
        SolarCharmBuilderWithEssenceRequirement {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            charms_required: self.charms_required,
            keywords: self.keywords,
            costs: self.costs,
            essence_required: essence_required
                .clamp(NonZeroU8::new(1).unwrap(), NonZeroU8::new(5).unwrap()),
        }
    }
}
