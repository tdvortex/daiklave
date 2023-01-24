use std::num::NonZeroU8;

use crate::{
    abilities::AbilityName,
    attributes::AttributeName,
    book_reference::BookReference,
    merits::merit::{prerequisite::MeritPrerequisite, MeritType},
};

use super::{FixedMeritTemplateBuilder, VariableMeritTemplateBuilder};

/// A merit builder after it has been clarified as Innate, Purchased, Story, or
/// Supernatural.
pub struct MeritTemplateBuilderWithType {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_type: MeritType,
    pub(crate) prerequisites: Vec<MeritPrerequisite>,
}

impl MeritTemplateBuilderWithType {
    /// Sets the book reference for the merit.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Adds a requirement that a certain ability is rated at or above a
    /// dot threshold. If a merit has any prerequisites, they are treated as an
    /// "or" relationship--the merit can be added as long as any prerequisite
    /// is satisfied.
    pub fn ability_prerequisite(mut self, ability_name: AbilityName, dots: NonZeroU8) -> Self {
        let upsert = MeritPrerequisite::Ability(ability_name, dots);
        if let Some(existing) = self.prerequisites.iter_mut().find(|prereq| {
            if let MeritPrerequisite::Ability(existing_name, _) = prereq {
                existing_name == &ability_name
            } else {
                false
            }
        }) {
            *existing = upsert;
        } else {
            self.prerequisites.push(upsert);
        }
        self
    }

    /// Adds a requirement that a certain attribute is rated at or above a
    /// dot threshold. If a merit has any prerequisites, they are treated as an
    /// "or" relationship--the merit can be added as long as any prerequisite
    /// is satisfied.
    pub fn attribute_prerequisite(
        mut self,
        attribute_name: AttributeName,
        dots: NonZeroU8,
    ) -> Self {
        let upsert = MeritPrerequisite::Attribute(attribute_name, dots);
        if let Some(existing) = self.prerequisites.iter_mut().find(|prereq| {
            if let MeritPrerequisite::Attribute(existing_name, _) = prereq {
                existing_name == &attribute_name
            } else {
                false
            }
        }) {
            *existing = upsert;
        } else {
            self.prerequisites.push(upsert);
        }
        self
    }

    /// Defines the merit template to only have a single dot rating, and a
    /// flat description for that merit.
    pub fn fixed_dots(self, dots: u8, description: String) -> FixedMeritTemplateBuilder {
        FixedMeritTemplateBuilder {
            name: self.name,
            book_reference: self.book_reference,
            merit_type: self.merit_type,
            dot_requirement: dots,
            description,
            prerequisites: self.prerequisites,
        }
    }

    /// Defines the merit template to be purchasable at multiple ratings, with
    /// a static description applicable to all ratings.
    pub fn variable_dots(self, shared_description: String) -> VariableMeritTemplateBuilder {
        VariableMeritTemplateBuilder {
            name: self.name,
            book_reference: self.book_reference,
            merit_type: self.merit_type,
            shared_description,
            dot_descriptions: Vec::new(),
            prerequisites: self.prerequisites,
        }
    }
}
