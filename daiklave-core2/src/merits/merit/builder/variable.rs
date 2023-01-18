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

/// A builder for a merit which may be purchased at multiple different dot
/// ratings, such as (2 or 4) or (1+).
pub struct VariableMeritTemplateBuilder {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) merit_type: MeritType,
    pub(crate) prerequisites: Vec<MeritPrerequisite>,
    pub(crate) shared_description: String,
    pub(crate) dot_descriptions: Vec<(u8, String)>,
}

impl VariableMeritTemplateBuilder {
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

    /// Adds an option for a dot rating on the merit. Each dot option must be
    /// in the range [0, 5] inclusive and must have a description of what that
    /// rating represents.
    pub fn dot_option(mut self, dots: u8, dot_description: String) -> Self {
        self.dot_descriptions.push((dots, dot_description));
        self
    }

    /// Completes the builder, returning a stackable merit template. Errors if
    /// no dot options have been specified or if any rating is above 5.
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

    /// Completes the builder, returning a nonstackable merit template. Errors if
    /// no dot options have been specified or if any rating is above 5.
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
                prerequisites: self.prerequisites,
            },
        ))
    }
}
