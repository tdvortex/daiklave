mod id;
use std::collections::HashSet;

pub use id::SolarCharmId;

use serde::{Serialize, Deserialize};

use crate::{book_reference::BookReference, martial_arts::MartialArtsCharmId, charms::{CharmKeyword, CharmCost, CharmActionType}};

/// A Solar charm. 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarCharm {
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    essence_required: u8,
    ability_required: u8,
    charms_required: HashSet<MartialArtsCharmId>,
    keywords: HashSet<CharmKeyword>,
    costs: Vec<CharmCost>,
    action_type: CharmActionType,
    duration: String,
}