use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCostType},
    martial_arts::{
        charm::{MartialArtsCharmKeyword, MartialArtsCharmName},
        style::MartialArtsStyleName,
    },
};

use super::MartialArtsCharmBuilderWithDescription;

/// A Martial Arts Charm builder after the effect duration has been specified.
pub struct MartialArtsCharmBuilderWithDuration {
    pub(crate) name: MartialArtsCharmName,
    pub(crate) style: MartialArtsStyleName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) charms_required: HashSet<MartialArtsCharmName>,
    pub(crate) mastery: Option<String>,
    pub(crate) terrestrial: Option<String>,
    pub(crate) enlightenment: Option<String>,
    pub(crate) keywords: HashSet<MartialArtsCharmKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
    pub(crate) summary: Option<String>,
    pub(crate) essence_required: NonZeroU8,
    pub(crate) ability_required: NonZeroU8,
    pub(crate) action_type: CharmActionType,
    pub(crate) duration: String,
}

impl MartialArtsCharmBuilderWithDuration {
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

    /// Sets the description for the Evocation. This should **not** include any
    /// Mastery, Terrestrial, or Enlightenment variants.
    pub fn description(self, description: String) -> MartialArtsCharmBuilderWithDescription {
        MartialArtsCharmBuilderWithDescription {
            name: self.name,
            style: self.style,
            book_reference: self.book_reference,
            charms_required: self.charms_required,
            mastery: self.mastery,
            terrestrial: self.terrestrial,
            enlightenment: self.enlightenment,
            keywords: self.keywords,
            costs: self.costs,
            summary: self.summary,
            essence_required: self.essence_required,
            ability_required: self.ability_required,
            action_type: self.action_type,
            duration: self.duration,
            description,
        }
    }
}
