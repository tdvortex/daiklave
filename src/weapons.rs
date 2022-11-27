use std::{collections::HashSet, ops::Deref};

use eyre::Result;

use crate::RangeBand;

enum WeightClass {
    Light,
    Medium,
    Heavy
}

enum AttackMethod {
    Archery(RangeBand),
    Brawl,
    MartialArts,
    Melee,
    Thrown(RangeBand),
}

enum Handedness {
    OneHanded,
    TwoHanded,
}

enum DamageType {
    Bashing,
    Lethal,
}

enum SpecialTag {
    Artifact,
    Exceptional,
    Flame,
}

pub enum Tag {
    Archery(RangeBand),
    Brawl,
    MartialArts,
    Melee,
    Thrown(RangeBand),
    TwoHanded,
}

enum Equipped {
    HandsFree,
    MainHandOnly(OneHandedWeapon),
    TwoDifferent(OneHandedWeapon, OneHandedWeapon),
    Paired(OneHandedWeapon),
    TwoHanded(TwoHandedWeapon),
}

#[derive(Debug)]
pub struct Weapons {


}

impl Default for Weapons {
    fn default() -> Self {
        Self {}
    }
}

impl Weapons {
    pub fn iter(&self) -> WeaponsIter {
        WeaponsIter {}
    }

    pub fn get_weapon(&self) -> Weapon { // TODO: add weapon index/identifier
        Weapon {  }
    }

    pub fn add_weapon(&mut self) { // TODO: add weapon constructor
        todo!()
    }

    pub fn remove_weapon(&mut self) {// TODO: add weapon index/identifier
        todo!()
    }

    pub fn equip_weapon(&mut self) -> Result<()> { // TODO: add weapon index/identifier
        todo!()
    }

    pub fn unequip_weapon(&mut self) -> Result<()> { // TODO: add weapon index/identifier
        todo!()
    }
}


struct WeaponDetails {
    name: String,

    special_tags: HashSet<SpecialTag>,
}

struct OneHandedWeapon(WeaponDetails);

impl Deref for OneHandedWeapon {
    type Target = WeaponDetails;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
struct TwoHandedWeapon(WeaponDetails);


impl Deref for TwoHandedWeapon {
    type Target = WeaponDetails;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct Weapon {
    
}

impl Weapon {
    pub fn name(&self) -> &str {
        todo!()
    }

    pub fn accuracy(&self, range: RangeBand) -> i8 {
        todo!()
    }

    pub fn damage(&self) -> i8 {
        todo!()
    }

    pub fn defense(&self) -> Option<i8> {
        todo!()
    }

    pub fn overwhelming(&self) -> i8 {
        todo!()
    }

    pub fn tags(&self) -> TagsIter {
        todo!()
    }
}


pub struct WeaponsIter {}

impl Iterator for WeaponsIter {
    type Item = Weapon;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}