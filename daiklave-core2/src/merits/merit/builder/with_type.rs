use crate::{book_reference::BookReference, merits::merit::{MeritType, prerequisite::MeritPrerequisite}, abilities::AbilityName, attributes::AttributeName};

use super::{FixedMeritTemplateBuilder, VariableMeritTemplateBuilder};

pub struct MeritTemplateBuilderWithType {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_type: MeritType,
    pub(crate) prerequisites: Vec<MeritPrerequisite>,
}

impl MeritTemplateBuilderWithType {
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

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

    pub fn variable_dots(self, shared_description: String) -> VariableMeritTemplateBuilder {
        VariableMeritTemplateBuilder {
            name: self.name,
            book_reference: self.book_reference,
            merit_type: self.merit_type,
            shared_description,
            dot_descriptions: Vec::new(),
            prerequisites: self.prerequisites
        }
    }
}