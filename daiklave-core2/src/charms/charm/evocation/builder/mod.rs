use std::{collections::{HashSet, HashMap}, num::NonZeroU8};

use crate::{book_reference::BookReference, charms::{charm::CharmId, CharmCostType}};

use super::{EvokableId, EvocationId, EvocationKeyword};

mod with_essence_requirement;
mod with_action_type;
mod with_duration;
mod with_description;
pub use with_action_type::EvocationBuilderWithActionType;
pub use with_essence_requirement::EvocationBuilderWithEssenceRequirement;
pub use with_duration::EvocationBuilderWithDuration;
pub use with_description::EvocationBuilderWithDescription;

/// A builder for an Evocation. Required fields (in order): name (already 
/// specified), evokable item (already specified), essence requirement,
/// action type, duration, and description. Optional fields: book reference,
/// other evocations as prerequisites, a charm which it upgrades, resonant
/// effect, dissonant effect, charm keywords, charm costs, and a short summary.
pub struct EvocationBuilder {
    pub(crate) evokable_id: EvokableId,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) name: String,
    pub(crate) summary: Option<String>,
    pub(crate) resonant: Option<String>,
    pub(crate) dissonant: Option<String>,
    pub(crate) evocation_tree: HashSet<EvocationId>,
    pub(crate) upgrade_charm: Option<CharmId>,
    pub(crate) keywords: HashSet<EvocationKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
}

impl EvocationBuilder {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn resonant(mut self, description: String) -> Self {
        self.resonant = Some(description);
        self
    }

    pub fn dissonant(mut self, description: String) -> Self {
        self.dissonant = Some(description);
        self
    }

    pub fn evocation_prerequisite(mut self, evocation_id: EvocationId) -> Self {
        self.evocation_tree.insert(evocation_id);
        self
    }

    pub fn upgrades(mut self, charm_id: CharmId) -> Self {
        self.upgrade_charm = Some(charm_id);
        self
    }

    pub fn cost(mut self, cost_type: CharmCostType, amount: NonZeroU8) -> Self {
        self.costs.entry(cost_type).and_modify(|prior| {
            *prior = (*prior).saturating_add(amount.get());
        }).or_insert(amount);

        self
    }

    pub fn keyword(mut self, keyword: EvocationKeyword) -> Self {
        self.keywords.insert(keyword);
        self
    }

    pub fn essence_required(self, essence_required: NonZeroU8) -> EvocationBuilderWithEssenceRequirement {
        EvocationBuilderWithEssenceRequirement {
            evokable_id: self.evokable_id,
            book_reference: self.book_reference,
            name: self.name,
            summary: self.summary,
            essence_required,
            resonant: self.resonant,
            dissonant: self.dissonant,
            evocation_tree: self.evocation_tree,
            upgrade_charm: self.upgrade_charm,
            keywords: self.keywords,
            costs: self.costs
        }
    }
}