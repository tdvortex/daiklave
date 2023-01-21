mod id;
mod keyword;
use std::collections::HashSet;

pub use id::SolarCharmId;
pub use keyword::SolarCharmKeyword;

use serde::{Serialize, Deserialize};

use crate::{book_reference::BookReference, charms::{CharmCost, CharmActionType}};

/// A Solar charm. 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarCharm {
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    essence_required: u8,
    ability_required: u8,
    charms_required: HashSet<SolarCharmId>,
    keywords: HashSet<SolarCharmKeyword>,
    costs: Vec<CharmCost>,
    action_type: CharmActionType,
    duration: String,
}