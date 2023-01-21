use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use crate::{book_reference::BookReference, charms::{CharmKeyword, CharmCost, CharmActionType}};

use self::evokable_id::EvokableItemId;

mod evokable_id;
mod id;
pub use id::EvocationId;

use super::CharmId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evocation {
    evokable_id: EvokableItemId,
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    resonant: Option<String>,
    dissonant: Option<String>,
    essence_required: u8,
    evocation_tree: HashSet<EvocationId>,
    upgrade_charm: Option<CharmId>,
    keywords: HashSet<CharmKeyword>,
    costs: Vec<CharmCost>,
    action_type: CharmActionType,
    duration: String,
}