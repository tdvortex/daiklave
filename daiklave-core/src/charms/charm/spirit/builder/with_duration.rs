use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{
    book_reference::BookReference,
    charms::{charm::SpiritCharmKeyword, CharmActionType, CharmCostType},
};

use super::SpiritCharmBuilderWithDescription;

/// A Spirit Charm builder after the duration has been specified.
pub struct SpiritCharmBuilderWithDuration {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpiritCharmKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
    pub(crate) essence_required: NonZeroU8,
    pub(crate) action_type: CharmActionType,
    pub(crate) duration: String,
}

impl SpiritCharmBuilderWithDuration {
    /// Sets the book reference for the Evocation.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets a summary for the evocation.
    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Adds a keyword to this Charm.
    pub fn keyword(mut self, keyword: SpiritCharmKeyword) -> Self {
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

    /// Adds the description for this Charm.
    pub fn description(self, description: String) -> SpiritCharmBuilderWithDescription {
        SpiritCharmBuilderWithDescription {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            keywords: self.keywords,
            costs: self.costs,
            essence_required: self.essence_required,
            action_type: self.action_type,
            duration: self.duration,
            description,
        }
    }
}
