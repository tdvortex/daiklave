use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCostType},
};

/// A builder path to construct an Evocation.
pub mod builder;
mod evokable_id;
mod id;
mod keyword;
pub use evokable_id::EvokableId;
pub use id::EvocationId;
pub use keyword::EvocationKeyword;

use super::CharmId;

/// A Charm which is drawn from the unique power of a Hearthstone or named
/// Artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evocation {
    evokable_id: EvokableId,
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    resonant: Option<String>,
    dissonant: Option<String>,
    essence_required: NonZeroU8,
    evocation_tree: HashSet<EvocationId>,
    upgrade_charm: Option<CharmId>,
    keywords: HashSet<EvocationKeyword>,
    costs: HashMap<CharmCostType, NonZeroU8>,
    action_type: CharmActionType,
    duration: String,
}
