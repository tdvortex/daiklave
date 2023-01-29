use crate::{CharacterMutation};

use super::{MartialArtsStyleName, MartialArtsStyle, builder::MartialArtsStyleBuilder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddMartialArtsStyle {
    style_name: MartialArtsStyleName,
    style: MartialArtsStyle,
}

impl AddMartialArtsStyle {
    pub fn builder(name: impl ToString) -> MartialArtsStyleBuilder {
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