use std::ops::Deref;

use crate::character::Character;

use super::{abilities::AbilityNameNoFocus, attributes::AttributeName};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PrerequisiteType {
    Ability(AbilityPrerequisite),
    Attribute(AttributePrerequisite),
    Essence(u8),
    Charm(i32),
    ExaltType(ExaltTypePrerequisite),
}

#[derive(Debug, Clone)]
pub struct Prerequisite {
    id: Option<i32>,
    prerequisite_type: PrerequisiteType,
}

impl Prerequisite {
    pub fn new(prerequisite_type: PrerequisiteType, id: Option<i32>) -> Self {
        Self {
            id,
            prerequisite_type,
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }
}

impl Deref for Prerequisite {
    type Target = PrerequisiteType;

    fn deref(&self) -> &Self::Target {
        &self.prerequisite_type
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbilityPrerequisite {
    pub ability_name: AbilityNameNoFocus,
    pub subskill: Option<String>,
    pub dots: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AttributePrerequisite {
    pub attribute_name: AttributeName,
    pub dots: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExaltTypePrerequisite {
    Solar,
    Lunar,
    DragonBlooded,
    Spirit,
    SpiritOrEclipse,
}

#[derive(Debug, Clone)]
pub struct PrerequisiteSet {
    id: Option<i32>,
    prerequisites: Vec<Prerequisite>,
}

impl PrerequisiteSet {
    pub fn new(prerequisites: Vec<Prerequisite>, id: Option<i32>) -> Self {
        Self { prerequisites, id }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }
}

impl Deref for PrerequisiteSet {
    type Target = Vec<Prerequisite>;

    fn deref(&self) -> &Self::Target {
        &self.prerequisites
    }
}

impl Character {
    fn meets_prerequisite(&self, prerequisite: &Prerequisite) -> bool {
        match prerequisite.deref() {
            PrerequisiteType::Ability(ability_prerequisite) => {
                self.abilities.meets_prerequisite(ability_prerequisite)
            }
            PrerequisiteType::Attribute(attribute_prerequisite) => {
                self.attributes.meets_prerequisite(attribute_prerequisite)
            }
            PrerequisiteType::Essence(_) => false,
            PrerequisiteType::Charm(_) => false,
            PrerequisiteType::ExaltType(exalt_type) => match exalt_type {
                ExaltTypePrerequisite::Solar => false,
                ExaltTypePrerequisite::Lunar => false,
                ExaltTypePrerequisite::DragonBlooded => false,
                ExaltTypePrerequisite::Spirit => false,
                ExaltTypePrerequisite::SpiritOrEclipse => false,
            },
        }
    }

    fn meets_prerequisite_set(&self, prerequisite_set: &PrerequisiteSet) -> bool {
        prerequisite_set
            .iter()
            .all(|prerequisite| self.meets_prerequisite(prerequisite))
    }

    pub fn meets_any_prerequisite_set(&self, prerequisite_sets: &[PrerequisiteSet]) -> bool {
        prerequisite_sets
            .iter()
            .any(|prerequisite_set| self.meets_prerequisite_set(prerequisite_set))
    }
}
