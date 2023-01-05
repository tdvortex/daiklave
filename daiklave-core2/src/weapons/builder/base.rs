use crate::{book_reference::BookReference, weapons::{WeaponWeightClass, mundane::MundaneWeapon, RangeBand, OtherWeaponTag}};

use super::artifact::BaseArtifactWeaponInsert;

pub struct BaseWeaponBuilder<'build> {
    name: &'build str
}

impl<'build> BaseWeaponBuilder<'build> {
    pub fn book_reference(self, book_reference: BookReference) -> Self {
        todo!()
    }

    pub fn weight_class(self, weight_class: WeaponWeightClass) -> BaseWeaponBuilderWithWeight<'build> {
        todo!()
    }
}

pub struct BaseWeaponBuilderWithWeight<'build> {
    name: &'build str
}

impl<'build> BaseWeaponBuilderWithWeight<'build> {
    pub fn book_reference(self, book_reference: BookReference) -> Self {
        todo!()
    }

    pub fn natural(self) -> BaseWeaponBuilderWithHandedness<'build> {
        todo!()
    }

    pub fn worn(self) -> BaseWeaponBuilderWithHandedness<'build> {
        todo!()
    }

    pub fn one_handed(self) -> BaseWeaponBuilderWithHandedness<'build> {
        todo!()
    }

    pub fn two_handed(self) -> BaseWeaponBuilderWithHandedness<'build> {
        todo!()
    }
}

pub struct BaseWeaponBuilderWithHandedness<'build> {
    name: &'build str
}

impl<'build> BaseWeaponBuilderWithHandedness<'build> {
    pub fn book_reference(self, book_reference: BookReference) ->Self {
        todo!()
    }

    pub fn lethal(self) -> BaseWeaponBuilderWithDamageType<'build> {
        todo!()
    }

    pub fn bashing(self) -> BaseWeaponBuilderWithDamageType<'build> {
        todo!()
    }
}

pub struct BaseWeaponBuilderWithDamageType<'build> {
    name: &'build str
}

impl<'build> BaseWeaponBuilderWithDamageType<'build> {
    pub fn book_reference(self, book_reference: BookReference) -> Self {
        todo!()
    }

    pub fn brawl(self) -> BaseWeaponBuilderWithAttack<'build> {
        todo!()
    }

    pub fn melee(self) -> BaseWeaponBuilderWithAttack<'build> {
        todo!()
    }

    pub fn archery(self) -> BaseWeaponBuilderWithAttack<'build> {
        todo!()
    }

    pub fn thrown(self) -> BaseWeaponBuilderWithAttack<'build> {
        todo!()
    }

    pub fn martial_arts(self) -> BaseWeaponBuilderWithAttack<'build> {
        todo!()
    }
}

pub struct BaseWeaponBuilderWithAttack<'build> {
    name: &'build str
}

impl<'build> BaseWeaponBuilderWithAttack<'build> {
    pub fn book_reference(self, book_reference: BookReference) -> Self {
        todo!()
    }

    pub fn thrown_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    pub fn archery_range(self, max_range: RangeBand) -> Self {
        todo!()
    }

    pub fn tag(self, tag: OtherWeaponTag) -> Self {
        todo!()
    }

    pub fn build_mundane(self) -> MundaneWeapon<'build> {
        todo!()
    }

    pub fn build_artifact(self) -> BaseArtifactWeaponInsert {
        todo!()
    }
}