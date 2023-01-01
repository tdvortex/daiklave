use serde::{Deserialize, Serialize};

use super::{
    exalt::{
        ExaltMemo,
    },
    mortal::MortalMemo, ExaltationView,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltationMemo {
    Mortal(Box<MortalMemo>),
    Exalt(Box<ExaltMemo>),
}

impl<'source> ExaltationMemo {
    pub fn as_ref(&'source self) -> ExaltationView<'source> {
        match self {
            ExaltationMemo::Mortal(box_memo) => ExaltationView::Mortal(Box::new(box_memo.as_ref().as_ref())),
            ExaltationMemo::Exalt(box_memo) => ExaltationView::Exalt(Box::new(box_memo.as_ref().as_ref())),
        }
    }
}