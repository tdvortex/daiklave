pub(crate) mod create;
pub(crate) mod tables;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::abilities::AbilityNameNoSubskill;
use crate::attributes::AttributeName;
use crate::id::Id;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum PrerequisiteType {
    Ability(AbilityPrerequisite),
    Attribute(AttributePrerequisite),
    Essence(u8),
    Charm(i32),
    ExaltType(ExaltTypePrerequisite),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct AbilityPrerequisite {
    pub ability_name: AbilityNameNoSubskill,
    pub subskill: Option<String>,
    pub dots: u8,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct AttributePrerequisite {
    pub attribute_name: AttributeName,
    pub dots: u8,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum ExaltTypePrerequisite {
    Solar,
    Lunar,
    DragonBlooded,
    Spirit,
    SpiritOrEclipse,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrerequisiteSet {
    id: Id,
    prerequisites: Vec<Prerequisite>,
}

impl PrerequisiteSet {
    pub fn create() -> PrerequisiteSetBuilder {
        PrerequisiteSetBuilder::default()
    }

    pub fn id(&self) -> Id {
        self.id
    }
}

impl Deref for PrerequisiteSet {
    type Target = Vec<Prerequisite>;

    fn deref(&self) -> &Self::Target {
        &self.prerequisites
    }
}

#[derive(Debug, Default)]
pub struct PrerequisiteSetBuilder {
    id: Id,
    prerequisites: Vec<Prerequisite>,
}

impl PrerequisiteSetBuilder {
    pub fn with_database_id(mut self, id: i32) -> Self {
        self.id = Id::Database(id);
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
