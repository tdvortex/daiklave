mod memo;
pub(crate) use memo::ExaltWondersMemo;

use std::collections::HashMap;

use crate::{
    artifact::wonders::{OwnedWonder, WonderId, WonderNoAttunement},
    exaltation::mortal::MortalWonders,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltWonders<'source>(
    pub(crate) HashMap<WonderId, (WonderNoAttunement<'source>, Option<u8>)>,
);

impl<'source> ExaltWonders<'source> {
    pub fn as_memo(&self) -> ExaltWondersMemo {
        ExaltWondersMemo(
            self.0
                .iter()
                .map(|(k, (no_attunement, attunement))| {
                    (*k, (no_attunement.as_memo(), *attunement))
                })
                .collect(),
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = WonderId> + '_ {
        self.0.keys().copied()
    }

    pub fn get(&self, wonder_id: WonderId) -> Option<OwnedWonder<'source>> {
        self.0.get(&wonder_id).map(|(no_attunement, attunement)| {
            OwnedWonder(wonder_id, no_attunement.clone(), *attunement)
        })
    }
}

impl<'source> From<MortalWonders<'source>> for ExaltWonders<'source> {
    fn from(mortal: MortalWonders<'source>) -> Self {
        ExaltWonders(
            mortal
                .0
                .into_iter()
                .map(|(id, no_attunement)| (id, (no_attunement, None)))
                .collect(),
        )
    }
}
