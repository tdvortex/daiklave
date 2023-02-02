mod with_ability_requirement;
mod with_action_type;
mod with_description;
mod with_duration;
mod with_essence_requirement;
mod with_name;
pub use with_ability_requirement::MartialArtsCharmBuilderWithAbilityRequirement;
pub use with_action_type::MartialArtsCharmBuilderWithActionType;
pub use with_description::MartialArtsCharmBuilderWithDescription;
pub use with_duration::MartialArtsCharmBuilderWithDuration;
pub use with_essence_requirement::MartialArtsCharmBuilderWithEssenceRequirement;
pub use with_name::MartialArtsCharmBuilderWithName;

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{
    book_reference::BookReference, charms::CharmCostType, martial_arts::style::MartialArtsStyleName,
};

use super::{MartialArtsCharmKeyword, MartialArtsCharmName};

/// A builder to construct a new Martial Arts charm.
pub struct MartialArtsCharmBuilder {
    pub(crate) style: MartialArtsStyleName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) charms_required: HashSet<MartialArtsCharmName>,
    pub(crate) mastery: Option<String>,
    pub(crate) terrestrial: Option<String>,
    pub(crate) enlightenment: Option<String>,
    pub(crate) keywords: HashSet<MartialArtsCharmKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
    pub(crate) summary: Option<String>,
}

impl MartialArtsCharmBuilder {
    /// Starts a new builder by specifying the Martial Arts style it belongs to.
    pub fn style(style_name: impl Into<MartialArtsStyleName>) -> Self {
        Self {
            style: style_name.into(),
            book_reference: Default::default(),
            charms_required: Default::default(),
            mastery: Default::default(),
            terrestrial: Default::default(),
            enlightenment: Default::default(),
            keywords: Default::default(),
            costs: Default::default(),
            summary: Default::default(),
        }
    }

    /// Sets the book reference for the Charm.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Sets a summary for the Charm.
    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    /// Adds another Martial Arts charm as a prerequisite.
    pub fn charm_prerequisite(mut self, charm_name: impl Into<MartialArtsCharmName>) -> Self {
        self.charms_required.insert(charm_name.into());
        self
    }

    /// Adds a description of the Mastery effect for this Charm.
    pub fn mastery(mut self, description: String) -> Self {
        self.mastery = Some(description);
        self
    }

    /// Adds a description of the Terrestrial effect for this Charm.
    pub fn terrestrial(mut self, description: String) -> Self {
        self.terrestrial = Some(description);
        self
    }

    /// Adds a description of the Enlightenment effect for this Charm.
    pub fn enlightenment(mut self, description: String) -> Self {
        self.enlightenment = Some(description);
        self
    }

    /// Adds a Charm keyword.
    pub fn keyword(mut self, keyword: MartialArtsCharmKeyword) -> Self {
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

    /// Sets the name of the Charm.
    pub fn name(self, name: impl Into<MartialArtsCharmName>) -> MartialArtsCharmBuilderWithName {
        MartialArtsCharmBuilderWithName {
            name: name.into(),
            style: self.style,
            book_reference: self.book_reference,
            charms_required: self.charms_required,
            mastery: self.mastery,
            terrestrial: self.terrestrial,
            enlightenment: self.enlightenment,
            keywords: self.keywords,
            costs: self.costs,
            summary: self.summary,
        }
    }
}
