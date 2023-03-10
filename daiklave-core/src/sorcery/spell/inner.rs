use std::{collections::HashSet, num::NonZeroU8};

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use super::{cost::SpellCost, SpellKeyword};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpellInner {
    pub(crate) summary: Option<String>,
    pub(crate) cost: SpellCost,
    pub(crate) duration: String,
    pub(crate) description: String,
    pub(crate) control_spell_description: Option<String>,
    pub(crate) distortion: Option<(NonZeroU8, String)>,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) keywords: HashSet<SpellKeyword>,
}
