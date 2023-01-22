use std::collections::{hash_map::Entry, HashMap};

use crate::{
    sorcery::{
        archetype::SorceryArchetypeMeritId,
        circles::{
            solar::{sorcerer::SolarCircleSorcerer, AddSolarSorcery},
            sorcery_circle::SorceryCircle,
            terrestrial::TerrestrialSpell,
        },
        spell::{Spell, SpellId},
        ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId,
        SorceryArchetypeMerit, SorceryError,
    },
    CharacterMutationError,
};

use super::{sorcerer_memo::CelestialCircleSorcererMemo, spell::CelestialSpell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CelestialCircleSorcerer<'source> {
    pub(crate) archetypes: HashMap<
        SorceryArchetypeId,
        (
            &'source SorceryArchetype,
            HashMap<SorceryArchetypeMeritId, &'source SorceryArchetypeMerit>,
        ),
    >,
    pub(crate) circle_archetypes: [SorceryArchetypeId; 2],
    pub(crate) shaping_ritual_ids: [ShapingRitualId; 2],
    pub(crate) shaping_rituals: [&'source ShapingRitual; 2],
    pub(crate) terrestrial_control_spell_id: SpellId,
    pub(crate) terrestrial_control_spell: &'source TerrestrialSpell,
    pub(crate) terrestrial_spells: HashMap<SpellId, &'source TerrestrialSpell>,
    pub(crate) celestial_control_spell_id: SpellId,
    pub(crate) celestial_control_spell: &'source CelestialSpell,
    pub(crate) celestial_spells: HashMap<SpellId, &'source CelestialSpell>,
}

impl<'view, 'source> CelestialCircleSorcerer<'source> {
    pub fn as_memo(&self) -> CelestialCircleSorcererMemo {
        CelestialCircleSorcererMemo {
            archetypes: self
                .archetypes
                .iter()
                .map(|(k, (archetype, merits))| {
                    (
                        *k,
                        (
                            (*archetype).to_owned(),
                            merits.iter().map(|(k, v)| (*k, (*v).to_owned())).collect(),
                        ),
                    )
                })
                .collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: { self.shaping_rituals.map(|ptr| ptr.to_owned()) },
            terrestrial_control_spell_id: self.terrestrial_control_spell_id,
            terrestrial_control_spell: self.terrestrial_control_spell.to_owned(),
            terrestrial_spells: self
                .terrestrial_spells
                .iter()
                .map(|(k, v)| (*k, (*v).to_owned()))
                .collect(),
            celestial_control_spell_id: self.celestial_control_spell_id,
            celestial_control_spell: self.celestial_control_spell.to_owned(),
            celestial_spells: self
                .celestial_spells
                .iter()
                .map(|(k, v)| (*k, (*v).to_owned()))
                .collect(),
        }
    }

    pub fn archetype(
        &'view self,
        id: SorceryArchetypeId,
    ) -> Option<(
        &'source SorceryArchetype,
        &'view HashMap<SorceryArchetypeMeritId, &'source SorceryArchetypeMerit>,
    )> {
        if self.circle_archetypes.contains(&id) {
            self.archetypes
                .get(&id)
                .map(|(archetype, merits)| (*archetype, merits))
        } else {
            None
        }
    }

    pub fn shaping_ritual(
        &self,
        circle: SorceryCircle,
    ) -> Option<(ShapingRitualId, &'source ShapingRitual)> {
        match circle {
            SorceryCircle::Terrestrial => {
                Some((self.shaping_ritual_ids[0], self.shaping_rituals[0]))
            }
            SorceryCircle::Celestial => Some((self.shaping_ritual_ids[1], self.shaping_rituals[1])),
            SorceryCircle::Solar => None,
        }
    }

    pub fn control_spell(&self, circle: SorceryCircle) -> Option<(SpellId, Spell<'source>)> {
        match circle {
            SorceryCircle::Terrestrial => Some((
                self.terrestrial_control_spell_id,
                Spell::Terrestrial(self.terrestrial_control_spell),
            )),
            SorceryCircle::Celestial => Some((
                self.celestial_control_spell_id,
                Spell::Celestial(self.celestial_control_spell),
            )),
            SorceryCircle::Solar => None,
        }
    }

    pub fn upgrade(
        &self,
        add_solar: &'source AddSolarSorcery,
    ) -> Result<SolarCircleSorcerer<'source>, CharacterMutationError> {
        if add_solar.shaping_ritual_id == self.shaping_ritual_ids[0]
            || add_solar.shaping_ritual_id == self.shaping_ritual_ids[1]
        {
            return Err(CharacterMutationError::SorceryError(
                SorceryError::DuplicateShapingRitual,
            ));
        }

        let mut archetypes = self.archetypes.clone();

        if let Entry::Vacant(e) = archetypes.entry(add_solar.archetype_id) {
            if let Some(new_archetype) = &add_solar.archetype {
                e.insert((new_archetype, HashMap::new()));
            } else {
                return Err(CharacterMutationError::SorceryError(
                    SorceryError::MissingArchetype,
                ));
            }
        }

        Ok(SolarCircleSorcerer {
            archetypes,
            circle_archetypes: [
                self.circle_archetypes[0],
                self.circle_archetypes[1],
                add_solar.archetype_id,
            ],
            shaping_ritual_ids: [
                self.shaping_ritual_ids[0],
                self.shaping_ritual_ids[1],
                add_solar.shaping_ritual_id,
            ],
            shaping_rituals: [
                self.shaping_rituals[0],
                self.shaping_rituals[1],
                &add_solar.shaping_ritual,
            ],
            terrestrial_control_spell_id: self.terrestrial_control_spell_id,
            terrestrial_control_spell: self.terrestrial_control_spell,
            terrestrial_spells: self.terrestrial_spells.clone(),
            celestial_control_spell_id: self.celestial_control_spell_id,
            celestial_control_spell: self.celestial_control_spell,
            celestial_spells: self.celestial_spells.clone(),
            solar_control_spell_id: add_solar.control_spell_id,
            solar_control_spell: &add_solar.control_spell,
            solar_spells: HashMap::new(),
        })
    }
}

impl<'view, 'source> From<&'view SolarCircleSorcerer<'source>>
    for CelestialCircleSorcerer<'source>
{
    fn from(solar: &'view SolarCircleSorcerer<'source>) -> Self {
        let mut archetypes = solar.archetypes.clone();
        if solar.circle_archetypes[2] != solar.circle_archetypes[0]
            && solar.circle_archetypes[2] != solar.circle_archetypes[1]
        {
            archetypes.remove(&solar.circle_archetypes[2]);
        }

        Self {
            archetypes,
            circle_archetypes: [solar.circle_archetypes[0], solar.circle_archetypes[1]],
            shaping_ritual_ids: [solar.shaping_ritual_ids[0], solar.shaping_ritual_ids[1]],
            shaping_rituals: [solar.shaping_rituals[0], solar.shaping_rituals[1]],
            terrestrial_control_spell_id: solar.terrestrial_control_spell_id,
            terrestrial_control_spell: solar.terrestrial_control_spell,
            terrestrial_spells: solar.terrestrial_spells.clone(),
            celestial_control_spell_id: solar.celestial_control_spell_id,
            celestial_control_spell: solar.celestial_control_spell,
            celestial_spells: solar.celestial_spells.clone(),
        }
    }
}
