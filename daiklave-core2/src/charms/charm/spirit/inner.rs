use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost},
};

use super::SpiritCharmKeyword;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SpiritCharmInner {
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    essence_required: u8,
    keywords: HashSet<SpiritCharmKeyword>,
    costs: Vec<CharmCost>,
    action_type: CharmActionType,
    duration: String,
}
