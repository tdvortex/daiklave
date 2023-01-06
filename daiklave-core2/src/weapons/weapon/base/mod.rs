pub mod builder;
mod id;

use std::collections::HashSet;

pub use id::BaseWeaponId;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use super::{
    ability::WeaponAbility, damage_type::WeaponDamageType, range::WeaponRange,
    tag::OptionalWeaponTag, weight_class::WeaponWeightClass,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseWeaponMemo {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) weight_class: WeaponWeightClass,
    pub(crate) range_bands: WeaponRange,
    pub(crate) primary_ability: WeaponAbility,
    pub(crate) damage_type: WeaponDamageType,
    pub(crate) tags: HashSet<OptionalWeaponTag>,
}
