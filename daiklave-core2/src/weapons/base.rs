use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use crate::book_reference::BookReference;

use super::{weight_class::WeaponWeightClass, range::WeaponRange, ability::WeaponAbility, damage_type::WeaponDamageType, tag::OtherWeaponTag};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::weapons) struct BaseWeapon<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    weight_class: WeaponWeightClass,
    range_bands: WeaponRange,
    primary_ability: WeaponAbility,
    damage_type: WeaponDamageType,
    tags: &'source HashSet<OtherWeaponTag>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) struct BaseWeaponMemo {
    name: String,
    book_reference: Option<BookReference>,
    weight_class: WeaponWeightClass,
    range_bands: WeaponRange,
    primary_ability: WeaponAbility,
    damage_type: WeaponDamageType,
    tags: HashSet<OtherWeaponTag>,
}

impl<'source> BaseWeaponMemo {
    pub fn as_ref(&'source self) -> BaseWeapon<'source> {
        BaseWeapon { 
            name: self.name.as_str(), 
            book_reference: self.book_reference, 
            weight_class: self.weight_class, 
            range_bands: self.range_bands, 
            primary_ability: self.primary_ability, 
            damage_type: self.damage_type,
            tags: &self.tags
        }
    }
}