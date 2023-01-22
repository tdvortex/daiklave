use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCostType},
    exaltation::exalt::exalt_type::solar::charm::{
        ability::SolarCharmAbility, SolarCharmId, SolarCharmKeyword,
    },
};

use super::with_duration::SolarCharmBuilderWithDuration;

/// A Solar Charm builder after the action type has been specified.
pub struct SolarCharmBuilderWithActionType {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) charms_required: HashSet<SolarCharmId>,
    pub(crate) keywords: HashSet<SolarCharmKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
    pub(crate) essence_required: NonZeroU8,
    pub(crate) ability: SolarCharmAbility,
    pub(crate) ability_required: u8,
    pub(crate) action_type: CharmActionType,
}

impl SolarCharmBuilderWithActionType {
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
    pub fn charm_prerequisite(mut self, charm_id: SolarCharmId) -> Self {
        self.charms_required.insert(charm_id);
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

    /// Sets the duration of the Charm's effects. Often "Instant" but may
    /// be any defined string, such as "Until the next full moon".
    pub fn duration(self, duration: String) -> SolarCharmBuilderWithDuration {
        SolarCharmBuilderWithDuration {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            charms_required: self.charms_required,
            keywords: self.keywords,
            costs: self.costs,
            essence_required: self.essence_required,
            ability: self.ability,
            ability_required: self.ability_required,
            action_type: self.action_type,
            duration,
        }
    }
}
