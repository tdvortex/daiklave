/// A builder path for constructing a new Solar Charm.
pub mod builder;

mod ability;
mod id;
mod keyword;
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

pub use id::SolarCharmId;
pub use keyword::SolarCharmKeyword;

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCostType},
};

use self::{ability::SolarCharmAbility, builder::SolarCharmBuilder};

/// A Solar charm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarCharm {
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    essence_required: NonZeroU8,
    ability: SolarCharmAbility,
    ability_requirement: u8,
    charms_required: HashSet<SolarCharmId>,
    keywords: HashSet<SolarCharmKeyword>,
    costs: HashMap<CharmCostType, NonZeroU8>,
    action_type: CharmActionType,
    duration: String,
}

impl SolarCharm {
    /// Starts building a new Solar Charm.
    pub fn builder(name: String) -> SolarCharmBuilder {
        SolarCharmBuilder {
            name,
            book_reference: None,
            summary: None,
            charms_required: HashSet::new(),
            keywords: HashSet::new(),
            costs: HashMap::new(),
        }
    }
}
