mod fixed;
mod variable;
mod with_type;
pub use fixed::FixedMeritTemplateBuilder;
pub use variable::VariableMeritTemplateBuilder;
pub use with_type::MeritTemplateBuilderWithType;

use crate::{abilities::AbilityName, attributes::AttributeName, book_reference::BookReference};

use super::{prerequisite::MeritPrerequisite, MeritType};

/// A builder to construct a merit template.
pub struct MeritTemplateBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) prerequisites: Vec<MeritPrerequisite>,
}

impl MeritTemplateBuilder {
    /// Sets the book reference for the merit.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// Adds a requirement that a certain ability is rated at or above a
    /// dot threshold. If a merit has any prerequisites, they are treated as an
    /// "or" relationship--the merit can be added as long as any prerequisite
    /// is satisfied.
    pub fn ability_prerequisite(mut self, ability_name: AbilityName, dots: u8) -> Self {
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
    pub fn attribute_prerequisite(mut self, attribute_name: AttributeName, dots: u8) -> Self {
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

    /// Set the merit to be Innate, Purchased, Story, or Supernatural.
    pub fn merit_type(self, merit_type: MeritType) -> MeritTemplateBuilderWithType {
        MeritTemplateBuilderWithType {
            name: self.name,
            book_reference: self.book_reference,
            merit_type,
            prerequisites: self.prerequisites,
        }
    }
}
