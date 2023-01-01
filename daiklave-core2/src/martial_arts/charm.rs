use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost, CharmKeyword},
};

use super::{charm_id::MartialArtsCharmId, style_id::MartialArtsStyleId};

/// A Martial Arts charm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MartialArtsCharm {
    style: MartialArtsStyleId,
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

impl MartialArtsCharm {
    pub fn style(&self) -> MartialArtsStyleId {
        self.style
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn essence_required(&self) -> u8 {
        self.essence_required
    }

    pub fn ability_required(&self) -> u8 {
        self.ability_required
    }

    pub fn charms_required(&self) -> impl Iterator<Item = MartialArtsCharmId> + '_ {
        self.charms_required.iter().copied()
    }

    pub fn keywords(&self) -> impl Iterator<Item = CharmKeyword> + '_ {
        self.keywords.iter().copied()
    }

    pub fn costs(&self) -> impl Iterator<Item = CharmCost> + '_ {
        self.costs.iter().copied()
    }
}
