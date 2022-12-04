use std::collections::HashSet;

use eyre::{eyre, Result};
use slab::Slab;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ArmorTag {
    Artifact,
    Concealable,
    Heavy,
    Light,
    Medium,
    Silent,
    Special,
}

#[derive(Debug)]
pub struct ArmorItem {
    name: String,
    weight_class: WeightClass,
    artifact: bool,
    concealable: bool,
    silent: bool,
    special: bool,
}

impl ArmorItem {
    pub fn new(name: String, tags: HashSet<ArmorTag>) -> Result<ArmorItem> {
        let mut weight_class = None::<WeightClass>;
        let mut artifact = false;
        let mut concealable = false;
        let mut silent = false;
        let mut special = false;

        for tag in tags {
            match tag {
                ArmorTag::Artifact => {
                    artifact = true;
                }
                ArmorTag::Concealable => {
                    concealable = true;
                }
                ArmorTag::Heavy => {
                    weight_class = Some(WeightClass::Heavy);
                }
                ArmorTag::Light => {
                    weight_class = Some(WeightClass::Light);
                }
                ArmorTag::Medium => {
                    weight_class = Some(WeightClass::Medium);
                }
                ArmorTag::Silent => {
                    silent = true;
                }
                ArmorTag::Special => {
                    special = true;
                }
            }
        }

        if weight_class.is_none() {
            return Err(eyre!(
                "armor must be exactly one of Light, Medium, or Heavy"
            ));
        }

        Ok(ArmorItem {
            name,
            weight_class: weight_class.unwrap(),
            artifact,
            concealable,
            silent,
            special,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn soak(&self) -> u8 {
        match (&self.weight_class, self.artifact) {
            (WeightClass::Light, false) => 3,
            (WeightClass::Medium, false) => 5,
            (WeightClass::Heavy, false) => 7,
            (WeightClass::Light, true) => 5,
            (WeightClass::Medium, true) => 8,
            (WeightClass::Heavy, true) => 11,
        }
    }

    pub fn mobility_penality(&self) -> i8 {
        match &self.weight_class {
            WeightClass::Light => 0,
            WeightClass::Medium => -1,
            WeightClass::Heavy => -2,
        }
    }

    pub fn hardness(&self) -> u8 {
        match (&self.weight_class, self.artifact) {
            (WeightClass::Light, true) => 4,
            (WeightClass::Medium, true) => 7,
            (WeightClass::Heavy, true) => 10,
            (_, false) => 0,
        }
    }

    pub fn attunement(&self) -> u8 {
        match (&self.weight_class, self.artifact) {
            (WeightClass::Light, true) => 4,
            (WeightClass::Medium, true) => 5,
            (WeightClass::Heavy, true) => 6,
            (_, false) => 0,
        }
    }

    pub fn tags(&self) -> HashSet<ArmorTag> {
        let mut hash_set = HashSet::new();

        match self.weight_class {
            WeightClass::Light => {
                hash_set.insert(ArmorTag::Light);
            }
            WeightClass::Medium => {
                hash_set.insert(ArmorTag::Medium);
            }
            WeightClass::Heavy => {
                hash_set.insert(ArmorTag::Heavy);
            }
        };

        if self.artifact {
            hash_set.insert(ArmorTag::Artifact);
        }

        if self.concealable {
            hash_set.insert(ArmorTag::Concealable);
        }

        if self.silent {
            hash_set.insert(ArmorTag::Silent);
        }

        if self.special {
            hash_set.insert(ArmorTag::Special);
        }

        hash_set
    }
}

#[derive(Debug, Default)]
pub struct Armor {
    equipped: Option<usize>,
    owned: Slab<ArmorItem>,
}

impl Armor {
    pub fn iter(&self) -> impl Iterator<Item = (usize, &ArmorItem)> {
        self.owned.iter()
    }

    pub fn equipped(&self) -> Option<&ArmorItem> {
        self.equipped.map(|key| self.get(key).unwrap())
    }

    pub fn get(&self, key: usize) -> Result<&ArmorItem> {
        self.owned
            .get(key)
            .ok_or_else(|| eyre!("armor item {} not found", key))
    }

    pub fn add_armor_item(&mut self, armor_item: ArmorItem) -> usize {
        self.owned.insert(armor_item)
    }

    pub fn remove_armor_item(&mut self, key: usize) -> Result<()> {
        if !self.owned.contains(key) {
            Err(eyre!("armor item {} not found", key))
        } else {
            if let Some(worn) = &self.equipped {
                if *worn == key {
                    self.unequip_armor_item();
                }
            }

            self.owned.remove(key);
            Ok(())
        }
    }

    pub fn equip_armor_item(&mut self, key: usize) -> Result<()> {
        if !self.owned.contains(key) {
            Err(eyre!("armor item {} not found", key))
        } else {
            self.equipped = Some(key);
            Ok(())
        }
    }

    pub fn unequip_armor_item(&mut self) {
        self.equipped = None;
    }
}

#[derive(Debug)]
enum WeightClass {
    Light,
    Medium,
    Heavy,
}
