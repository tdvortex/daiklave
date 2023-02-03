use crate::CharacterMutation;

use super::{builder::MartialArtsStyleBuilder, MartialArtsStyleDetails, MartialArtsStyleName};

/// Add a Martial Arts style (and the associated Martial Artist merit) to a
/// character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMartialArtsStyle {
    pub(crate) style_name: MartialArtsStyleName,
    pub(crate) style: MartialArtsStyleDetails,
}

impl AddMartialArtsStyle {
    /// Starts building a Martial Arts style with the given name.
    pub fn name(name: impl Into<MartialArtsStyleName>) -> MartialArtsStyleBuilder {
        MartialArtsStyleBuilder {
            name: name.into(),
            book_reference: None,
            max_armor_weight: None,
        }
    }
}

impl From<AddMartialArtsStyle> for CharacterMutation {
    fn from(add_martial_arts_style: AddMartialArtsStyle) -> Self {
        Self::AddMerit(add_martial_arts_style.into())
    }
}
