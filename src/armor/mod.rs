pub(crate) mod create;
pub(crate) mod destroy;
pub(crate) mod update;
pub use destroy::destroy_armor;
use serde::{Deserialize, Serialize};
pub use update::ArmorDiff;
pub(crate) mod tables;
use std::{collections::HashSet, hash::Hash};

use eyre::{eyre, Result};

use crate::data_source::{BookReference, DataSource};

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

#[derive(Debug, Clone, Eq, Hash, Serialize, Deserialize)]
pub struct ArmorItem {
    id: Option<i32>,
    name: String,
    weight_class: WeightClass,
    artifact: bool,
    concealable: bool,
    silent: bool,
    special: bool,
    data_source: DataSource,
}

impl PartialEq for ArmorItem {
    fn eq(&self, other: &Self) -> bool {
        if self.id.is_some() && other.id.is_some() {
            self.id == other.id
        } else if self.id.is_none() && other.id.is_none() {
            self.weight_class == other.weight_class
                && self.artifact == other.artifact
                && self.concealable == other.concealable
                && self.silent == other.silent
                && self.special == other.special
                && self.data_source == other.data_source
        } else {
            false
        }
    }
}

impl ArmorItem {
    pub(crate) fn new(
        name: String,
        tags: HashSet<ArmorTag>,
        id: Option<i32>,
        data_source: DataSource,
    ) -> Result<ArmorItem> {
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
            id,
            name,
            weight_class: weight_class.unwrap(),
            artifact,
            concealable,
            silent,
            special,
            data_source,
        })
    }

    pub fn from_book(book_title: String, page_number: i16) -> ArmorBuilder {
        ArmorBuilder {
            id: None,
            name: None,
            weight_class: None,
            tags: HashSet::new(),
            data_source: DataSource::Book(BookReference {
                book_title,
                page_number,
            }),
        }
    }

    pub fn custom(creator_id: Option<i32>) -> ArmorBuilder {
        ArmorBuilder {
            id: None,
            name: None,
            weight_class: None,
            tags: HashSet::new(),
            data_source: DataSource::Custom(creator_id),
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn data_source(&self) -> &DataSource {
        &self.data_source
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Armor {
    inventory: Vec<(ArmorItem, bool)>,
}

impl Armor {
    pub fn iter(&self) -> impl Iterator<Item = (usize, &ArmorItem, bool)> {
        self.inventory
            .iter()
            .enumerate()
            .map(|(index, (item, worn))| (index, item, *worn))
    }

    pub fn get_by_ref(&self, item: &ArmorItem) -> Option<(usize, &ArmorItem, bool)> {
        self.iter()
            .find(|&(_, inventory_item, _)| inventory_item == item)
    }

    pub fn get_by_index(&self, index: usize) -> Option<(usize, &ArmorItem, bool)> {
        self.inventory
            .get(index)
            .map(|(item, worn)| (index, item, *worn))
    }

    pub fn add_armor_item(&mut self, armor_item: ArmorItem, worn: bool) {
        if worn {
            self.unequip_armor_item();
        }
        self.inventory.push((armor_item, worn));
        self.inventory.sort_by(|a, b| a.0.name().cmp(b.0.name()));
        self.inventory.dedup_by(|(a, _), (b, _)| a == b);
    }

    pub fn remove_armor_item(&mut self, index: usize) -> Result<(ArmorItem, bool)> {
        if self.inventory.len() <= index {
            Err(eyre!("armor item {} not found", index))
        } else {
            Ok(self.inventory.remove(index))
        }
    }

    pub fn equip_armor_item(&mut self, index: usize) -> Result<()> {
        if self.inventory.len() <= index {
            Err(eyre!("armor item {} not found", index))
        } else {
            self.inventory
                .iter_mut()
                .enumerate()
                .for_each(|(inventory_index, (_, worn))| *worn = inventory_index == index);
            Ok(())
        }
    }

    pub fn unequip_armor_item(&mut self) {
        self.inventory
            .iter_mut()
            .for_each(|(_, worn)| *worn = false);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum WeightClass {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug)]
pub struct ArmorBuilder {
    id: Option<i32>,
    name: Option<String>,
    weight_class: Option<WeightClass>,
    tags: HashSet<ArmorTag>,
    data_source: DataSource,
}

impl ArmorBuilder {
    pub(crate) fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn as_light(mut self) -> Self {
        self.weight_class = Some(WeightClass::Light);
        self
    }

    pub fn as_medium(mut self) -> Self {
        self.weight_class = Some(WeightClass::Medium);
        self
    }

    pub fn as_heavy(mut self) -> Self {
        self.weight_class = Some(WeightClass::Heavy);
        self
    }

    pub fn as_artifact(mut self) -> Self {
        self.tags.insert(ArmorTag::Artifact);
        self
    }

    pub fn with_tag(mut self, tag: ArmorTag) -> Self {
        match tag {
            ArmorTag::Artifact => self.as_artifact(),
            ArmorTag::Heavy => self.as_heavy(),
            ArmorTag::Light => self.as_light(),
            ArmorTag::Medium => self.as_medium(),
            other_tag => {
                self.tags.insert(other_tag);
                self
            }
        }
    }

    pub fn build(mut self) -> Result<ArmorItem> {
        if self.name.is_none() {
            return Err(eyre!("armor must have a name"));
        }

        if self.weight_class.is_none() {
            return Err(eyre!(
                "armor must be exactly one of Light, Medium, or Heavy"
            ));
        }

        let weight_tag = match self.weight_class.unwrap() {
            WeightClass::Light => ArmorTag::Light,
            WeightClass::Medium => ArmorTag::Medium,
            WeightClass::Heavy => ArmorTag::Heavy,
        };
        self.tags.insert(weight_tag);

        ArmorItem::new(self.name.unwrap(), self.tags, self.id, self.data_source)
    }
}
