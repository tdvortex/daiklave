mod memo;

use std::ops::Deref;

pub use memo::MundaneArmorMemo;

use super::base::BaseArmor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MundaneArmor<'source>(pub &'source BaseArmor);

impl<'source> Deref for MundaneArmor<'source> {
    type Target = BaseArmor;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> MundaneArmor<'source> {
    pub fn name(&self) -> &'source str {
        self.0.name.as_str()
    }
}