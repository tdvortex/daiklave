use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCostType},
    martial_arts::{charm::{AddMartialArtsCharm, MartialArtsCharmDetails, MartialArtsCharmKeyword, MartialArtsCharmName}, style::MartialArtsStyleName},
};

/// A Martial Arts Charm builder after the description has been provided. To
/// complete the builder, call build().
pub struct MartialArtsCharmBuilderWithDescription {
    pub(crate) name: MartialArtsCharmName,
    pub(crate) style: MartialArtsStyleName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) charms_required: HashSet<String>,
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
    pub(crate) description: String,
}

impl MartialArtsCharmBuilderWithDescription {
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
    pub fn charm_prerequisite(mut self, charm_name: String) -> Self {
        self.charms_required.insert(charm_name);
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

    /// Completes the builder, returning a Martial Arts Charm.
    pub fn build(self) -> AddMartialArtsCharm {
        AddMartialArtsCharm {
            name: self.name,
            charm: MartialArtsCharmDetails {
                style: self.style,
                book_reference: self.book_reference,
                summary: self.summary,
                description: self.description,
                mastery: self.mastery,
                terrestrial: self.terrestrial,
                enlightenment: self.enlightenment,
                essence_required: self.essence_required,
                ability_required: self.ability_required,
                charms_required: self.charms_required,
                keywords: self.keywords,
                costs: self.costs,
                action_type: self.action_type,
                duration: self.duration,
            },
        }
    }
}
