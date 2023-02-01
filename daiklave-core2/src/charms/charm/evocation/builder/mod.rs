use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{
    book_reference::BookReference,
    charms::{charm::{CharmNameMutation, CharmName}, CharmCostType},
};

use super::{EvocationKeyword, EvokableNameMutation, EvocationName, EvokableName};

mod with_action_type;
mod with_description;
mod with_duration;
mod with_essence_requirement;
mod with_name;
pub use with_action_type::EvocationBuilderWithActionType;
pub use with_description::EvocationBuilderWithDescription;
pub use with_duration::EvocationBuilderWithDuration;
pub use with_name::EvocationBuilderWithName;
pub use with_essence_requirement::EvocationBuilderWithEssenceRequirement;

/// A builder to construct a new Evocation charm.
pub struct EvocationBuilder {
    pub(crate) evokable_name: EvokableNameMutation,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) summary: Option<String>,
    pub(crate) resonant: Option<String>,
    pub(crate) dissonant: Option<String>,
    pub(crate) evocation_tree: HashSet<EvocationName>,
    pub(crate) upgrade_charm: Option<CharmNameMutation>,
    pub(crate) keywords: HashSet<EvocationKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
}

impl EvocationBuilder {
    /// Starts the builder to create an evocation for a specific item capable
    /// of having evocations, either an artifact or hearthstone.
    pub fn evocation_of(name: EvokableName<'_>) -> Self {
        Self {
            evokable_name: name.into(),
            book_reference: Default::default(),
            summary: Default::default(),
            resonant: Default::default(),
            dissonant: Default::default(),
            evocation_tree: Default::default(),
            upgrade_charm: Default::default(),
            keywords: Default::default(),
            costs: Default::default(),
        }
    }

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
    pub fn evocation_prerequisite(mut self, prerequisite_name: impl Into<EvocationName>) -> Self {
        self.evocation_tree.insert(prerequisite_name.into());
        self
    }

    /// Sets this Evocation as an upgrade of another Charm, usually a
    /// Solar Charm (or other Exalt-specific type).
    pub fn upgrades(mut self, charm_name: CharmName<'_>) -> Self {
        self.upgrade_charm = Some(charm_name.into());
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

    /// Adds a keyword to this Charm.
    pub fn keyword(mut self, keyword: EvocationKeyword) -> Self {
        self.keywords.insert(keyword);
        self
    }

    /// Specifies the name for this Evocation.
    pub fn name(
        self,
        name: impl Into<EvocationName>,
    ) -> EvocationBuilderWithName {
        EvocationBuilderWithName {
            evokable_name: self.evokable_name,
            book_reference: self.book_reference,
            name: name.into(),
            summary: self.summary,
            resonant: self.resonant,
            dissonant: self.dissonant,
            evocation_tree: self.evocation_tree,
            upgrade_charm: self.upgrade_charm,
            keywords: self.keywords,
            costs: self.costs,
        }
    }
}

impl<'a, T> From<T> for EvocationBuilder where T: Into<EvokableName<'a>> {
    fn from(name: T) -> Self {
        Self::evocation_of(name.into())
    }
}