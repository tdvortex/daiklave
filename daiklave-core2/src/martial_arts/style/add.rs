use crate::{CharacterMutation};

use super::{MartialArtsStyleName, MartialArtsStyle, builder::MartialArtsStyleBuilder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMartialArtsStyle {
    style_name: MartialArtsStyleName,
    style: MartialArtsStyle,
}

impl AddMartialArtsStyle {
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