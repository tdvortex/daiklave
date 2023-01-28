use serde::{Serialize, Deserialize};

use crate::CharacterMutation;

use super::{MajorLanguage, LocalTongueName};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetNativeLanguage {
    /// One of the major regional languages of Creation, or other significant
    /// language.
    MajorLanguage(MajorLanguage),
    /// A local tongue not widely spoken.
    LocalTongue(LocalTongueName),
}

impl From<MajorLanguage> for SetNativeLanguage {
    fn from(major_language: MajorLanguage) -> Self {
        Self::MajorLanguage(major_language)
    }
}

impl From<LocalTongueName> for SetNativeLanguage {
    fn from(local_tongue: LocalTongueName) -> Self {
        Self::LocalTongue(local_tongue)
    }
}

impl From<SetNativeLanguage> for CharacterMutation {
    fn from(set_native_language: SetNativeLanguage) -> Self {
        Self::SetNativeLanguage(set_native_language)
    }
}