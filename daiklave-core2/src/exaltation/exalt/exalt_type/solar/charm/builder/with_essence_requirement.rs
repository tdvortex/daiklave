use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{
    book_reference::BookReference,
    charms::CharmCostType,
    exaltation::exalt::exalt_type::solar::charm::{ability::SolarCharmAbility, SolarCharmKeyword},
};

use super::with_ability_requirement::SolarCharmBuilderWithAbilityRequirement;

/// A Solar Charm builder after the Essence requirement has been specified.
pub struct SolarCharmBuilderWithEssenceRequirement {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) charms_required: HashSet<String>,
    pub(crate) keywords: HashSet<SolarCharmKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
    pub(crate) essence_required: NonZeroU8,
}

impl SolarCharmBuilderWithEssenceRequirement {
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

    /// Sets an ability requirement for using this Charm. Maxes out at 5 dots.
    /// Note that this *can* be 0 dots; this is used for Integrity Bridge Charms,
    /// as they are categorized as Integrity charms despite requiring no
    /// Integrity dots in some cases.
    pub fn ability_required(
        self,
        ability: SolarCharmAbility,
        rating: u8,
    ) -> SolarCharmBuilderWithAbilityRequirement {
        SolarCharmBuilderWithAbilityRequirement {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            charms_required: self.charms_required,
            keywords: self.keywords,
            costs: self.costs,
            essence_required: self.essence_required,
            ability,
            ability_required: rating,
        }
    }
}
