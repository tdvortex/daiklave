mod memo;
pub(crate) use memo::MortalWondersMemo;

use std::collections::HashMap;

use crate::{artifact::wonders::{WonderId, WonderNoAttunement, OwnedWonder}, exaltation::exalt::ExaltWonders};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalWonders<'source>(pub(crate) HashMap<WonderId, WonderNoAttunement<'source>>);

impl<'source> MortalWonders<'source> {
    pub fn as_memo(&self) -> MortalWondersMemo {
        MortalWondersMemo(self.0.iter().map(|(k, v)| (*k, v.as_memo())).collect())
    }

    pub fn iter(&self) -> impl Iterator<Item = WonderId> + '_ {
        self.0.keys().copied()
    }

    pub fn get(&self, wonder_id: WonderId) -> Option<OwnedWonder<'source>> {
        self.0.get(&wonder_id).map(|no_attunement| OwnedWonder(wonder_id, no_attunement.clone(), None))
    }
}

impl<'source> From<ExaltWonders<'source>> for MortalWonders<'source> {
    fn from(exalt: ExaltWonders<'source>) -> Self {
        MortalWonders(exalt.0.into_iter().map(|(id, (no_attunement, _))| (id, no_attunement)).collect())
    }
}