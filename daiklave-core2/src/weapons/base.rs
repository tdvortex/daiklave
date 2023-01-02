use std::collections::HashSet;

use crate::book_reference::BookReference;

use super::{weight_class::WeaponWeightClass, range::WeaponRange, ability::WeaponAbility, damage_type::WeaponDamageType, tag::WeaponTag};

pub(in crate::weapons) struct BaseWeapon<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    weight_class: WeaponWeightClass,
    range_bands: WeaponRange,
    primary_ability: WeaponAbility,
    damage_type: WeaponDamageType,
    tags: HashSet<WeaponTag>,
}