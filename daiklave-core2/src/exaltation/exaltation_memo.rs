use serde::{Deserialize, Serialize};

use super::{
    exalt::{
        ExaltMemo,
    },
    mortal::MortalMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltationMemo {
    Mortal(Box<MortalMemo>),
    Exalt(Box<ExaltMemo>),
}