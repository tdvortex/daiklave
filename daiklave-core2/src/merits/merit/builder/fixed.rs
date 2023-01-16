use crate::{
    abilities::AbilityName,
    attributes::AttributeName,
    book_reference::BookReference,
    merits::merit::{
        nonstackable::NonStackableMeritTemplate,
        prerequisite::MeritPrerequisite,
        stackable::StackableMeritTemplateId,
        template::{MeritTemplate, MeritTemplateDotOptions},
        MeritError, MeritType, NonStackableMeritId, StackableMeritTemplate,
    },
};

pub struct FixedMeritTemplateBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_type: MeritType,
    pub(crate) prerequisites: Vec<MeritPrerequisite>,
    pub(crate) dot_requirement: u8,
    pub(crate) description: String,
}

impl FixedMeritTemplateBuilder {
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

    pub fn stackable(
        self,
        id: StackableMeritTemplateId,
    ) -> Result<StackableMeritTemplate, MeritError> {
        if !(0..=5).contains(&self.dot_requirement) {
            Err(MeritError::InvalidDotRating)
        } else {
            Ok(StackableMeritTemplate(
                id,
                MeritTemplate {
                    name: self.name,
                    book_reference: self.book_reference,
                    merit_type: self.merit_type,
                    shared_description: self.description,
                    dot_options: MeritTemplateDotOptions::Fixed(self.dot_requirement),
                    prerequisites: self.prerequisites,
                },
            ))
        }
    }

    pub fn nonstackable(
        self,
        id: NonStackableMeritId,
    ) -> Result<NonStackableMeritTemplate, MeritError> {
        if !(0..=5).contains(&self.dot_requirement) {
            Err(MeritError::InvalidDotRating)
        } else {
            Ok(NonStackableMeritTemplate(
                id,
                MeritTemplate {
                    name: self.name,
                    book_reference: self.book_reference,
                    merit_type: self.merit_type,
                    shared_description: self.description,
                    dot_options: MeritTemplateDotOptions::Fixed(self.dot_requirement),
                    prerequisites: self.prerequisites,
                },
            ))
        }
    }
}
