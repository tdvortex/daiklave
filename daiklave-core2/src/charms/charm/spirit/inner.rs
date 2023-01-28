use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU8,
};

use serde::{Deserialize, Serialize};

use crate::{
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost, CharmCostType},
};

use super::SpiritCharmKeyword;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SpiritCharmInner {
    pub book_reference: Option<BookReference>,
    pub summary: Option<String>,
    pub description: String,
    pub essence_required: NonZeroU8,
    pub keywords: HashSet<SpiritCharmKeyword>,
    pub costs: HashMap<CharmCostType, NonZeroU8>,
    pub action_type: CharmActionType,
    pub duration: String,
}

impl SpiritCharmInner {
    pub fn keywords(&self) -> impl Iterator<Item = SpiritCharmKeyword> + '_ {
        self.keywords.iter().copied()
    }

    pub fn costs(&self) -> impl Iterator<Item = CharmCost> + '_ {
        self.costs
            .iter()
            .map(|(cost_type, amount)| CharmCost::new(*cost_type, amount.get()))
    }
}
