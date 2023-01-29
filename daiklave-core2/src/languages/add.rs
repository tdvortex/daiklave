use std::collections::HashSet;

use crate::{CharacterMutation, merits::merit::AddMerit};

use super::language::{MajorLanguage, LocalTongueName, LanguageMutation};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AddLanguages(pub(crate) HashSet<LanguageMutation>);

impl AddLanguages {
    pub fn major_language(&mut self, major_language: MajorLanguage) -> &mut Self {
        self.0.insert(LanguageMutation::MajorLanguage(major_language));
        self
    }

    pub fn local_tongue(&mut self, local_tongue: LocalTongueName) -> &mut Self {
        self.0.insert(LanguageMutation::LocalTongue(local_tongue));
        self
    }
}

impl From<AddLanguages> for CharacterMutation {
    fn from(add_languages: AddLanguages) -> Self {
        AddMerit::from(add_languages).into()
    }
}