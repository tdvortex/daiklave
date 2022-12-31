use std::{ops::Deref, collections::HashMap};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    book_reference::BookReference,
    charms::{CharmCost, CharmKeyword},
    id::UniqueId, Character, exalt_state::{ExaltState, exalt::{Exalt, exalt_type::{ExaltType, solar::{Solar, SolarView}, ExaltTypeView}, ExaltView}, ExaltStateView}, CharacterView,
};

/// A sorcery archetype, representing one path to sorcerous knowledge. This
/// unlocks various shaping rituals as well as unique merits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SorceryArchetype {
    name: String,
    book_reference: Option<BookReference>,
    description: String,
}

impl SorceryArchetype {
    /// Creates a new SorceryArchetype.
    pub fn new(name: String, book_reference: Option<BookReference>, description: String) -> Self {
        Self {
            name,
            book_reference,
            description,
        }
    }

    /// The name of the archetype
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The book reference for the archetype, if any
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    /// A description of the archetype
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}

/// A unique Id for a Sorcery Archetype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SorceryArchetypeId(pub UniqueId);

impl Deref for SorceryArchetypeId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A shaping ritual, one method that a sorcerous archetype might use to
/// generate Sorcerous Motes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapingRitual {
    archetype_id: SorceryArchetypeId,
    book_reference: Option<BookReference>,
    description: String,
}

impl ShapingRitual {
    /// Create a new ShapingRitual
    pub fn new(
        archetype_id: SorceryArchetypeId,
        book_reference: Option<BookReference>,
        description: String,
    ) -> Self {
        Self {
            archetype_id,
            book_reference,
            description,
        }
    }

    /// The Id of the SorceryArchetype associated with this ritual
    pub fn archetype_id(&self) -> SorceryArchetypeId {
        self.archetype_id
    }

    /// The book reference for the shaping ritual, if any
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    /// A description of the shaping ritual
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}

/// A unique Id for a ShapingRitual
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShapingRitualId(pub UniqueId);

impl Deref for ShapingRitualId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Sorcery Spell. Note that this is almost never used directly; instead,
/// it is typically wrapped in TerrestrialSpell, CelestialSpell, or SolarSpell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spell {
    name: String,
    book_reference: Option<BookReference>,
    costs: Vec<CharmCost>,
    keywords: Vec<CharmKeyword>,
    duration: String,
    description: String,
}

impl Spell {
    /// Creates a new Spell
    pub fn new(
        name: String,
        book_reference: Option<BookReference>,
        costs: Vec<CharmCost>,
        keywords: Vec<CharmKeyword>,
        duration: String,
        description: String,
    ) -> Self {
        Self {
            name,
            book_reference,
            costs,
            keywords,
            duration,
            description,
        }
    }

    /// The Spell's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The book reference for the spell, if any
    pub fn book_reference(&self) -> Option<&BookReference> {
        self.book_reference.as_ref()
    }

    /// The costs required to cast the spell
    pub fn costs(&self) -> &[CharmCost] {
        &self.costs
    }

    /// The keywords of this spell.
    pub fn keywords(&self) -> &[CharmKeyword] {
        &self.keywords
    }

    /// The duration of the spell effect after casting.
    pub fn duration(&self) -> &str {
        self.duration.as_str()
    }

    /// A description of the spell.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}

/// A unique Id for a Spell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpellId(pub UniqueId);

impl Deref for SpellId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Spell of the first (Terrestrial) Circle. Derefs to Spell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerrestrialSpell(Spell);

impl TerrestrialSpell {
    /// Wraps a Spell as a TerrestrialSpell
    pub fn from_spell(spell: Spell) -> Self {
        Self(spell)
    }
}

impl Deref for TerrestrialSpell {
    type Target = Spell;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Spell of the second (Celestial) Circle. Derefs to Spell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CelestialSpell(Spell);

impl CelestialSpell {
    /// Wraps a Spell as a CelestialSpell
    pub fn from_spell(spell: Spell) -> Self {
        Self(spell)
    }
}

impl Deref for CelestialSpell {
    type Target = Spell;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Spell of the third (Solar) Circle. Derefs to Spell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarSpell(Spell);

impl SolarSpell {
    /// Wraps a Spell as a CelestialSpell
    pub fn from_spell(spell: Spell) -> Self {
        Self(spell)
    }
}

impl Deref for SolarSpell {
    type Target = Spell;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'char> Character {
    pub fn sorcery(&'char self) -> Option<Sorcery<'char>> {
        self.exalt_state.sorcery()
    }
}

impl<'view, 'source> CharacterView<'source> {
    pub fn sorcery(&'view self) -> Option<SorceryView<'view, 'source>> {
        self.exalt_state.sorcery()
    }
}

pub struct Sorcery<'char>(SorcerySwitch<'char>);

pub struct SorceryView<'view, 'source>(SorceryViewSwitch<'view, 'source>);

enum SorcerySwitch<'char> {
    Mortal(&'char TerrestrialCircleSorcerer),
    Exalt(ExaltSorcerySwitch<'char>),
}

enum SorceryViewSwitch<'view, 'source> {
    Mortal(&'view TerrestrialCircleSorcererView<'source>),
    Exalt(ExaltSorceryViewSwitch<'view, 'source>),
}

#[derive(Debug, Error)]
pub enum SorceryError {
    #[error("Missing an archetype for a shaping ritual")]
    MissingArchetype,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TerrestrialCircleSorcerer {
    archetype_id: SorceryArchetypeId,
    archetype: SorceryArchetype,
    shaping_ritual_id: ShapingRitualId,
    shaping_ritual: ShapingRitual,
    control_spell_id: SpellId,
    control_spell: TerrestrialSpell,
    other_spells: HashMap<SpellId, TerrestrialSpell>,
}

impl TerrestrialCircleSorcerer {
    pub fn new(
        archetype_id: SorceryArchetypeId,
        archetype: SorceryArchetype,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: ShapingRitual,
        control_spell_id: SpellId,
        control_spell: TerrestrialSpell,
    ) -> Result<Self, SorceryError> {
        if shaping_ritual.archetype_id != archetype_id {
            return Err(SorceryError::MissingArchetype);
        }

        Ok(Self {
            archetype_id,
            archetype,
            shaping_ritual_id,
            shaping_ritual,
            control_spell_id,
            control_spell,
            other_spells: HashMap::new(),
        })
    }
}

impl<'char> TerrestrialCircleSorcerer {
    fn as_view(&'char self) -> TerrestrialCircleSorcererView<'char> {
        TerrestrialCircleSorcererView {
            archetype_id: self.archetype_id,
            archetype: &self.archetype,
            shaping_ritual_id: self.shaping_ritual_id,
            shaping_ritual: &self.shaping_ritual,
            control_spell_id: self.control_spell_id,
            control_spell: &self.control_spell,
            other_spells: self.other_spells.iter().map(|(k, v)| (*k, v)).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CelestialCircleSorcerer {
    archetypes: HashMap<SorceryArchetypeId, SorceryArchetype>,
    circle_archetypes: [SorceryArchetypeId; 2],
    shaping_ritual_ids: [ShapingRitualId; 2],
    shaping_rituals: [ShapingRitual; 2],
    terrestrial_control_spell_id: SpellId,
    terrestrial_control_spell: TerrestrialSpell,
    terrestrial_spells: HashMap<SpellId, TerrestrialSpell>,
    celestial_control_spell_id: SpellId,
    celestial_control_spell: CelestialSpell,
    celestial_spells: HashMap<SpellId, CelestialSpell>,
}

impl<'char> CelestialCircleSorcerer {
    fn as_view(&'char self) -> CelestialCircleSorcererView<'char> {
        CelestialCircleSorcererView {
            archetypes: self.archetypes.iter().map(|(k, v)| (*k, v)).collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: self.shaping_rituals.iter().enumerate().fold([None; 2], |mut opt_arr, (i, el)| {
                opt_arr[i] = Some(el);
                opt_arr
            }).map(|opt| opt.unwrap()),
            terrestrial_control_spell_id: self.terrestrial_control_spell_id,
            terrestrial_control_spell: &self.terrestrial_control_spell,
            terrestrial_spells: self.terrestrial_spells.iter().map(|(k, v)| (*k, v)).collect(),
            celestial_control_spell_id: self.celestial_control_spell_id,
            celestial_control_spell: &self.celestial_control_spell,
            celestial_spells: self.celestial_spells.iter().map(|(k, v)| (*k, v)).collect(),
        }
    }
}

impl From<CelestialCircleSorcerer> for TerrestrialCircleSorcerer {
    fn from(mut celestial: CelestialCircleSorcerer) -> Self {
        Self {
            archetype_id: celestial.circle_archetypes[0],
            archetype: celestial.archetypes.remove(&celestial.circle_archetypes[0]).expect("Archetypes should be owned"),
            shaping_ritual_id: celestial.shaping_ritual_ids[0],
            shaping_ritual: celestial.shaping_rituals[0].clone(),
            control_spell_id: celestial.terrestrial_control_spell_id,
            control_spell: celestial.terrestrial_control_spell,
            other_spells: celestial.terrestrial_spells,
        }
    }
}

impl<'view, 'source> From<&'view CelestialCircleSorcererView<'source>> for TerrestrialCircleSorcererView<'source> {
    fn from(celestial: &'view CelestialCircleSorcererView<'source>) -> Self {
        Self {
            archetype_id: celestial.circle_archetypes[0],
            archetype: celestial.archetypes.get(&celestial.circle_archetypes[0]).expect("Archetypes should be owned"),
            shaping_ritual_id: celestial.shaping_ritual_ids[0],
            shaping_ritual: celestial.shaping_rituals[0],
            control_spell_id: celestial.terrestrial_control_spell_id,
            control_spell: celestial.terrestrial_control_spell,
            other_spells: celestial.terrestrial_spells.clone(),
        }
    }
}

impl From<SolarCircleSorcerer> for TerrestrialCircleSorcerer {
    fn from(mut solar: SolarCircleSorcerer) -> Self {
        Self {
            archetype_id: solar.circle_archetypes[0],
            archetype: solar.archetypes.remove(&solar.circle_archetypes[0]).expect("Archetypes should be owned"),
            shaping_ritual_id: solar.shaping_ritual_ids[0],
            shaping_ritual: solar.shaping_rituals[0].clone(),
            control_spell_id: solar.terrestrial_control_spell_id,
            control_spell: solar.terrestrial_control_spell,
            other_spells: solar.terrestrial_spells,
        }
    }
}


impl<'view, 'source>  From<&'view SolarCircleSorcererView<'source>> for TerrestrialCircleSorcererView<'source> {
    fn from(solar: &'view SolarCircleSorcererView<'source>) -> Self {
        Self {
            archetype_id: solar.circle_archetypes[0],
            archetype: solar.archetypes.get(&solar.circle_archetypes[0]).expect("Archetypes should be owned"),
            shaping_ritual_id: solar.shaping_ritual_ids[0],
            shaping_ritual: solar.shaping_rituals[0],
            control_spell_id: solar.terrestrial_control_spell_id,
            control_spell: solar.terrestrial_control_spell,
            other_spells: solar.terrestrial_spells.clone(),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SolarCircleSorcerer {
    archetypes: HashMap<SorceryArchetypeId, SorceryArchetype>,
    circle_archetypes: [SorceryArchetypeId; 3],
    shaping_ritual_ids: [ShapingRitualId; 3],
    shaping_rituals: [ShapingRitual; 3],
    terrestrial_control_spell_id: SpellId,
    terrestrial_control_spell: TerrestrialSpell,
    terrestrial_spells: HashMap<SpellId, TerrestrialSpell>,
    celestial_control_spell_id: SpellId,
    celestial_control_spell: CelestialSpell,
    celestial_spells: HashMap<SpellId, CelestialSpell>,
    solar_control_spell_id: SpellId,
    solar_control_spell: SolarSpell,
    solar_spells: HashMap<SpellId, SolarSpell>,
}

impl<'char> SolarCircleSorcerer {
    fn as_view(&'char self) -> SolarCircleSorcererView<'char> {
        SolarCircleSorcererView {
            archetypes: self.archetypes.iter().map(|(k, v)| (*k, v)).collect(),
            circle_archetypes: self.circle_archetypes,
            shaping_ritual_ids: self.shaping_ritual_ids,
            shaping_rituals: self.shaping_rituals.iter().enumerate().fold([None; 3], |mut opt_arr, (i, el)| {
                opt_arr[i] = Some(el);
                opt_arr
            }).map(|opt| opt.unwrap()),
            terrestrial_control_spell_id: self.terrestrial_control_spell_id,
            terrestrial_control_spell: &self.terrestrial_control_spell,
            terrestrial_spells: self.terrestrial_spells.iter().map(|(k, v)| (*k, v)).collect(),
            celestial_control_spell_id: self.celestial_control_spell_id,
            celestial_control_spell: &self.celestial_control_spell,
            celestial_spells: self.celestial_spells.iter().map(|(k, v)| (*k, v)).collect(),
            solar_control_spell_id: self.solar_control_spell_id,
            solar_control_spell: &self.solar_control_spell,
            solar_spells: self.solar_spells.iter().map(|(k, v)| (*k, v)).collect(),
        }
    }
}

enum ExaltSorcerySwitch<'char> {
    Solar(&'char SolarSorcerer),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SolarSorcerer {
    Terrestrial(TerrestrialCircleSorcerer),
    Celestial(CelestialCircleSorcerer),
    Solar(SolarCircleSorcerer),
}

impl<'char> SolarSorcerer {
    pub fn as_view(&'char self) -> SolarSorcererView<'char> {
        match self {
            SolarSorcerer::Terrestrial(terrestrial) => SolarSorcererView::Terrestrial(terrestrial.as_view()),
            SolarSorcerer::Celestial(celestial) => SolarSorcererView::Celestial(celestial.as_view()),
            SolarSorcerer::Solar(solar) => SolarSorcererView::Solar(solar.as_view()),
        }
    }
}

impl<'char> ExaltState {
    fn sorcery(&'char self) -> Option<Sorcery<'char>> {
        match self {
            ExaltState::Mortal(mortal) => {
                if let Some(terrestrial) = &mortal.sorcery {
                    Some(Sorcery(SorcerySwitch::Mortal(&terrestrial)))
                } else {
                    None
                }
            }
            ExaltState::Exalt(exalt) => exalt.sorcery()
        }
    }
}

impl<'char> Exalt {
    fn sorcery(&'char self) -> Option<Sorcery<'char>> {
        match &self.exalt_type {
            ExaltType::Solar(solar) => {
                if let Some(sorcerer) = solar.sorcery() {
                    Some(Sorcery(SorcerySwitch::Exalt(ExaltSorcerySwitch::Solar(&sorcerer))))
                } else {
                    None
                }
            }
        }
    }
}

impl Solar {
    fn sorcery(&self) -> Option<&SolarSorcerer> {
        self.sorcery.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerrestrialCircleSorcererView<'source> {
    archetype_id: SorceryArchetypeId,
    archetype: &'source SorceryArchetype,
    shaping_ritual_id: ShapingRitualId,
    shaping_ritual: &'source ShapingRitual,
    control_spell_id: SpellId,
    control_spell: &'source TerrestrialSpell,
    other_spells: HashMap<SpellId, &'source TerrestrialSpell>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CelestialCircleSorcererView<'source> {
    archetypes: HashMap<SorceryArchetypeId, &'source SorceryArchetype>,
    circle_archetypes: [SorceryArchetypeId; 2],
    shaping_ritual_ids: [ShapingRitualId; 2],
    shaping_rituals: [&'source ShapingRitual; 2],
    terrestrial_control_spell_id: SpellId,
    terrestrial_control_spell: &'source TerrestrialSpell,
    terrestrial_spells: HashMap<SpellId, &'source TerrestrialSpell>,
    celestial_control_spell_id: SpellId,
    celestial_control_spell: &'source CelestialSpell,
    celestial_spells: HashMap<SpellId, &'source CelestialSpell>,
}

enum ExaltSorceryViewSwitch<'view, 'source> {
    Solar(&'view SolarSorcererView<'source>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SolarSorcererView<'source> {
    Terrestrial(TerrestrialCircleSorcererView<'source>),
    Celestial(CelestialCircleSorcererView<'source>),
    Solar(SolarCircleSorcererView<'source>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SolarCircleSorcererView<'source> {
    archetypes: HashMap<SorceryArchetypeId, &'source SorceryArchetype>,
    circle_archetypes: [SorceryArchetypeId; 3],
    shaping_ritual_ids: [ShapingRitualId; 3],
    shaping_rituals: [&'source ShapingRitual; 3],
    terrestrial_control_spell_id: SpellId,
    terrestrial_control_spell: &'source TerrestrialSpell,
    terrestrial_spells: HashMap<SpellId, &'source TerrestrialSpell>,
    celestial_control_spell_id: SpellId,
    celestial_control_spell: &'source CelestialSpell,
    celestial_spells: HashMap<SpellId, &'source CelestialSpell>,
    solar_control_spell_id: SpellId,
    solar_control_spell: &'source SolarSpell,
    solar_spells: HashMap<SpellId, &'source SolarSpell>,
}

impl<'view, 'source> ExaltStateView<'source> {
    fn sorcery(&'view self) -> Option<SorceryView<'view, 'source>> {
        match self {
            ExaltStateView::Mortal(mortal) => {
                if let Some(terrestrial) = &mortal.sorcery {
                    Some(SorceryView(SorceryViewSwitch::Mortal(&terrestrial)))
                } else {
                    None
                }
            }
            ExaltStateView::Exalt(exalt) => exalt.sorcery()
        }
    }
}

impl<'view, 'source> ExaltView<'source> {
    fn sorcery(&'view self) -> Option<SorceryView<'view, 'source>> {
        match &self.exalt_type {
            ExaltTypeView::Solar(solar) => {
                if let Some(sorcerer) = solar.sorcery() {
                    Some(SorceryView(SorceryViewSwitch::Exalt(ExaltSorceryViewSwitch::Solar(&sorcerer))))
                } else {
                    None
                }
            }
        }
    }
}

impl<'view, 'source> SolarView<'source> {
    fn sorcery(&'view self) -> Option<&'view SolarSorcererView<'source>> {
        self.sorcery.as_ref()
    }
}

impl<'source> From<SolarSorcererView<'source>> for SolarSorcerer {
    fn from(view: SolarSorcererView) -> Self {
        match view {
            SolarSorcererView::Terrestrial(terrestrial) => SolarSorcerer::Terrestrial(terrestrial.into()),
            SolarSorcererView::Celestial(celestial) => SolarSorcerer::Celestial(celestial.into()),
            SolarSorcererView::Solar(solar) => SolarSorcerer::Solar(solar.into()),
        }
    }
}

impl<'source> From<TerrestrialCircleSorcererView<'source>> for TerrestrialCircleSorcerer {
    fn from(view: TerrestrialCircleSorcererView) -> Self {
        Self {
            archetype_id: view.archetype_id,
            archetype: view.archetype.to_owned(),
            shaping_ritual_id: view.shaping_ritual_id,
            shaping_ritual: view.shaping_ritual.to_owned(),
            control_spell_id: view.control_spell_id,
            control_spell: view.control_spell.to_owned(),
            other_spells: view.other_spells.into_iter().map(|(k, v)| (k, v.to_owned())).collect(),
        }
    }
}

impl<'source> From<CelestialCircleSorcererView<'source>> for CelestialCircleSorcerer {
    fn from(view: CelestialCircleSorcererView) -> Self {
        Self {
            archetypes: view.archetypes.into_iter().map(|(k, v)| (k, v.to_owned())).collect(),
            circle_archetypes: view.circle_archetypes,
            shaping_ritual_ids: view.shaping_ritual_ids,
            shaping_rituals: view.shaping_rituals.map(|ptr| ptr.to_owned()),
            terrestrial_control_spell_id: view.terrestrial_control_spell_id,
            terrestrial_control_spell: view.terrestrial_control_spell.to_owned(),
            terrestrial_spells: view.terrestrial_spells.into_iter().map(|(k, v)| (k, v.to_owned())).collect(),
            celestial_control_spell_id: view.celestial_control_spell_id,
            celestial_control_spell: view.celestial_control_spell.to_owned(),
            celestial_spells: view.celestial_spells.into_iter().map(|(k, v)| (k, v.to_owned())).collect(),
        }
    }
}

impl<'source> From<SolarCircleSorcererView<'source>> for SolarCircleSorcerer {
    fn from(view: SolarCircleSorcererView) -> Self {
        Self {
            archetypes: view.archetypes.into_iter().map(|(k, v)| (k, v.to_owned())).collect(),
            circle_archetypes: view.circle_archetypes,
            shaping_ritual_ids: view.shaping_ritual_ids,
            shaping_rituals: view.shaping_rituals.map(|ptr| ptr.to_owned()),
            terrestrial_control_spell_id: view.terrestrial_control_spell_id,
            terrestrial_control_spell: view.terrestrial_control_spell.to_owned(),
            terrestrial_spells: view.terrestrial_spells.into_iter().map(|(k, v)| (k, v.to_owned())).collect(),
            celestial_control_spell_id: view.celestial_control_spell_id,
            celestial_control_spell: view.celestial_control_spell.to_owned(),
            celestial_spells: view.celestial_spells.into_iter().map(|(k, v)| (k, v.to_owned())).collect(),
            solar_control_spell_id: view.solar_control_spell_id,
            solar_control_spell: view.solar_control_spell.to_owned(),
            solar_spells: view.solar_spells.into_iter().map(|(k, v)| (k, v.to_owned())).collect(),
        }
    }
}