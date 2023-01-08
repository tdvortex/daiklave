mod memo;

use std::ops::Deref;

pub use memo::MundaneArmor;

use super::base::BaseArmor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MundaneArmorView<'source>(pub &'source BaseArmor);

impl<'source> Deref for MundaneArmorView<'source> {
    type Target = BaseArmor;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'source> MundaneArmorView<'source> {
    pub fn as_memo(&self) -> MundaneArmor {
        MundaneArmor(self.0.to_owned())
    }

    pub fn name(&self) -> &'source str {
        self.0.name.as_str()
    }
}
