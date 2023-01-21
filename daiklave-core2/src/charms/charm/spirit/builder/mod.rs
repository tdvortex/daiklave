mod with_action_type;
mod with_description;
mod with_duration;
mod with_essence_requirement;
pub use with_action_type::SpiritCharmBuilderWithActionType;
pub use with_description::SpiritCharmBuilderWithDescription;
pub use with_duration::SpiritCharmBuilderWithDuration;
pub use with_essence_requirement::SpiritCharmBuilderWithEssenceRequirement;

use std::{collections::{HashSet, HashMap}, num::NonZeroU8};

use crate::{book_reference::BookReference, charms::{CharmCostType}};

use super::SpiritCharmKeyword;

/// A builder for a Spirit Charm. Required fields, in order, are:
/// name (already specified), Essence requirement, action type, duration,
/// description, and finally if it is an Eclipse charm. Optional fields: 
/// book reference, charm keywords, charm costs, and a short summary.
pub struct SpiritCharmBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) keywords: HashSet<SpiritCharmKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>
}

impl SpiritCharmBuilder {
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
        self.costs.entry(cost_type).and_modify(|prior| {
            *prior = (*prior).saturating_add(amount.get());
        }).or_insert(amount);

        self
    }

    /// Sets an essence requirement for using this Charm. Maxes out at 5 dots.
    pub fn essence_required(self, rating: NonZeroU8) -> SpiritCharmBuilderWithEssenceRequirement {
        SpiritCharmBuilderWithEssenceRequirement {
            name: self.name,
            book_reference: self.book_reference,
            summary: self.summary,
            keywords: self.keywords,
            costs: self.costs,
            essence_required: rating,
        }
    }
}