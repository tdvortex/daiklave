use crate::{
    abilities::AbilityNameNoSubskill,
    anima::{AnimaEffect, AnimaLevel},
    character::ExperiencePoints,
    charms::{SolarCharm, Spell},
    essence::{Essence, MotePool},
    id::SpellId,
    solar::{
        DawnTraits, EclipseTraits, NightTraits, SolarTraits, SolarTraitsBuilder, TwilightTraits,
        ZenithTraits,
    },
    sorcery::{
        CelestialCircleTraits, MortalSorcererLevel, ShapingRitual, SolarCircleTraits,
        SolarSorcererLevel, Sorcerer, SorceryCircle, TerrestrialCircleTraits,
    },
};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExaltType {
    Mortal(MortalSorcererLevel),
    Solar(SolarTraits),
}

impl ExaltType {
    pub fn set_anima_level(&mut self, anima_level: AnimaLevel) -> Result<()> {
        match self {
            Self::Mortal(_) => Err(eyre!("Mortals do not have anima")),
            Self::Solar(solar_traits) => {
                solar_traits.anima_level = anima_level;
                Ok(())
            }
        }
    }

    pub fn set_essence_rating(&mut self, dots: u8) -> Result<()> {
        match self {
            Self::Mortal(_) => Err(eyre!("Mortals do not have Essence ratings")),
            Self::Solar(solar_traits) => {
                solar_traits.essence = Essence::solar(dots)?;
                Ok(())
            }
        }
    }

    pub fn set_peripheral_motes(&mut self, mote_pool: MotePool) -> Result<()> {
        match self {
            Self::Mortal(_) => Err(eyre!("Mortals do not have mote pools")),
            Self::Solar(solar_traits) => {
                solar_traits.essence.peripheral = mote_pool;
                Ok(())
            }
        }
    }

    pub fn set_personal_motes(&mut self, mote_pool: MotePool) -> Result<()> {
        match self {
            Self::Mortal(_) => Err(eyre!("Mortals do not have mote pools")),
            Self::Solar(solar_traits) => {
                solar_traits.essence.personal = mote_pool;
                Ok(())
            }
        }
    }

    pub fn set_solar_experience(&mut self, experience_points: ExperiencePoints) -> Result<()> {
        match self {
            Self::Mortal(_) => Err(eyre!("Only Solars have Solar Experience")),
            Self::Solar(solar_traits) => {
                solar_traits.set_solar_experience(experience_points);
                Ok(())
            }
        }
    }

    pub fn set_sorcery_level(
        &mut self,
        sorcery_circle: SorceryCircle,
        shaping_ritual: ShapingRitual,
        control_spell: Spell,
    ) -> Result<()> {
        match (self, sorcery_circle) {
            (ExaltType::Mortal(sorcery_level), SorceryCircle::Terrestrial) => {
                *sorcery_level = MortalSorcererLevel::Terrestrial(TerrestrialCircleTraits::new(
                    shaping_ritual,
                    control_spell,
                )?);
                Ok(())
            }
            (ExaltType::Mortal(_), _) => {
                Err(eyre!("Mortals can only be Terrestrial Circle sorcerers"))
            }
            (ExaltType::Solar(solar_traits), SorceryCircle::Terrestrial) => {
                match solar_traits.sorcery_mut() {
                    SolarSorcererLevel::None => {
                        *solar_traits.sorcery_mut() = SolarSorcererLevel::Terrestrial(
                            TerrestrialCircleTraits::new(shaping_ritual, control_spell)?,
                        );
                        Ok(())
                    }
                    SolarSorcererLevel::Terrestrial(old_terrestrial)
                    | SolarSorcererLevel::Celestial(old_terrestrial, _)
                    | SolarSorcererLevel::Solar(old_terrestrial, _, _) => {
                        let id = control_spell.id();
                        old_terrestrial.add_spell(control_spell)?;
                        old_terrestrial.swap_control_spell(id)?;
                        old_terrestrial.swap_shaping_ritual(shaping_ritual);

                        Ok(())
                    }
                }
            }
            (ExaltType::Solar(solar_traits), SorceryCircle::Celestial) => {
                if solar_traits.essence.rating() < 3 {
                    return Err(eyre!(
                        "Must be Essence 3 to take Celestial sorcery (Supernal doesn't apply)"
                    ));
                }

                match solar_traits.sorcery_mut() {
                    SolarSorcererLevel::None => {
                        Err(eyre!("Must be Terrestrial before becoming Celestial"))
                    }
                    SolarSorcererLevel::Terrestrial(terrestrial) => {
                        *solar_traits.sorcery_mut() = SolarSorcererLevel::Celestial(
                            terrestrial.clone(),
                            CelestialCircleTraits::new(shaping_ritual, control_spell)?,
                        );
                        Ok(())
                    }
                    SolarSorcererLevel::Celestial(_, old_celestial)
                    | SolarSorcererLevel::Solar(_, old_celestial, _) => {
                        let id = control_spell.id();
                        old_celestial.add_spell(control_spell)?;
                        old_celestial.swap_control_spell(id)?;
                        old_celestial.swap_shaping_ritual(shaping_ritual);

                        Ok(())
                    }
                }
            }
            (ExaltType::Solar(solar_traits), SorceryCircle::Solar) => {
                if solar_traits.essence.rating() < 5 {
                    return Err(eyre!(
                        "Must be Essence 5 to take Solar sorcery (Supernal doesn't apply)"
                    ));
                }

                match solar_traits.sorcery_mut() {
                    SolarSorcererLevel::None | SolarSorcererLevel::Terrestrial(_) => Err(eyre!(
                        "Must be Terrestrial and Celestial before becoming Solar"
                    )),
                    SolarSorcererLevel::Celestial(terrestrial, celestial) => {
                        *solar_traits.sorcery_mut() = SolarSorcererLevel::Solar(
                            terrestrial.clone(),
                            celestial.clone(),
                            SolarCircleTraits::new(shaping_ritual, control_spell)?,
                        );
                        Ok(())
                    }
                    SolarSorcererLevel::Solar(_, _, old_solar) => {
                        let id = control_spell.id();
                        old_solar.add_spell(control_spell)?;
                        old_solar.swap_control_spell(id)?;
                        old_solar.swap_shaping_ritual(shaping_ritual);

                        Ok(())
                    }
                }
            }
        }
    }

    pub fn remove_sorcery_circle(&mut self) -> Result<()> {
        match self {
            ExaltType::Mortal(mortal_sorcerer_level) => {
                if let MortalSorcererLevel::Terrestrial(_) = mortal_sorcerer_level {
                    *mortal_sorcerer_level = MortalSorcererLevel::None;
                    Ok(())
                } else {
                    Err(eyre!("No sorcery circles to remove"))
                }
            }
            ExaltType::Solar(solar_traits) => match solar_traits.sorcery_mut() {
                SolarSorcererLevel::None => Err(eyre!("No sorcery circles to remove")),
                SolarSorcererLevel::Terrestrial(_) => {
                    *solar_traits.sorcery_mut() = SolarSorcererLevel::None;
                    Ok(())
                }
                SolarSorcererLevel::Celestial(terrestrial_traits, _) => {
                    *solar_traits.sorcery_mut() =
                        SolarSorcererLevel::Terrestrial(terrestrial_traits.clone());
                    Ok(())
                }
                SolarSorcererLevel::Solar(terrestrial_traits, celestial_traits, _) => {
                    *solar_traits.sorcery_mut() = SolarSorcererLevel::Celestial(
                        terrestrial_traits.clone(),
                        celestial_traits.clone(),
                    );
                    Ok(())
                }
            },
        }
    }

    pub fn add_spell(&mut self, spell: Spell) -> Result<()> {
        match self {
            ExaltType::Mortal(mortal_sorcerer_traits) => mortal_sorcerer_traits.add_spell(spell),
            ExaltType::Solar(solar_traits) => solar_traits.sorcery_mut().add_spell(spell),
        }
    }

    pub fn remove_spell(&mut self, id: SpellId) -> Result<()> {
        match self {
            ExaltType::Mortal(mortal_sorcerer_traits) => mortal_sorcerer_traits.remove_spell(id),
            ExaltType::Solar(solar_traits) => solar_traits.sorcery_mut().remove_spell(id),
        }
    }

    pub fn set_limit_trigger(&mut self, trigger: String) -> Result<()> {
        match self {
            ExaltType::Mortal(_) => Err(eyre!("Mortals do not have Limit")),
            ExaltType::Solar(solar_traits) => {
                solar_traits.limit.limit_trigger = trigger;
                Ok(())
            }
        }
    }

    pub fn set_limit_track(&mut self, value: u8) -> Result<()> {
        match self {
            ExaltType::Mortal(_) => Err(eyre!("Mortals do not have Limit")),
            ExaltType::Solar(solar_traits) => {
                solar_traits.limit.track = value;
                Ok(())
            }
        }
    }
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

#[derive(Debug, Default)]
pub struct MortalBuilder {
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

#[derive(Debug)]
pub(crate) enum ExaltTypeBuilder {
    Mortal(MortalBuilder),
    Solar(SolarTraitsBuilder),
}

impl Default for ExaltTypeBuilder {
    fn default() -> Self {
        Self::Mortal(MortalBuilder::default())
    }
}

impl ExaltTypeBuilder {
    pub(crate) fn essence(&self) -> Option<&Essence> {
        match self {
            ExaltTypeBuilder::Mortal(_) => None,
            ExaltTypeBuilder::Solar(solar_builder) => Some(solar_builder.essence()),
        }
    }

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

    pub(crate) fn with_anima_effect(self, effect: AnimaEffect) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Mortal(_) => Err(eyre!("Mortals do not have Anima")),
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.with_anima_effect(effect)?,
            )),
        }
    }

    pub(crate) fn into_solar(self) -> Self {
        match self {
            ExaltTypeBuilder::Mortal(mortal_builder) => {
                let mut solar_builder = SolarTraits::builder();

                // Preserve sorcery if possible
                if let MortalSorcererLevel::Terrestrial(terrestrial_traits) =
                    mortal_builder.sorcerer_level
                {
                    let shaping_ritual = terrestrial_traits
                        .shaping_rituals()
                        .expect("Terrestrial sorcery must have one shaping ritual")[0]
                        .clone();
                    let (control_spell, other_spells) = terrestrial_traits
                        .spells()
                        .expect("Terrestrial sorcery must have at least one spell")
                        .iter()
                        .fold(
                            (None, Vec::new()),
                            |(mut control_spell, mut other_spells), (spell, is_control)| {
                                if *is_control {
                                    control_spell = Some((*spell).clone());
                                } else {
                                    other_spells.push((*spell).clone());
                                }
                                (control_spell, other_spells)
                            },
                        );

                    solar_builder = solar_builder
                        .with_terrestrial_circle_sorcery(shaping_ritual, control_spell.unwrap())
                        .expect("Terrestial Circle is valid for both mortals and Solars");
                    for other_spell in other_spells.into_iter() {
                        solar_builder = solar_builder
                            .with_spell(other_spell)
                            .expect("Valid spell for mortal should be valid for solar as well");
                    }
                }

                ExaltTypeBuilder::Solar(solar_builder)
            }
            ExaltTypeBuilder::Solar(solar_builder) => ExaltTypeBuilder::Solar(solar_builder),
        }
    }

    pub(crate) fn into_dawn(self, dawn_traits: DawnTraits) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.into_dawn(dawn_traits),
            )),
            _ => Err(eyre!("Must be Solar before being Dawn Caste")),
        }
    }

    pub(crate) fn into_zenith(self, zenith_traits: ZenithTraits) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.into_zenith(zenith_traits),
            )),
            _ => Err(eyre!("Must be Solar before being Zenith Caste")),
        }
    }

    pub(crate) fn into_night(self, night_traits: NightTraits) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.into_night(night_traits),
            )),
            _ => Err(eyre!("Must be Solar before being Night Caste")),
        }
    }

    pub(crate) fn into_twilight(self, twilight_traits: TwilightTraits) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.into_twilight(twilight_traits),
            )),
            _ => Err(eyre!("Must be Solar before being Twilight Caste")),
        }
    }

    pub(crate) fn into_eclipse(self, eclipse_traits: EclipseTraits) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.into_eclipse(eclipse_traits),
            )),
            _ => Err(eyre!("Must be Solar before being Eclipse Caste")),
        }
    }

    pub(crate) fn with_favored_ability(self, ability: AbilityNameNoSubskill) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Mortal(_) => Err(eyre!("Mortals do not have Favored abilities")),
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.with_favored_ability(ability)?,
            )),
        }
    }

    pub(crate) fn with_solar_charm(self, charm: SolarCharm) -> Result<Self> {
        match self {
            ExaltTypeBuilder::Solar(solar_builder) => Ok(ExaltTypeBuilder::Solar(
                solar_builder.with_solar_charm_checked(charm)?,
            )),
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
            ExaltTypeBuilder::Mortal(_) => {
                Err(eyre!("Mortals may not use Celestial circle sorcery"))
            }
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
            ExaltTypeBuilder::Mortal(_) => Err(eyre!("Mortals may not use Solar circle sorcery")),
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
