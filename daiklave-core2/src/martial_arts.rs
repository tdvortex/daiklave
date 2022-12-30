use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    abilities::{Ability, AbilityView, AbilityNameVanilla},
    armor::ArmorWeight,
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost, CharmKeyword},
    id::UniqueId,
    weapons::WeaponId,
    CharacterMutationError, CharacterView, exalt_state::{ExaltStateView, mortal::MortalView, exalt::ExaltView},
};

/// A unique identifier for a Martial Arts style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MartialArtsStyleId(pub UniqueId);

impl Deref for MartialArtsStyleId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Martial Arts style description.
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MartialArtsStyle {
    book_reference: Option<BookReference>,
    name: String,
    description: String,
    usable_weapons: HashSet<WeaponId>,
    max_armor_weight: Option<ArmorWeight>,
}

impl MartialArtsStyle {
    /// Construct a new Martial Arts style
    pub fn new(
        book_reference: Option<BookReference>,
        name: String,
        description: String,
        usable_weapons: HashSet<WeaponId>,
        max_armor_weight: Option<ArmorWeight>,
    ) -> Self {
        Self {
            book_reference,
            name,
            description,
            usable_weapons,
            max_armor_weight,
        }
    }

    /// The page reference for the style (if any).
    pub fn book_reference(&self) -> Option<BookReference> {
        self.book_reference
    }

    /// The style's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The style's description.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// A list of weapon ids, which may be either mortal weapons (e.g. sword)
    /// or base artifact weapons (e.g. daiklave), usable by the style.
    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.usable_weapons.iter().copied()
    }

    /// The maximum weight of armor which may be worn with the style, or None
    /// if incompatible with armor.
    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        self.max_armor_weight
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtist {
    style: MartialArtsStyle,
    ability: Ability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MortalMartialArtistView<'source> {
    style: &'source MartialArtsStyle,
    ability: AbilityView<'source>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMartialArtist {
    style: MartialArtsStyle,
    ability: Ability,
    charms: HashMap<MartialArtsCharmId, MartialArtsCharm>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltMartialArtistView<'source> {
    style: &'source MartialArtsStyle,
    ability: AbilityView<'source>,
    charms: HashMap<MartialArtsCharmId, &'source MartialArtsCharm>,
}

impl From<MortalMartialArtist> for ExaltMartialArtist {
    fn from(mortal_artist: MortalMartialArtist) -> Self {
        Self {
            style: mortal_artist.style,
            ability: mortal_artist.ability,
            charms: HashMap::new(),
        }
    }
}

impl From<ExaltMartialArtist> for MortalMartialArtist {
    fn from(exalt_artist: ExaltMartialArtist) -> Self {
        Self {
            style: exalt_artist.style,
            ability: exalt_artist.ability,
        }
    }
}

impl<'source> From<MortalMartialArtistView<'source>> for ExaltMartialArtistView<'source> {
    fn from(mortal_artist: MortalMartialArtistView<'source>) -> Self {
        Self {
            style: mortal_artist.style,
            ability: mortal_artist.ability,
            charms: HashMap::new(),
        }
    }
}

impl<'source> From<ExaltMartialArtistView<'source>> for MortalMartialArtistView<'source> {
    fn from(exalt_artist: ExaltMartialArtistView<'source>) -> Self {
        Self {
            style: exalt_artist.style,
            ability: exalt_artist.ability,
        }
    }
}


/// A unique identifier for a Martial Arts Charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MartialArtsCharmId(pub UniqueId);

impl Deref for MartialArtsCharmId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Martial Arts charm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MartialArtsCharm {
    style: MartialArtsStyleId,
    book_reference: Option<BookReference>,
    name: String,
    summary: Option<String>,
    description: String,
    essence_required: u8,
    ability_required: u8,
    charms_required: Vec<MartialArtsCharmId>,
    keywords: Vec<CharmKeyword>,
    costs: Vec<CharmCost>,
    action_type: Vec<CharmActionType>,
    duration: String,
}

/// An error when tryng to add a Martial Arts style to a character.
#[derive(Debug, Error)]
pub enum AddMartialArtsStyleError {
    /// Prerequisite conditions were not met.
    #[error("Prerequisite not met: {0}")]
    PrerequsitesNotMet(String),
    /// Style already exists.
    #[error("Already have style with this id")]
    DuplicateStyle,
}

impl<'source> CharacterView<'source> {
    /// Checks if a Martial Arts style can be added to the character.
    pub fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.abilities().dots(AbilityNameVanilla::Brawl) < 1 {
            return Err(CharacterMutationError::AddMartialArtsStyleError(AddMartialArtsStyleError::PrerequsitesNotMet("Brawl must be 1+ to take Martial Artist merit".to_owned())));
        }

        self.exalt_state.check_add_martial_arts_style(id, style)
    }

    /// Checks if a Martial Arts style can be added to the character.
    pub fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.exalt_state.add_martial_arts_style(id, style)?;

        Ok(self)
    }
}

impl<'source> ExaltStateView<'source> {
    pub(crate) fn check_add_martial_arts_style(&self, id: MartialArtsStyleId, style: &MartialArtsStyle) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => mortal.check_add_martial_arts_style(id, style),
            ExaltStateView::Exalted(exalt) => exalt.check_add_martial_arts_style(id, style),
        }
    }

    pub(crate) fn add_martial_arts_style(&mut self, id: MartialArtsStyleId, style: &'source MartialArtsStyle) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => {mortal.add_martial_arts_style(id, style)?;}
            ExaltStateView::Exalted(exalt) => {exalt.add_martial_arts_style(id, style)?;}
        }
        Ok(self)
    }
}

impl<'source> MortalView<'source> {
    pub(crate) fn check_add_martial_arts_style(&self, id: MartialArtsStyleId, _style: &MartialArtsStyle) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::AddMartialArtsStyleError(AddMartialArtsStyleError::DuplicateStyle))
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_martial_arts_style(&mut self, id: MartialArtsStyleId, style: &'source MartialArtsStyle) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.martial_arts_styles.insert(id, MortalMartialArtistView {
            style,
            ability: AbilityView::Zero
        });
        Ok(self)
    }
}

impl<'source> ExaltView<'source> {
    pub(crate) fn check_add_martial_arts_style(&self, id: MartialArtsStyleId, _style: &MartialArtsStyle) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::AddMartialArtsStyleError(AddMartialArtsStyleError::DuplicateStyle))
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_martial_arts_style(&mut self, id: MartialArtsStyleId, style: &'source MartialArtsStyle) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.martial_arts_styles.insert(id, ExaltMartialArtistView {
            style,
            ability: AbilityView::Zero,
            charms: HashMap::new(),
        });
        Ok(self)
    }
}
