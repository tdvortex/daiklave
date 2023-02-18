use serde::{Deserialize, Serialize};

use super::{exalt::ExaltMemo, mortal::MortalMemo, Exaltation};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltationMemo {
    Mortal(Box<MortalMemo>),
    Exalt(Box<ExaltMemo>),
}

impl From<Exaltation<'_>> for ExaltationMemo {
    fn from(exaltation: Exaltation<'_>) -> Self {
        match exaltation {
            Exaltation::Mortal(box_view) => Self::Mortal(Box::new(box_view.as_ref().into())),
            Exaltation::Exalt(box_view) => Self::Exalt(Box::new(box_view.as_ref().into())),
        }
    }
}