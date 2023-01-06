use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use super::{
    ability::WeaponAbility, damage_type::WeaponDamageType, range::WeaponRange, tag::OptionalWeaponTag,
    weight_class::WeaponWeightClass,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseWeapon<'source> {
    pub(crate) name: &'source str,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) weight_class: WeaponWeightClass,
    pub(crate) range_bands: WeaponRange,
    pub(crate) primary_ability: WeaponAbility,
    pub(crate) damage_type: WeaponDamageType,
    pub(crate) tags: HashSet<OptionalWeaponTag>,
}

impl<'source> BaseWeapon<'source> {
    pub(crate) fn as_memo(&self) -> BaseWeaponMemo {
        BaseWeaponMemo {
            name: self.name.to_string(),
            book_reference: self.book_reference,
            weight_class: self.weight_class,
            range_bands: self.range_bands,
            primary_ability: self.primary_ability,
            damage_type: self.damage_type,
            tags: self.tags.to_owned(),
        }
    }

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
pub struct BaseWeaponMemo {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) weight_class: WeaponWeightClass,
    pub(crate) range_bands: WeaponRange,
    pub(crate) primary_ability: WeaponAbility,
    pub(crate) damage_type: WeaponDamageType,
    pub(crate) tags: HashSet<OptionalWeaponTag>,
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
            tags: self.tags.clone(),
        }
    }
}
