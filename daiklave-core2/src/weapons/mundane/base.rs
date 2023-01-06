// use std::ops::Deref;

// use serde::{Deserialize, Serialize};

// use crate::weapons::base::{BaseWeapon, BaseWeaponMemo};

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct BaseMundaneWeapon<'source>(pub(crate) BaseWeapon<'source>);

// impl<'source> Deref for BaseMundaneWeapon<'source> {
//     type Target = BaseWeapon<'source>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<'source> BaseMundaneWeapon<'source> {
//     pub fn as_memo(&self) -> BaseMundaneWeaponMemo {
//         BaseMundaneWeaponMemo(self.0.as_memo())
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// struct BaseMundaneWeaponMemo(BaseWeaponMemo);

// impl<'source> BaseMundaneWeaponMemo {
//     pub fn as_ref(&'source self) -> BaseMundaneWeapon<'source> {
//         BaseMundaneWeapon(self.0.as_ref())
//     }
// }
