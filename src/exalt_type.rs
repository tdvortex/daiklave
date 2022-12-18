use crate::{
    anima::AnimaLevel,
    charms::{Spell, SolarCharm},
    solar::{SolarTraits, SolarTraitsBuilder, DawnTraits, ZenithTraits, TwilightTraits, EclipseTraits, NightTraits},
    sorcery::{MortalSorcererLevel, ShapingRitual, Sorcerer, TerrestrialCircleTraits}, abilities::AbilityNameNoSubskill,
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

impl ExaltType {
    fn builder() -> ExaltTypeBuilder {
        ExaltTypeBuilder::Mortal(MortalBuilder { sorcerer_level: MortalSorcererLevel::None })
    }
}

struct MortalBuilder {
    sorcerer_level: MortalSorcererLevel,
}

impl MortalBuilder {
    fn with_terrestrial_circle_sorcery(
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

    fn with_spell(mut self, spell: Spell) -> Result<Self> {
        self.sorcerer_level.add_spell(spell)?;
        Ok(self)
    }

    fn build(self) -> Result<ExaltType> {
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

    pub(crate) fn as_solar(self) -> Self {
        match self {
            ExaltTypeBuilder::Mortal(mortal_builder) => {
                let mut solar_builder = SolarTraits::builder();

                // Preserve sorcery if possible
                if let MortalSorcererLevel::Terrestrial(terrestrial_traits) = mortal_builder.sorcerer_level {
                    let shaping_ritual = terrestrial_traits.shaping_rituals().expect("Terrestrial sorcery must have one shaping ritual")[0].clone();
                    let (control_spell, other_spells) = terrestrial_traits.spells().expect("Terrestrial sorcery must have at least one spell").iter()
                        .fold((None, Vec::new()), |(mut control_spell, mut other_spells), (spell, is_control)| {
                            if *is_control {
                                control_spell = Some((*spell).clone());
                            } else {
                                other_spells.push((*spell).clone());
                            }
                            (control_spell, other_spells)
                        });

                    solar_builder = solar_builder.with_terrestrial_circle_sorcery(shaping_ritual, control_spell.unwrap()).expect("Terrestial Circle is valid for both mortals and Solars");
                    for other_spell in other_spells.into_iter() {
                        solar_builder = solar_builder.with_spell(other_spell).expect("Valid spell for mortal should be valid for solar as well");
                    }
                }

                ExaltTypeBuilder::Solar(solar_builder)
            }
            ExaltTypeBuilder::Solar(solar_builder) => ExaltTypeBuilder::Solar(solar_builder),
        }
    }

    pub(crate) fn as_dawn(self, dawn_traits: DawnTraits) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => {
                Ok(ExaltTypeBuilder::Solar(solar_builder.as_dawn(dawn_traits)))
            }
            _ => Err(eyre!("Must be Solar before being Dawn Caste")),
        }
    }

    pub(crate) fn as_zenith(self, zenith_traits: ZenithTraits) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => {
                Ok(ExaltTypeBuilder::Solar(solar_builder.as_zenith(zenith_traits)))
            }
            _ => Err(eyre!("Must be Solar before being Zenith Caste")),
        }
    }

    pub(crate) fn as_night(self, night_traits: NightTraits) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => {
                Ok(ExaltTypeBuilder::Solar(solar_builder.as_night(night_traits)))
            }
            _ => Err(eyre!("Must be Solar before being Night Caste")),
        }
    }

    pub(crate) fn as_twilight(self, twilight_traits: TwilightTraits) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => {
                Ok(ExaltTypeBuilder::Solar(solar_builder.as_twilight(twilight_traits)))
            }
            _ => Err(eyre!("Must be Solar before being Twilight Caste")),
        }
    }

    pub(crate) fn as_eclipse(self, eclipse_traits: EclipseTraits) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => {
                Ok(ExaltTypeBuilder::Solar(solar_builder.as_eclipse(eclipse_traits)))
            }
            _ => Err(eyre!("Must be Solar before being Eclipse Caste")),
        }
    }

    pub(crate) fn with_favored_ability(self, ability: AbilityNameNoSubskill) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Mortal(_) => Err(eyre!("Mortals do not have Favored abilities")),
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(solar_builder.with_favored_ability(ability)?)), 
        }
    }

    pub(crate) fn with_solar_charm(self, charm: SolarCharm) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(solar_builder.with_solar_charm_checked(charm)?)), 
            ExaltTypeBuilder::Mortal(_) => Err(eyre!("Non-Solars cannot use Solar Charms")),
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

    pub(crate) fn with_celestial_circle_sorcery(
        self,
        shaping_ritual: ShapingRitual,
        control_spell: Spell,
    ) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Mortal(mortal_builder) => Err(eyre!("Mortals may not use Celestial circle sorcery")),
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.with_celestial_circle_sorcery(shaping_ritual, control_spell)?,
            )),
        }
    }

    pub(crate) fn with_solar_circle_sorcery(
        self,
        shaping_ritual: ShapingRitual,
        control_spell: Spell,
    ) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Mortal(mortal_builder) => Err(eyre!("Mortals may not use Solar circle sorcery")),
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.with_solar_circle_sorcery(shaping_ritual, control_spell)?,
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
