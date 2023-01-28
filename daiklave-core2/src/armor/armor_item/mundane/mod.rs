mod add;
mod memo;
mod name;
mod remove;

use std::ops::Deref;

pub use add::AddMundaneArmor;
pub use memo::MundaneArmor;
pub use name::MundaneArmorName;
pub use remove::RemoveMundaneArmor;

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
}
