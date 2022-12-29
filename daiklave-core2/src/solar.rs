use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::{essence::{Essence, EssenceView, Motes, MoteState, MoteCommitmentView, MotesView}, exalt_type::{ExaltType, ExaltState, ExaltStateView, ExaltTypeView}, CharacterMutationError, CommittedMotesId, Character, CharacterView, AbilityName, guided::ExaltationChoice};

use self::{dawn::{DawnView}, zenith::{ZenithView}, twilight::{TwilightView}, night::{NightView}, eclipse::{EclipseView}};
mod dawn;
mod eclipse;
mod night;
mod twilight;
mod zenith;

pub use dawn::{Dawn, DawnBuilder};
pub use eclipse::{Eclipse, EclipseBuilder};
pub use night::{Night, NightBuilder};
pub use twilight::{Twilight, TwilightBuilder};
pub use zenith::{Zenith, ZenithBuilder};


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


impl SolarCasteView {
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        match self {
            SolarCasteView::Dawn(dawn) => dawn.has_caste_ability(ability),
            SolarCasteView::Zenith(zenith) => zenith.has_caste_ability(ability),
            SolarCasteView::Twilight(twilight) => twilight.has_caste_ability(ability),
            SolarCasteView::Night(night) => night.has_caste_ability(ability),
            SolarCasteView::Eclipse(eclipse) => eclipse.has_caste_ability(ability),
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        match self {
            SolarCasteView::Dawn(dawn) => dawn.supernal_ability(),
            SolarCasteView::Zenith(zenith) => zenith.supernal_ability(),
            SolarCasteView::Twilight(twilight) => twilight.supernal_ability(),
            SolarCasteView::Night(night) => night.supernal_ability(),
            SolarCasteView::Eclipse(eclipse) => eclipse.supernal_ability(),
        }
    }
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

impl<'source> SolarView<'source> {
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
    #[error("Must use correct caste abilities")]
    InvalidCasteAbility,
}

impl SolarTraitsBuilder {
    pub fn set_dawn(&mut self, dawn: Dawn) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Dawn(dawn));
        self
    }

    pub fn set_zenith(&mut self, zenith: Zenith) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Zenith(zenith));
        self
    }

    pub fn set_twilight(&mut self, twilight: Twilight) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Twilight(twilight));
        self
    }

    pub fn set_night(&mut self, night: Night) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Night(night));
        self
    }

    pub fn set_eclipse(&mut self, eclipse: Eclipse) -> &mut Self {
        if !self.favored_abilities.is_empty() {
            self.favored_abilities.clear();
        }

        self.caste = Some(SolarCaste::Eclipse(eclipse));
        self
    }

    pub fn add_favored_ability(&mut self, ability: AbilityName) -> Result<&mut Self, SolarBuilderError> {
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

pub(crate) fn validate_solar_caste_ability(exaltation: ExaltationChoice, ability: AbilityName) -> bool {
    match (exaltation, ability) {
        (ExaltationChoice::Dawn, AbilityName::Archery)
        | (ExaltationChoice::Dawn, AbilityName::Awareness)
        | (ExaltationChoice::Dawn, AbilityName::Brawl)
        | (ExaltationChoice::Dawn, AbilityName::Dodge)
        | (ExaltationChoice::Dawn, AbilityName::Melee)
        | (ExaltationChoice::Dawn, AbilityName::Resistance)
        | (ExaltationChoice::Dawn, AbilityName::Thrown)
        | (ExaltationChoice::Dawn, AbilityName::War)

        | (ExaltationChoice::Zenith, AbilityName::Athletics)
        | (ExaltationChoice::Zenith, AbilityName::Integrity)
        | (ExaltationChoice::Zenith, AbilityName::Performance)
        | (ExaltationChoice::Zenith, AbilityName::Lore)
        | (ExaltationChoice::Zenith, AbilityName::Presence)
        | (ExaltationChoice::Zenith, AbilityName::Resistance)
        | (ExaltationChoice::Zenith, AbilityName::Survival)
        | (ExaltationChoice::Zenith, AbilityName::War)

        | (ExaltationChoice::Twilight, AbilityName::Bureaucracy)
        | (ExaltationChoice::Twilight, AbilityName::Craft)
        | (ExaltationChoice::Twilight, AbilityName::Integrity)
        | (ExaltationChoice::Twilight, AbilityName::Investigation)
        | (ExaltationChoice::Twilight, AbilityName::Linguistics)
        | (ExaltationChoice::Twilight, AbilityName::Lore)
        | (ExaltationChoice::Twilight, AbilityName::Medicine)
        | (ExaltationChoice::Twilight, AbilityName::Occult)

        | (ExaltationChoice::Night, AbilityName::Athletics)
        | (ExaltationChoice::Night, AbilityName::Awareness)
        | (ExaltationChoice::Night, AbilityName::Dodge)
        | (ExaltationChoice::Night, AbilityName::Investigation)
        | (ExaltationChoice::Night, AbilityName::Larceny)
        | (ExaltationChoice::Night, AbilityName::Ride)
        | (ExaltationChoice::Night, AbilityName::Stealth)
        | (ExaltationChoice::Night, AbilityName::Socialize)

        | (ExaltationChoice::Eclipse, AbilityName::Bureaucracy)
        | (ExaltationChoice::Eclipse, AbilityName::Larceny)
        | (ExaltationChoice::Eclipse, AbilityName::Linguistics)
        | (ExaltationChoice::Eclipse, AbilityName::Occult)
        | (ExaltationChoice::Eclipse, AbilityName::Presence)
        | (ExaltationChoice::Eclipse, AbilityName::Ride)
        | (ExaltationChoice::Eclipse, AbilityName::Sail)
        | (ExaltationChoice::Eclipse, AbilityName::Socialize) => true,
        _ => false
    }
}