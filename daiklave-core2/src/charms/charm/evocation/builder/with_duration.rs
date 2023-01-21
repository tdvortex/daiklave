use std::{num::NonZeroU8, collections::{HashSet, HashMap}};

use crate::{charms::{CharmActionType, charm::{evocation::{EvokableId, EvocationId, EvocationKeyword}, CharmId}, CharmCostType}, book_reference::BookReference};

use super::EvocationBuilderWithDescription;

/// An Evocation builder after the Duration has been specified.
pub struct EvocationBuilderWithDuration {
    pub(crate) evokable_id: EvokableId,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) name: String,
    pub(crate) summary: Option<String>,
    pub(crate) essence_required: NonZeroU8,
    pub(crate) resonant: Option<String>,
    pub(crate) dissonant: Option<String>,
    pub(crate) evocation_tree: HashSet<EvocationId>,
    pub(crate) upgrade_charm: Option<CharmId>,
    pub(crate) keywords: HashSet<EvocationKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
    pub(crate) action_type: CharmActionType,
    pub(crate) duration: String,
}

impl EvocationBuilderWithDuration {
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

    /// Adds a description which applies if the Exalt is resonant with the 
    /// magic material of the artifact.
    pub fn resonant(mut self, description: String) -> Self {
        self.resonant = Some(description);
        self
    }

    /// Adds a description which applies if the Exalt is dissonant with the 
    /// magic material of the artifact.
    pub fn dissonant(mut self, description: String) -> Self {
        self.dissonant = Some(description);
        self
    }

    /// Adds a charm tree prerequisite on other Evocations.
    pub fn evocation_prerequisite(mut self, evocation_id: EvocationId) -> Self {
        self.evocation_tree.insert(evocation_id);
        self
    }

    /// Sets this Evocation as an upgrade of another Charm, usually a 
    /// Solar Charm (or other Exalt-specific type).
    pub fn upgrades(mut self, charm_id: CharmId) -> Self {
        self.upgrade_charm = Some(charm_id);
        self
    }

    /// Adds a cost to use this Charm.
    pub fn cost(mut self, cost_type: CharmCostType, amount: NonZeroU8) -> Self {
        self.costs.entry(cost_type).and_modify(|prior| {
            *prior = (*prior).saturating_add(amount.get());
        }).or_insert(amount);

        self
    }

    /// Adds a keyword to this Charm.
    pub fn keyword(mut self, keyword: EvocationKeyword) -> Self {
        self.keywords.insert(keyword);
        self
    }

    /// Sets the description for the Evocation. This should **not** include any
    /// resonant or dissonant effects.
    pub fn description(self, description: String) -> EvocationBuilderWithDescription {
        EvocationBuilderWithDescription {
            evokable_id: self.evokable_id,
            book_reference: self.book_reference,
            name: self.name,
            summary: self.summary,
            essence_required: self.essence_required,
            resonant: self.resonant,
            dissonant: self.dissonant,
            evocation_tree: self.evocation_tree,
            upgrade_charm: self.upgrade_charm,
            keywords: self.keywords,
            costs: self.costs,
            action_type: self.action_type,
            duration: self.duration,
            description,
        }
    }
}