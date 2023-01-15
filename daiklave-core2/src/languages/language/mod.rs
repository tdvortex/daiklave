mod major;
pub use major::MajorLanguage;

mod memo;
pub use memo::LanguageMutation;

use serde::{Serialize, Deserialize};

/// A language spoken by a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language<'source> {
    /// One of the major languages of Creation.
    MajorLanguage(MajorLanguage),
    /// A local tongue, not broadly spoken.
    LocalTongue(&'source str),
}

impl<'source> Language<'source> {
    pub(crate) fn as_memo(&self) -> LanguageMutation {
        match self {
            Language::MajorLanguage(major) => LanguageMutation::MajorLanguage(*major),
            Language::LocalTongue(local) => LanguageMutation::LocalTongue(local.to_string()),
        }
    }
}

impl<'source> Default for Language<'source> {
    fn default() -> Self {
        Self::MajorLanguage(Default::default())
    }
}