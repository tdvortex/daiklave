use eyre::{eyre, Result};
use std::ops::Deref;

use crate::{
    data_source::{BookReference, DataSource},
    id::Id,
};

pub enum MagicMaterial {
    Orichalcum,
    Starmetal,
    Soulsteel,
    Moonsilver,
    RedJade,
    BlueJade,
    GreenJade,
    BlackJade,
    WhiteJade,
}

pub struct Hearthstone(ArtifactTraits);

impl Deref for Hearthstone {
    type Target = ArtifactTraits;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ArtifactTraits {
    id: Id,
    data_source: DataSource,
    merit_dots: u8,
    name: String,
    lore: Option<String>,
    powers: Option<String>,
    magic_material: Option<MagicMaterial>,
    hearthstone_slot_count: u8,
    hearthstones_slotted: Vec<Id>,
}

impl ArtifactTraits {
    pub fn custom(id: Id, creator_id: Id) -> ArtifactBuilder {
        ArtifactBuilder {
            id,
            data_source: DataSource::Custom(creator_id),
            merit_dots: None,
            name: None,
            lore: None,
            powers: None,
            magic_material: None,
            hearthstone_slots: 0,
            hearthstones_slotted: Vec::new(),
        }
    }

    pub fn from_book(id: Id, book_title: String, page_number: i16) -> ArtifactBuilder {
        ArtifactBuilder {
            id,
            data_source: DataSource::Book(BookReference {
                book_title,
                page_number,
            }),
            merit_dots: None,
            name: None,
            lore: None,
            powers: None,
            magic_material: None,
            hearthstone_slots: 0,
            hearthstones_slotted: Vec::new(),
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn data_source(&self) -> &DataSource {
        &self.data_source
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn lore(&self) -> Option<&str> {
        self.lore.as_deref()
    }

    pub fn powers(&self) -> Option<&str> {
        self.powers.as_deref()
    }

    pub fn try_into_hearthstone(self) -> Result<Hearthstone> {
        if self.magic_material.is_some() {
            Err(eyre!("Hearthstones have no magic material"))
        } else if self.hearthstone_slot_count > 0 || self.hearthstones_slotted.len() > 0 {
            Err(eyre!("Hearthstones cannot slot other hearthstones"))
        } else {
            Ok(Hearthstone(self))
        }
    }
}

pub struct ArtifactBuilder {
    id: Id,
    data_source: DataSource,
    merit_dots: Option<u8>,
    name: Option<String>,
    lore: Option<String>,
    powers: Option<String>,
    magic_material: Option<MagicMaterial>,
    hearthstone_slots: u8,
    hearthstones_slotted: Vec<Id>,
}

impl ArtifactBuilder {
    pub fn with_database_id(mut self, id: i32) -> Self {
        self.id = Id::Database(id);
        self
    }

    pub fn with_merit_dots(mut self, dots: u8) -> Self {
        self.merit_dots = Some(dots);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_lore(mut self, lore: String) -> Self {
        self.lore = Some(lore);
        self
    }

    pub fn with_magic_material(mut self, magic_material: MagicMaterial) -> Self {
        self.magic_material = Some(magic_material);
        self
    }

    pub fn with_powers(mut self, powers: String) -> Self {
        self.powers = Some(powers);
        self
    }

    pub fn with_hearthstone_slots(mut self, slots: u8) -> Self {
        self.hearthstone_slots = slots;
        self
    }

    pub fn with_slotted_hearthstone(mut self, hearthstone: &Hearthstone) -> Result<Self> {
        let id = (*hearthstone).id();

        if self.hearthstone_slots == 0 {
            Err(eyre!("Artifact has no hearthstone slots"))
        } else if self.hearthstones_slotted.contains(&id) {
            Ok(self)
        } else if self.hearthstones_slotted.len() == self.hearthstone_slots as usize {
            Err(eyre!("All hearthstone slots are filled"))
        } else {
            self.hearthstones_slotted.push(id);
            Ok(self)
        }
    }

    pub fn build(mut self) -> Result<ArtifactTraits> {
        if self.name.is_none() {
            return Err(eyre!("Artifacts must have a name"));
        }

        if self.merit_dots.is_none() {
            return Err(eyre!("Artifacts must have dots specified (even if zero)"));
        }

        self.hearthstones_slotted.sort();

        Ok(ArtifactTraits {
            id: self.id,
            data_source: self.data_source,
            merit_dots: self.merit_dots.unwrap(),
            name: self.name.unwrap(),
            lore: self.lore,
            powers: self.powers,
            magic_material: self.magic_material,
            hearthstone_slot_count: self.hearthstone_slots,
            hearthstones_slotted: self.hearthstones_slotted,
        })
    }
}
