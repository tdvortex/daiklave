use crate::{
    anima::AnimaLevel,
    charms::Spell,
    solar::{SolarTraits, SolarTraitsBuilder},
    sorcery::{MortalSorcererLevel, ShapingRitual, Sorcerer, TerrestrialCircleTraits},
};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExaltType {
    Mortal(MortalSorcererLevel),
    Solar(SolarTraits),
}

impl Default for ExaltType {
    fn default() -> Self {
        Self::Mortal(MortalSorcererLevel::default())
    }
}

impl Sorcerer for ExaltType {
    fn shaping_rituals(&self) -> Option<Vec<&crate::sorcery::ShapingRitual>> {
        match self {
            ExaltType::Mortal(mortal_sorcerer_level) => mortal_sorcerer_level.shaping_rituals(),
            ExaltType::Solar(solar_traits) => solar_traits.shaping_rituals(),
        }
    }

    fn spells(&self) -> Option<Vec<(&crate::charms::Spell, bool)>> {
        match self {
            ExaltType::Mortal(mortal_sorcerer_level) => mortal_sorcerer_level.spells(),
            ExaltType::Solar(solar_traits) => solar_traits.spells(),
        }
    }
}

struct MortalBuilder {
    sorcerer_level: MortalSorcererLevel,
}

impl MortalBuilder {
    pub fn with_terrestrial_circle_sorcery(
        mut self,
        shaping_ritual: ShapingRitual,
        control_spell: Spell,
    ) -> Result<Self> {
        match &mut self.sorcerer_level {
            MortalSorcererLevel::None => {
                self.sorcerer_level = MortalSorcererLevel::Terrestrial(
                    TerrestrialCircleTraits::new(shaping_ritual, control_spell)?,
                );
                Ok(self)
            }
            MortalSorcererLevel::Terrestrial(terrestrial_traits) => {
                terrestrial_traits.swap_shaping_ritual(shaping_ritual);
                let id = control_spell.id();
                terrestrial_traits.add_spell(control_spell)?;
                terrestrial_traits.swap_control_spell(id)?;
                Ok(self)
            }
        }
    }

    pub fn with_spell(mut self, spell: Spell) -> Result<Self> {
        self.sorcerer_level.add_spell(spell)?;
        Ok(self)
    }

    pub fn build(self) -> Result<ExaltType> {
        Ok(ExaltType::Mortal(self.sorcerer_level))
    }
}

enum ExaltTypeBuilder {
    Mortal(MortalBuilder),
    Solar(SolarTraitsBuilder),
}

impl ExaltTypeBuilder {
    pub(crate) fn with_essence_rating(self, rating: u8) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Mortal(_) => Err(eyre!("Mortals do not have Essence")),
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.with_essence_rating(rating)?,
            )),
        }
    }

    pub(crate) fn with_limit(self, limit_trigger: String, track: u8) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Mortal(_) => Err(eyre!("Mortals do not have Limit")),
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.with_limit(limit_trigger, track),
            )),
        }
    }

    pub(crate) fn with_anima_level(self, anima_level: AnimaLevel) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Mortal(_) => Err(eyre!("Mortals do not have Anima")),
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.with_anima_level(anima_level),
            )),
        }
    }

    pub(crate) fn with_terrestrial_circle_sorcery(
        self,
        shaping_ritual: ShapingRitual,
        control_spell: Spell,
    ) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Mortal(mortal_builder) => Ok(ExaltTypeBuilder::Mortal(
                mortal_builder.with_terrestrial_circle_sorcery(shaping_ritual, control_spell)?,
            )),
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.with_terrestrial_circle_sorcery(shaping_ritual, control_spell)?,
            )),
        }
    }

    pub(crate) fn with_spell(self, spell: Spell) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Mortal(mortal_builder) => {
                Ok(ExaltTypeBuilder::Mortal(mortal_builder.with_spell(spell)?))
            }
            ExaltTypeBuilder::Solar(solar_builder) => {
                Ok(ExaltTypeBuilder::Solar(solar_builder.with_spell(spell)?))
            }
        }
    }

    pub(crate) fn build(self) -> Result<ExaltType> {
        match self {
            ExaltTypeBuilder::Mortal(mortal_builder) => mortal_builder.build(),
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltType::Solar(solar_builder.build()?)),
        }
    }
}
