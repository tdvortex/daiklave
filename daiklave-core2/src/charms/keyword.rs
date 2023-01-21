use serde::{Deserialize, Serialize};

use crate::{exaltation::exalt::exalt_type::solar::charm::SolarCharmKeyword, sorcery::SpellKeyword, martial_arts::MartialArtsCharmKeyword};

use super::charm::{EvocationKeyword, SpiritCharmKeyword};

/// All the keywords that may be associated with a specific Charm or Spell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub enum CharmKeyword {
    /// Air Dragon-Blooded Aura
    Air,
    /// Aggravated damage-dealing
    Aggravated,
    /// Non-canonical. Ask your Storyteller before using.
    Apocryphal,
    /// Lunar Archetype
    Archetype,
    /// Relies on Dragon-Blooded Aura
    Aura,
    /// Balanced elemental energy for Dragon-Blooded aura
    Balanced,
    /// Bridge, alternate purchase costs
    Bridge,
    /// Affects clash attacks
    Clash,
    /// Allows or applies to a Counterattack
    Counterattack,
    /// Only applies to Decisive attacks
    DecisiveOnly,
    /// Effects are limited when the Exalt's type is dissonant with the Magic
    /// Material of its construction.
    Dissonant,
    /// Dual (different applications to Decisive and Withering)
    Dual,
    /// The Charm has additional effect when used by Sidereals.
    Enlightenment,
    /// An Excellency Charm
    Excellency,
    /// The key Form ability for a Martial Arts style.
    Form,
    /// Fire Dragon-Blooded Aura
    Fire,
    /// Earth Dragon-Blooded Aura
    Earth,
    /// Charm has additional effect when used by Solars or Sidereals.
    Mastery,
    /// Does not increase Anima when used
    Mute,
    /// Applies when captaining a ship or boat
    Pilot,
    /// Relates to Lunar shapeshifting
    Protean,
    /// Mind-affecting Charms
    Psyche,
    /// Cannot be used while crashed
    Perilous,
    /// Has additional effect when the Exalt is resonant with the Magic
    /// Material of the artifact
    Resonant,
    /// Charm requires spending Crafting experience
    Salient,
    /// A Dragon-Blooded signature Elemental Charm for an Ability
    Signature,
    /// Can be applied multiple times
    Stackable,
    /// Charm has reduced effect or increased cost when used by the
    /// Dragon-Blooded.
    Terrestrial,
    /// Affects Withering and Decisive attacks equally
    Uniform,
    /// Water Dragon-Blooded Aura
    Water,
    /// Applies to Withering attacks only
    WitheringOnly,
    /// Wood Dragon-Blooded Aura
    Wood,
    /// Only usable in written form
    WrittenOnly,
}

impl From<EvocationKeyword> for CharmKeyword {
    fn from(keyword: EvocationKeyword) -> Self {
        match keyword {
            EvocationKeyword::Aggravated => Self::Aggravated,
            EvocationKeyword::Clash => Self::Clash,
            EvocationKeyword::Counterattack => Self::Counterattack,
            EvocationKeyword::DecisiveOnly => Self::DecisiveOnly,
            EvocationKeyword::Dissonant => Self::Dissonant,
            EvocationKeyword::Dual => Self::Dual,
            EvocationKeyword::Mute => Self::Mute,
            EvocationKeyword::Psyche => Self::Psyche,
            EvocationKeyword::Resonant => Self::Resonant,
            EvocationKeyword::Stackable => Self::Stackable,
            EvocationKeyword::Uniform => Self::Uniform,
            EvocationKeyword::WitheringOnly => Self::WitheringOnly,
        }
    }
}

impl From<SolarCharmKeyword> for CharmKeyword {
    fn from(keyword: SolarCharmKeyword) -> Self {
        match keyword {
            SolarCharmKeyword::Aggravated => Self::Aggravated,
            SolarCharmKeyword::Apocryphal => Self::Apocryphal,
            SolarCharmKeyword::Bridge => Self::Bridge,
            SolarCharmKeyword::Clash => Self::Clash,
            SolarCharmKeyword::Counterattack => Self::Counterattack,
            SolarCharmKeyword::DecisiveOnly => Self::DecisiveOnly,
            SolarCharmKeyword::Dual => Self::Dual,
            SolarCharmKeyword::Mute => Self::Mute,
            SolarCharmKeyword::Pilot => Self::Pilot,
            SolarCharmKeyword::Psyche => Self::Psyche,
            SolarCharmKeyword::Perilous => Self::Perilous,
            SolarCharmKeyword::Salient => Self::Salient,
            SolarCharmKeyword::Stackable => Self::Stackable,
            SolarCharmKeyword::Uniform => Self::Uniform,
            SolarCharmKeyword::WitheringOnly => Self::WitheringOnly,
            SolarCharmKeyword::WrittenOnly => Self::WrittenOnly,
        }
    }
}

impl From<SpellKeyword> for CharmKeyword {
    fn from(keyword: SpellKeyword) -> Self {
        match keyword {
            SpellKeyword::Aggravated => Self::Aggravated,
            SpellKeyword::DecisiveOnly => Self::DecisiveOnly,
            SpellKeyword::Perilous => Self::Perilous,
            SpellKeyword::Psyche => Self::Psyche,
        }
    }
}

impl From<SpiritCharmKeyword> for CharmKeyword {
    fn from(keyword: SpiritCharmKeyword) -> Self {
        match keyword {
            SpiritCharmKeyword::Aggravated => Self::Aggravated,
            SpiritCharmKeyword::Clash => Self::Clash,
            SpiritCharmKeyword::Counterattack => Self::Counterattack,
            SpiritCharmKeyword::DecisiveOnly => Self::DecisiveOnly,
            SpiritCharmKeyword::Dual => Self::Dual,
            SpiritCharmKeyword::Psyche => Self::Psyche,
            SpiritCharmKeyword::Perilous => Self::Perilous,
            SpiritCharmKeyword::Stackable => Self::Stackable,
            SpiritCharmKeyword::Uniform => Self::Uniform,
            SpiritCharmKeyword::WitheringOnly => Self::WitheringOnly,
            SpiritCharmKeyword::WrittenOnly => Self::WrittenOnly,
        }
    }
}

impl From<MartialArtsCharmKeyword> for CharmKeyword {
    fn from(keyword: MartialArtsCharmKeyword) -> Self {
        match keyword {
            MartialArtsCharmKeyword::Aggravated => Self::Aggravated,
            MartialArtsCharmKeyword::Clash => Self::Clash,
            MartialArtsCharmKeyword::Counterattack => Self::Counterattack,
            MartialArtsCharmKeyword::DecisiveOnly => Self::DecisiveOnly,
            MartialArtsCharmKeyword::Dual => Self::Dual,
            MartialArtsCharmKeyword::Enlightenment => Self::Enlightenment,
            MartialArtsCharmKeyword::Form => Self::Form,
            MartialArtsCharmKeyword::Mastery => Self::Mastery,
            MartialArtsCharmKeyword::Mute => Self::Mute,
            MartialArtsCharmKeyword::Psyche => Self::Psyche,
            MartialArtsCharmKeyword::Perilous => Self::Perilous,
            MartialArtsCharmKeyword::Stackable => Self::Stackable,
            MartialArtsCharmKeyword::Terrestrial => Self::Terrestrial,
            MartialArtsCharmKeyword::Uniform => Self::Uniform,
            MartialArtsCharmKeyword::WitheringOnly => Self::WitheringOnly,
        }
    }
}