pub(crate) mod create;
pub(crate) mod tables;
use std::ops::Deref;

use crate::abilities::AbilityNameNoSubskill;
use crate::attributes::AttributeName;
use crate::character::Character;

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
    prerequisite_type: PrerequisiteType,
}

impl Prerequisite {
    pub fn prerequisite_type(&self) -> &PrerequisiteType {
        &self.prerequisite_type
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
    pub ability_name: AbilityNameNoSubskill,
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
    pub fn create() -> PrerequisiteSetBuilder {
        PrerequisiteSetBuilder::default()
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
    fn _meets_prerequisite(&self, prerequisite: &Prerequisite) -> bool {
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

    fn _meets_prerequisite_set(&self, prerequisite_set: &PrerequisiteSet) -> bool {
        prerequisite_set.is_empty()
            || prerequisite_set
                .iter()
                .all(|prerequisite| self._meets_prerequisite(prerequisite))
    }

    pub(crate) fn _meets_any_prerequisite_set(&self, prerequisite_sets: &[PrerequisiteSet]) -> bool {
        prerequisite_sets.is_empty()
            || prerequisite_sets
                .iter()
                .any(|prerequisite_set| self._meets_prerequisite_set(prerequisite_set))
    }
}

#[derive(Debug, Default)]
pub struct PrerequisiteSetBuilder {
    id: Option<i32>,
    prerequisites: Vec<Prerequisite>,
}

impl PrerequisiteSetBuilder {
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    pub fn requiring_ability(
        mut self,
        ability_name_no_subskill: AbilityNameNoSubskill,
        dots: u8,
    ) -> Self {
        self.prerequisites.push(Prerequisite {
            prerequisite_type: PrerequisiteType::Ability(AbilityPrerequisite {
                ability_name: ability_name_no_subskill,
                subskill: None,
                dots,
            }),
        });
        self
    }

    pub fn requiring_craft_focus(mut self, focus: String, dots: u8) -> Self {
        self.prerequisites.push(Prerequisite {
            prerequisite_type: PrerequisiteType::Ability(AbilityPrerequisite {
                ability_name: AbilityNameNoSubskill::Craft,
                subskill: Some(focus),
                dots,
            }),
        });
        self
    }

    pub fn requiring_martial_arts_style(mut self, style: String, dots: u8) -> Self {
        self.prerequisites.push(Prerequisite {
            prerequisite_type: PrerequisiteType::Ability(AbilityPrerequisite {
                ability_name: AbilityNameNoSubskill::MartialArts,
                subskill: Some(style),
                dots,
            }),
        });
        self
    }

    pub fn requiring_attribute(mut self, attribute_name: AttributeName, dots: u8) -> Self {
        self.prerequisites.push(Prerequisite {
            prerequisite_type: PrerequisiteType::Attribute(AttributePrerequisite {
                attribute_name,
                dots,
            }),
        });
        self
    }

    pub fn requiring_essence_rating(mut self, dots: u8) -> Self {
        self.prerequisites.push(Prerequisite {
            prerequisite_type: PrerequisiteType::Essence(dots),
        });
        self
    }

    pub fn requiring_charm(mut self, charm_id: i32) -> Self {
        self.prerequisites.push(Prerequisite {
            prerequisite_type: PrerequisiteType::Charm(charm_id),
        });
        self
    }

    pub fn requiring_solar(mut self) -> Self {
        self.prerequisites.push(Prerequisite {
            prerequisite_type: PrerequisiteType::ExaltType(ExaltTypePrerequisite::Solar),
        });
        self
    }

    pub fn requiring_lunar(mut self) -> Self {
        self.prerequisites.push(Prerequisite {
            prerequisite_type: PrerequisiteType::ExaltType(ExaltTypePrerequisite::Lunar),
        });
        self
    }

    pub fn requiring_dragon_blooded(mut self) -> Self {
        self.prerequisites.push(Prerequisite {
            prerequisite_type: PrerequisiteType::ExaltType(ExaltTypePrerequisite::DragonBlooded),
        });
        self
    }

    pub fn requiring_spirit(mut self, eclipse_keyword: bool) -> Self {
        let maybe_with_eclipse = if eclipse_keyword {
            ExaltTypePrerequisite::SpiritOrEclipse
        } else {
            ExaltTypePrerequisite::Spirit
        };
        self.prerequisites.push(Prerequisite {
            prerequisite_type: PrerequisiteType::ExaltType(maybe_with_eclipse),
        });
        self
    }

    pub fn build(self) -> PrerequisiteSet {
        PrerequisiteSet {
            id: self.id,
            prerequisites: self.prerequisites,
        }
    }
}
