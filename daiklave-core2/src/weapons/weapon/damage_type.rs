use serde::{Deserialize, Serialize};

use super::WeaponTag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponDamageType {
    Bashing,
    Lethal,
}

impl From<WeaponDamageType> for WeaponTag {
    fn from(damage: WeaponDamageType) -> Self {
        match damage {
            WeaponDamageType::Bashing => WeaponTag::Bashing,
            WeaponDamageType::Lethal => WeaponTag::Lethal,
        }
    }
}
