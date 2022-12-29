use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::{essence::{Essence, EssenceView, Motes, MoteState, MoteCommitmentView, MotesView}, exalt_type::{ExaltType, ExaltState, ExaltStateView, ExaltTypeView}, CharacterMutationError, CommittedMotesId, Character, CharacterView, AbilityName};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum DawnCasteAbility {
    Archery,
    Awareness,
    Brawl,
    Dodge,
    Melee,
    Resistance,
    Thrown,
    War
}

impl From<DawnCasteAbility> for AbilityName {
    fn from(value: DawnCasteAbility) -> Self {
        match value {
            DawnCasteAbility::Archery => Self::Archery,
            DawnCasteAbility::Awareness => Self::Awareness,
            DawnCasteAbility::Brawl => Self::Brawl,
            DawnCasteAbility::Dodge => Self::Dodge,
            DawnCasteAbility::Melee => Self::Melee,
            DawnCasteAbility::Resistance => Self::Resistance,
            DawnCasteAbility::Thrown => Self::Thrown,
            DawnCasteAbility::War => Self::War,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum DawnSupernalAbility {
    Archery,
    Awareness,
    Brawl,
    Dodge,
    MartialArts,
    Melee,
    Resistance,
    Thrown,
    War
}

impl From<DawnSupernalAbility> for AbilityName {
    fn from(value: DawnSupernalAbility) -> Self {
        match value {
            DawnSupernalAbility::Archery => Self::Archery,
            DawnSupernalAbility::Awareness => Self::Awareness,
            DawnSupernalAbility::Brawl => Self::Brawl,
            DawnSupernalAbility::Dodge => Self::Dodge,
            DawnSupernalAbility::MartialArts => Self::MartialArts,
            DawnSupernalAbility::Melee => Self::Melee,
            DawnSupernalAbility::Resistance => Self::Resistance,
            DawnSupernalAbility::Thrown => Self::Thrown,
            DawnSupernalAbility::War => Self::War,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dawn {
    caste_not_supernal: [DawnCasteAbility; 4],
    supernal: DawnSupernalAbility,
}

impl Dawn {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self.caste_not_supernal.iter().any(|dawn_caste_ability| AbilityName::from(*dawn_caste_ability) == ability) {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DawnView {
    caste_not_supernal: [DawnCasteAbility; 4],
    supernal: DawnSupernalAbility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ZenithAbility {
    Athletics,
    Integrity,
    Performance,
    Lore,
    Presence,
    Resistance,
    Survival,
    War,
}

impl From<ZenithAbility> for AbilityName {
    fn from(value: ZenithAbility) -> Self {
        match value {
            ZenithAbility::Athletics => Self::Athletics,
            ZenithAbility::Integrity => Self::Integrity,
            ZenithAbility::Performance => Self::Performance,
            ZenithAbility::Lore => Self::Lore,
            ZenithAbility::Presence => Self::Presence,
            ZenithAbility::Resistance => Self::Resistance,
            ZenithAbility::Survival => Self::Survival,
            ZenithAbility::War => Self::War,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Zenith {
    caste_not_supernal: [ZenithAbility; 4],
    supernal: ZenithAbility,
}

impl Zenith {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self.caste_not_supernal.iter().any(|zenith_caste_ability| AbilityName::from(*zenith_caste_ability) == ability) {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZenithView {
    caste_not_supernal: [ZenithAbility; 4],
    supernal: ZenithAbility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum TwilightAbility {
    Bureaucracy,
    Craft,
    Integrity,
    Investigation,
    Linguistics,
    Lore,
    Medicine,
    Occult,
}

impl From<TwilightAbility> for AbilityName {
    fn from(value: TwilightAbility) -> Self {
        match value {
            TwilightAbility::Bureaucracy => Self::Bureaucracy,
            TwilightAbility::Craft => Self::Craft,
            TwilightAbility::Integrity => Self::Integrity,
            TwilightAbility::Investigation => Self::Investigation,
            TwilightAbility::Linguistics => Self::Linguistics,
            TwilightAbility::Lore => Self::Lore,
            TwilightAbility::Medicine => Self::Medicine,
            TwilightAbility::Occult => Self::Occult,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Twilight {
    caste_not_supernal: [TwilightAbility; 4],
    supernal: TwilightAbility,
}

impl Twilight {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self.caste_not_supernal.iter().any(|twilight_ability| AbilityName::from(*twilight_ability) == ability) {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwilightView {
    caste_not_supernal: [TwilightAbility; 4],
    supernal: TwilightAbility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum NightAbility {
    Athletics,
    Awareness,
    Dodge,
    Investigation,
    Larceny,
    Ride,
    Stealth,
    Socialize,
}

impl From<NightAbility> for AbilityName {
    fn from(value: NightAbility) -> Self {
        match value {
            NightAbility::Athletics => Self::Athletics,
            NightAbility::Awareness => Self::Awareness,
            NightAbility::Dodge => Self::Dodge,
            NightAbility::Investigation => Self::Investigation,
            NightAbility::Larceny => Self::Larceny,
            NightAbility::Ride => Self::Ride,
            NightAbility::Stealth => Self::Stealth,
            NightAbility::Socialize => Self::Socialize,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Night {
    caste_not_supernal: [NightAbility; 4],
    supernal: NightAbility,
}

impl Night {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self.caste_not_supernal.iter().any(|night_ability| AbilityName::from(*night_ability) == ability) {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NightView {
    caste_not_supernal: [NightAbility; 4],
    supernal: NightAbility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum EclipseAbility {
    Bureaucracy,
    Larceny,
    Linguistics,
    Occult,
    Presence,
    Ride,
    Sail,
    Socialize,
}

impl From<EclipseAbility> for AbilityName {
    fn from(value: EclipseAbility) -> Self {
        match value {
            EclipseAbility::Bureaucracy => Self::Bureaucracy,
            EclipseAbility::Larceny => Self::Larceny,
            EclipseAbility::Linguistics => Self::Linguistics,
            EclipseAbility::Occult => Self::Occult,
            EclipseAbility::Presence => Self::Presence,
            EclipseAbility::Ride => Self::Ride,
            EclipseAbility::Sail => Self::Sail,
            EclipseAbility::Socialize => Self::Socialize,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Eclipse {
    caste_not_supernal: [EclipseAbility; 4],
    supernal: EclipseAbility,
}

impl Eclipse {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        if self.caste_not_supernal.iter().any(|eclipse_ability| AbilityName::from(*eclipse_ability) == ability) {
            true
        } else {
            AbilityName::from(self.supernal) == ability
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EclipseView {
    caste_not_supernal: [EclipseAbility; 4],
    supernal: EclipseAbility,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolarCaste {
    Dawn(Dawn),
    Zenith(Zenith),
    Twilight(Twilight),
    Night(Night),
    Eclipse(Eclipse),
}

impl SolarCaste {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        match self {
            SolarCaste::Dawn(dawn) => dawn.has_caste_ability(ability),
            SolarCaste::Zenith(zenith) => zenith.has_caste_ability(ability),
            SolarCaste::Twilight(twilight) => twilight.has_caste_ability(ability),
            SolarCaste::Night(night) => night.has_caste_ability(ability),
            SolarCaste::Eclipse(eclipse) => eclipse.has_caste_ability(ability),
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        match self {
            SolarCaste::Dawn(dawn) => dawn.supernal_ability(),
            SolarCaste::Zenith(zenith) => zenith.supernal_ability(),
            SolarCaste::Twilight(twilight) => twilight.supernal_ability(),
            SolarCaste::Night(night) => night.supernal_ability(),
            SolarCaste::Eclipse(eclipse) => eclipse.supernal_ability(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolarCasteView {
    Dawn(DawnView),
    Zenith(ZenithView),
    Twilight(TwilightView),
    Night(NightView),
    Eclipse(EclipseView),
}

/// Traits which are unique to being a Solar Exalted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Solar {
    pub(crate) essence: Essence,
    caste: SolarCaste,
    favored_abilities: [AbilityName; 5],
}

/// Traits which are unique to being a Solar Exalted, with &str
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolarView<'source> {
    pub(crate) essence: EssenceView<'source>,
    caste: SolarCasteView,
    favored_abilities: [AbilityName; 5],
}

pub struct SolarTraitsBuilder {
    caste: Option<SolarCaste>,
    favored_abilities: HashSet<AbilityName>,
}

impl Solar {
    /// Creates a builder to construct SolarTraits.
    pub fn builder() -> SolarTraitsBuilder {
        SolarTraitsBuilder {
            caste: None,
            favored_abilities: HashSet::new(),
        }
    }

    /// Returns True if the ability is a caste ability for the charcter. Note
    /// that MartialArts is a caste ability if and only if Brawl is a caste
    /// ability.
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        self.caste.has_caste_ability(ability)
    }

    /// Returns the Solar's supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        self.caste.supernal_ability()
    }

    /// Returns True if the ability is a favored ability for the charcter. Note
    /// that MartialArts is a favored ability if and only if Brawl is a favored
    /// ability.
    pub fn has_favored_ability(&self, ability: AbilityName) -> bool {
        self.favored_abilities.iter().any(|&a| a == ability)
    }
}

#[derive(Debug, Error)]
pub enum SolarBuilderError {
    #[error("Caste and Favored abilities must be unique")]
    UniqueCasteAndFavored,
    #[error("Required field missing: {0}")]
    MissingField(&'static str),
    #[error("Must have 5 Caste and 5 Favored abilities")]
    CasteAndFavoredCount,
    #[error("Martial Arts cannot be a Caste or Favored ability")]
    MartialArts,
}

impl SolarTraitsBuilder {
    pub fn dawn(&mut self, dawn: Dawn) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Dawn(dawn));
        self
    }

    pub fn zenith(&mut self, zenith: Zenith) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Zenith(zenith));
        self
    }

    pub fn twilight(&mut self, twilight: Twilight) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Twilight(twilight));
        self
    }

    pub fn night(&mut self, night: Night) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Night(night));
        self
    }

    pub fn eclipse(&mut self, eclipse: Eclipse) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Eclipse(eclipse));
        self
    }

    pub fn favored_ability(&mut self, ability: AbilityName) -> Result<&mut Self, SolarBuilderError> {
        if ability == AbilityName::MartialArts {
            Err(SolarBuilderError::MartialArts)
        } else if self.caste.as_ref().map_or(false, |c| c.has_caste_ability(ability)) {
            Err(SolarBuilderError::UniqueCasteAndFavored)

        } else if !self.favored_abilities.insert(ability) {
            Err(SolarBuilderError::UniqueCasteAndFavored)
        } else {
            Ok(self)
        }
    }



    /// Consumes the builder to finalize Solar Traits.
    pub fn build(self) -> Result<Solar, SolarBuilderError> {
        if self.caste.is_none() {
            return Err(SolarBuilderError::MissingField("caste"));
        }

        if self.favored_abilities.len() != 5 {
            return Err(SolarBuilderError::CasteAndFavoredCount);
        }

        let mut option_arr = [None; 5];

        for (i, ability) in self.favored_abilities.into_iter().enumerate() {
            option_arr[i] = Some(ability);
        }

        let mut arr = option_arr.map(|el| el.unwrap());
        arr.sort();


        Ok(Solar {
            essence: Essence {
                rating: 1,
                motes: Motes {
                    peripheral: MoteState {
                        available: 33,
                        spent: 0,
                    },
                    personal: MoteState {
                        available: 13,
                        spent: 0,
                    },
                    commitments: HashMap::new(),
                },
            },
            caste: self.caste.unwrap(),
            favored_abilities: arr,
        })
    }
}


impl ExaltType {
    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        match self {
            ExaltType::Solar(solar_traits) => Some(solar_traits),
        }
    }
}

impl<'source> ExaltTypeView<'source> {
    pub fn is_solar(&self) -> bool {
        true
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        match self {
            ExaltTypeView::Solar(solar_traits) => Some(solar_traits),
        }
    }
}

impl ExaltState {
    pub fn is_solar(&self) -> bool {
        if let Self::Exalted(exalt_type) = self {
            exalt_type.is_solar()
        } else {
            false
        }
    }

    pub fn solar_traits(&self) -> Option<&Solar> {
        if let Self::Exalted(exalt_type) = self {
            exalt_type.solar_traits()
        } else {
            None
        }
    }
    
    pub fn check_set_solar(
        &self,
        _solar_traits: &Solar,
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(
        &mut self,
        solar_traits: &Solar,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self = Self::Exalted(ExaltType::Solar(solar_traits.clone()));
        Ok(self)
    }
}

impl<'source> ExaltStateView<'source> {
    pub fn is_solar(&self) -> bool {
        if let Self::Exalted(exalt_type) = self {
            exalt_type.is_solar()
        } else {
            false
        }
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        if let Self::Exalted(exalt_type) = self {
            exalt_type.solar_traits()
        } else {
            None
        }
    }

    pub fn check_set_solar(
        &self,
        _solar_traits: &'source Solar,
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    pub fn set_solar(
        &mut self,
        solar_traits: &'source Solar,
    ) -> Result<&mut Self, CharacterMutationError> {
        let rating = solar_traits.essence.rating();

        let peripheral = {
            let available = solar_traits.essence.motes().peripheral().available();
            let spent = solar_traits.essence.motes().peripheral().spent();
            MoteState { available, spent }
        };

        let personal = {
            let available = solar_traits.essence.motes().personal().available();
            let spent = solar_traits.essence.motes().personal().spent();
            MoteState { available, spent }
        };

        let commitments = solar_traits
            .essence
            .motes()
            .committed()
            .map(|(id, name, peripheral, personal)| {
                (
                    id,
                    MoteCommitmentView {
                        name,
                        peripheral,
                        personal,
                    },
                )
            })
            .collect::<HashMap<CommittedMotesId, MoteCommitmentView>>();

        let motes = MotesView {
            peripheral,
            personal,
            commitments,
        };

        let essence = EssenceView { rating, motes };

        let caste = match &solar_traits.caste {
            SolarCaste::Dawn(dawn) => SolarCasteView::Dawn(
                DawnView { 
                    caste_not_supernal: dawn.caste_not_supernal, 
                    supernal: dawn.supernal,
                }
            ),
            SolarCaste::Zenith(zenith) => SolarCasteView::Zenith(
                ZenithView { 
                    caste_not_supernal: zenith.caste_not_supernal, 
                    supernal: zenith.supernal,
                }
            ),
            SolarCaste::Twilight(twilight) => SolarCasteView::Twilight(
                TwilightView { 
                    caste_not_supernal: twilight.caste_not_supernal,
                    supernal: twilight.supernal,
                }
            ),
            SolarCaste::Night(night) => SolarCasteView::Night(
                NightView { 
                    caste_not_supernal: night.caste_not_supernal, 
                    supernal: night.supernal,
                }
            ),
            SolarCaste::Eclipse(eclipse) => SolarCasteView::Eclipse(
                EclipseView { 
                    caste_not_supernal: eclipse.caste_not_supernal, 
                    supernal: eclipse.supernal,
                }
            ),
        };
        let favored_abilities = solar_traits.favored_abilities;

        let solar_traits_view = SolarView { 
            essence,
            caste,
            favored_abilities,
        };

        *self = Self::Exalted(ExaltTypeView::Solar(solar_traits_view));
        Ok(self)
    }
}

impl Character {
    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exalt_state.is_solar()
    }

    /// Returns the character's Solar-specific traits, or None if not a Solar.
    pub fn solar_traits(&self) -> Option<&Solar> {
        self.exalt_state.solar_traits()
    }

    /// Checks if character can be turned into a Solar Exalted with given
    /// traits.
    pub fn check_set_solar(
        &self,
        solar_traits: &Solar,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_solar(solar_traits)
    }

    /// Sets a character's Exaltation to be the given Solar exaltation. If the
    /// character was previously mortal, permanent willpower rating is
    /// increased by 2 (reflecting the difference between mortal default and
    /// Exalt default).
    pub fn set_solar(
        &mut self,
        solar_traits: &Solar,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_solar(solar_traits)?;
        if self.is_mortal() {
            let new_willpower_rating = self.willpower().rating() + 2;
            self.set_willpower_rating(new_willpower_rating)?;
        }
        self.exalt_state.set_solar(solar_traits)?;
        Ok(self)
    }
}

impl<'source> CharacterView<'source> {
    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exalt_state.is_solar()
    }

    /// Returns the character's Solar-specific traits, or None if not a Solar.
    pub fn solar_traits(&self) -> Option<&SolarView> {
        self.exalt_state.solar_traits()
    }

    /// Checks if character can be turned into a Solar Exalted with given
    /// traits.
    pub fn check_set_solar(
        &self,
        solar_traits: &Solar,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_solar(solar_traits)
    }

    /// Sets a character's Exaltation to be the given Solar exaltation. If the
    /// character was previously mortal, permanent willpower rating is
    /// increased by 2 (reflecting the difference between mortal default and
    /// Exalt default).
    pub fn set_solar(
        &mut self,
        solar_traits: &'source Solar,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_solar(solar_traits)?;
        if self.is_mortal() {
            let new_willpower_rating = self.willpower().rating() + 2;
            self.set_willpower_rating(new_willpower_rating)?;
        }
        self.exalt_state.set_solar(solar_traits)?;
        Ok(self)
    }
}