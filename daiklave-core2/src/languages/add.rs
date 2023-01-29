use crate::{CharacterMutation, merits::merit::AddMerit};

use super::language::{MajorLanguage, LocalTongueName};

pub struct AddLanguages {
    pub major_languages: Vec<MajorLanguage>,
    pub local_tongues: Vec<LocalTongueName>,
}

impl From<AddLanguages> for CharacterMutation {
    fn from(add_languages: AddLanguages) -> Self {
        Self::AddMerit(add_languages.into::<AddMerit>())
    }
}