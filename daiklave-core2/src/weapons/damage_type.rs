use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(in crate::weapons) enum WeaponDamageType {
    Bashing,
    Lethal,
}
