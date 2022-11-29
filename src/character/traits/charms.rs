use eyre::{eyre, Result};
use slab::Slab;

use crate::character::traits::prerequisite::PrerequisiteSet;

use super::{experience::CraftingExperienceType, health::DamageLevel};

pub struct BookReference {
    pub book_name: String,
    pub page_number: usize,
}

pub struct Charms {
    active: Vec<usize>,
    known: Slab<Charm>,
}

impl Charms {
    pub fn active_iter(&self) -> impl Iterator<Item = (usize, &Charm)> {
        self.active
            .iter()
            .map(|&key| (key, self.known.get(key).unwrap()))
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &Charm)> {
        self.known.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut Charm)> {
        self.known.iter_mut()
    }

    pub fn add_charm(&mut self, charm: Charm) -> usize {
        self.known.insert(charm)
    }

    pub fn get(&self, key: usize) -> Result<&Charm> {
        self.known
            .get(key)
            .ok_or_else(|| eyre!("charm {} not found", key))
    }

    pub fn get_mut(&mut self, key: usize) -> Result<&mut Charm> {
        self.known
            .get_mut(key)
            .ok_or_else(|| eyre!("charm {} not found", key))
    }

    pub fn remove_charm(&mut self, key: usize) -> Result<()> {
        if !self.known.contains(key) {
            Err(eyre!("charm {} not found", key))
        } else {
            self.active.retain(|&x| x != key);
            self.known.remove(key);
            Ok(())
        }
    }
}

pub struct Charm {
    pub cost: Vec<CostType>,
    pub prerequisites: PrerequisiteSet,
    pub action_type: ActionType,
    pub keywords: Vec<CharmKeyword>,
    pub duration: Duration,
    pub name: String,
    pub reference: Option<BookReference>,
    pub summary: String,
    pub full_description: String,
}

pub enum CostType {
    Motes(u8),
    Willpower(u8),
    Health(DamageLevel, u8),
    AnimaLevels(u8),
    Initiative(u8),
    Experience(u8),
    CraftingExperience(CraftingExperienceType),
    SorcerousMotes(u8),
}

pub enum CharmKeyword {
    Air,
    Aggravated,
    Archetype,
    Aura,
    Balanced,
    Bridge,
    Clash,
    Counterattack,
    DecisiveOnly,
    Dual,
    Excellency,
    Fire,
    Earth,
    Mute,
    Pilot,
    Protean,
    Psyche,
    Perilous,
    Salient,
    Signature,
    Stackable,
    Uniform,
    Water,
    WitheringOnly,
    Wood,
    WrittenOnly,
}

pub enum Duration {
    Instant,
    Tick,
    Turn,
    Round,
    Scene,
    Indefinite,
    Permanent,
    Special(String),
}

pub enum ActionType {
    Simple,
    Supplemental,
    Reflexive,
    Permanent,
}

pub struct Spell(Charm);
