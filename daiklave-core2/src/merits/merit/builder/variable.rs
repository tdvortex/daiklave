use crate::{book_reference::BookReference, merits::merit::{MeritType, prerequisite::MeritPrerequisite, stackable::StackableMeritTemplateId, StackableMeritTemplate, MeritError, template::{MeritTemplate, MeritTemplateDotOptions}, NonStackableMeritId, nonstackable::NonStackableMeritTemplate}, abilities::AbilityName, attributes::AttributeName};

pub struct VariableMeritTemplateBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_type: MeritType,
    pub(crate) prerequisites: Vec<MeritPrerequisite>,
    pub(crate) shared_description: String,
    pub(crate) dot_descriptions: Vec<(u8, String)>,
}

impl VariableMeritTemplateBuilder {
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

    pub fn dot_option(mut self, dots: u8, dot_description: String) -> Self {
        self.dot_descriptions.push((dots, dot_description));
        self
    }

    pub fn stackable(
        self,
        id: StackableMeritTemplateId,
    ) -> Result<StackableMeritTemplate, MeritError> {
        if self.dot_descriptions.is_empty() {
            return Err(MeritError::MissingDotRating);
        }

        let mut options = [None, None, None, None, None, None];

        for (dot_level, dot_description) in self.dot_descriptions.into_iter() {
            if !(0..=5).contains(&dot_level) {
                return Err(MeritError::InvalidDotRating);
            } else {
                options[dot_level as usize] = Some(dot_description);
            }
        }

        Ok(StackableMeritTemplate(
            id,
            MeritTemplate {
                name: self.name,
                book_reference: self.book_reference,
                merit_type: self.merit_type,
                shared_description: self.shared_description,
                dot_options: MeritTemplateDotOptions::Variable(options),
                prerequisites: self.prerequisites,
            },
        ))
    }

    pub fn nonstackable(
        self,
        id: NonStackableMeritId,
    ) -> Result<NonStackableMeritTemplate, MeritError> {
        if self.dot_descriptions.is_empty() {
            return Err(MeritError::MissingDotRating);
        }

        let mut options = [None, None, None, None, None, None];

        for (dot_level, dot_description) in self.dot_descriptions.into_iter() {
            if !(0..=5).contains(&dot_level) {
                return Err(MeritError::InvalidDotRating);
            } else {
                options[dot_level as usize] = Some(dot_description);
            }
        }

        Ok(NonStackableMeritTemplate(
            id,
            MeritTemplate {
                name: self.name,
                book_reference: self.book_reference,
                merit_type: self.merit_type,
                shared_description: self.shared_description,
                dot_options: MeritTemplateDotOptions::Variable(options),
                prerequisites: self.prerequisites
            },
        ))
    }
}