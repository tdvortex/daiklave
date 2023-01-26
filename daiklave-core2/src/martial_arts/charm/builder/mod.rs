mod with_ability_requirement;
mod with_action_type;
mod with_description;
mod with_duration;
mod with_essence_requirement;
pub use with_ability_requirement::MartialArtsCharmBuilderWithAbilityRequirement;
pub use with_action_type::MartialArtsCharmBuilderWithActionType;
pub use with_description::MartialArtsCharmBuilderWithDescription;
pub use with_duration::MartialArtsCharmBuilderWithDuration;
pub use with_essence_requirement::MartialArtsCharmBuilderWithEssenceRequirement;

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use crate::{book_reference::BookReference, charms::CharmCostType};

use super::{MartialArtsCharmId, MartialArtsCharmKeyword};

/// A builder path to construct a Martial Arts charm. Required fields, in
/// order, are: name(already specified), style name (already specified), Essence
/// requirement, ability dots requirement, action type, duration, and
/// description. Optional fields: book reference, prerequisite Martial Arts
/// Charms, a Mastery effect, a Terrestrial effect, an Enlightenment effect,
/// Charm keywords, costs, and a short summary.
pub struct MartialArtsCharmBuilder {
    pub(crate) name: String,
    pub(crate) style: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) charms_required: HashSet<MartialArtsCharmId>,
    pub(crate) mastery: Option<String>,
    pub(crate) terrestrial: Option<String>,
    pub(crate) enlightenment: Option<String>,
    pub(crate) keywords: HashSet<MartialArtsCharmKeyword>,
    pub(crate) costs: HashMap<CharmCostType, NonZeroU8>,
    pub(crate) summary: Option<String>,
}

impl MartialArtsCharmBuilder {
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
    pub fn charm_prerequisite(mut self, charm_id: MartialArtsCharmId) -> Self {
        self.charms_required.insert(charm_id);
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

    /// Sets an essence requirement for using this Charm. Maxes out at 5 dots.
    pub fn essence_required(
        self,
        essence_required: NonZeroU8,
    ) -> MartialArtsCharmBuilderWithEssenceRequirement {
        MartialArtsCharmBuilderWithEssenceRequirement {
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
            essence_required,
        }
    }
}
