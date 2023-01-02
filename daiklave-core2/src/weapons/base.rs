use std::collections::HashSet;

use crate::book_reference::BookReference;

use super::{weight_class::WeaponWeightClass, range::WeaponRange, ability::WeaponAbility, damage_type::WeaponDamageType, tag::OtherWeaponTag};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) struct BaseWeapon<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    weight_class: WeaponWeightClass,
    range_bands: WeaponRange,
    primary_ability: WeaponAbility,
    damage_type: WeaponDamageType,
    tags: HashSet<OtherWeaponTag>,
}

impl<'source> BaseWeapon<'source> {
    pub fn name(&self) -> &'source str {
        self.name
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    pub fn weight_class(&self) -> WeaponWeightClass {
        self.weight_class
    }
}