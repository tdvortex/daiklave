mod add;
mod local_tongue_name;
mod major;
mod remove;
pub use major::MajorLanguage;
pub use remove::RemoveLanguage;

mod mutation;
mod set_native;
pub use add::AddLanguage;
pub use local_tongue_name::LocalTongueName;
pub(crate) use mutation::LanguageMutation;
pub use set_native::SetNativeLanguage;

use serde::{Deserialize, Serialize};

/// A language spoken by a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language<'source> {
    /// One of the major languages of Creation.
    MajorLanguage(MajorLanguage),
    /// A local tongue, not broadly spoken.
    LocalTongue(&'source str),
}

impl<'source> From<&'source LanguageMutation> for Language<'source> {
    fn from(mutation: &'source LanguageMutation) -> Self {
        match mutation {
            LanguageMutation::MajorLanguage(major) => Self::MajorLanguage(*major),
            LanguageMutation::LocalTongue(local) => Self::LocalTongue(local),
        }
    }
}
