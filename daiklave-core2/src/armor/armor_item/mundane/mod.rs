mod memo;

pub use memo::MundaneArmorMemo;

use super::base::BaseArmor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MundaneArmor<'source>(pub &'source BaseArmor);
