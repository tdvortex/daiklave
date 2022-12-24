use crate::{id::{WonderId, OwnedHearthstoneId, CharacterId}, data_source::{DataSource, BookReference}, artifact::MagicMaterial, hearthstone::OwnedHearthstone};
use eyre::{eyre, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Wonder {
    id: WonderId,
    data_source: DataSource,
    name: String,
    merit_dots: u8,
    lore: Option<String>,
    powers: Option<String>,
    magic_material: Option<MagicMaterial>,
    hearthstone_slots: Vec<Option<OwnedHearthstone>>,
    attunement_cost: u8,
    committed_motes: Option<(u8, u8)>,
}

impl Wonder {
    pub fn from_book(id: WonderId, book_title: String, page_number: i16) -> WonderBuilder {
        WonderBuilder { 
            id, 
            data_source: DataSource::Book(BookReference{book_title, page_number}), 
            name: None, 
            merit_dots: None, 
            lore: None, 
            powers: None, 
            magic_material: None, 
            hearthstone_slots: 0,
            attunement_cost: None }
    }

    pub fn custom(id: WonderId, creator_id: CharacterId) -> WonderBuilder {
        WonderBuilder { 
            id, 
            data_source: DataSource::Custom(creator_id), 
            name: None, 
            merit_dots: None, 
            lore: None, 
            powers: None, 
            magic_material: None, 
            hearthstone_slots: 0,
            attunement_cost: None }
    }


    pub fn slots(&self) -> (usize, usize) {
        self.hearthstone_slots.iter().fold((0, 0), |(filled, total), slot| {
            if slot.is_some() {
                (filled + 1, total + 1)
            } else {
                (filled, total + 1)
            }
        })
    }

    pub fn slot_heartstone(&mut self, stone: OwnedHearthstone) -> Result<()> {
        let open_slot = self.hearthstone_slots.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| eyre!("All slots are full"))?;
        *open_slot = Some(stone);
        Ok(())
    }

    pub fn unslot_hearthstone(&mut self, id: OwnedHearthstoneId) -> Result<OwnedHearthstone> {
        let slot = self.hearthstone_slots.iter_mut().find(|slot| if let Some(stone) = slot {
            stone.id() == id
        } else {
            false
        }).ok_or_else(|| eyre!("Hearthstone not slotted in this wonder"))?;

        Ok(std::mem::take(slot).unwrap())
    }

    pub fn attune(&mut self, peripheral: u8, personal: u8) -> Result<()> {
        if self.committed_motes.is_some() {
            Err(eyre!("Already attuned to Wonder"))
        } else if peripheral + personal != self.attunement_cost {
            Err(eyre!("Wrong attunement, needed {} but got {}", self.attunement_cost, peripheral + personal))
        } else {
            self.committed_motes = Some((peripheral, personal));
            Ok(())
        }
    }

    pub fn unattune(&mut self) -> Result<(u8, u8)> {
        if let Some((peripheral, personal)) = self.committed_motes {
            self.committed_motes = None;
            Ok((peripheral, personal))
        } else {
            Err(eyre!("Not attuned, cannot unattune"))
        }
    }
}

pub struct WonderBuilder {
    id: WonderId,
    data_source: DataSource,
    name: Option<String>,
    merit_dots: Option<u8>,
    lore: Option<String>,
    powers: Option<String>,
    magic_material: Option<MagicMaterial>,
    hearthstone_slots: usize,
    attunement_cost: Option<u8>,
}

impl WonderBuilder {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_merit_dots(mut self, dots: u8) -> Self {
        self.merit_dots = Some(dots);
        self
    }

    pub fn with_lore(mut self, lore: String) -> Self {
        self.lore = Some(lore);
        self
    }

    pub fn with_powers(mut self, powers: String) -> Self {
        self.powers = Some(powers);
        self
    }

    pub fn with_magic_material(mut self, magic_material: MagicMaterial) -> Self {
        self.magic_material = Some(magic_material);
        self
    }

    pub fn with_hearthstone_slots(mut self, count: usize) -> Self {
        self.hearthstone_slots = count;
        self
    }

    pub fn with_attunement_cost(mut self, cost: u8) -> Self {
        self.attunement_cost = Some(cost);
        self
    }

    pub fn build(self) -> Result<Wonder> {
        Ok(Wonder { 
            id: self.id, 
            data_source: self.data_source, 
            name: self.name.ok_or_else(|| eyre!("Wonders must have a name"))?, 
            merit_dots: self.merit_dots.ok_or_else(|| eyre!("Wonders must specify merit dots"))?, 
            lore: self.lore, 
            powers: self.powers, 
            magic_material: self.magic_material, 
            hearthstone_slots: vec![None; self.hearthstone_slots],
            attunement_cost: self.attunement_cost.ok_or_else(|| eyre!("Wonders must specify an attunement cost (even if it's zero)"))?, 
            committed_motes: None 
        })
    }
}