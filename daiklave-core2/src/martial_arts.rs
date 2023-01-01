use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    armor::ArmorWeight,
    book_reference::BookReference,
    charms::{CharmActionType, CharmCost, CharmKeyword},
    exalt_state::{ExaltState, ExaltStateView},
    id::UniqueId,
    weapons::WeaponId,
    Character, CharacterMutationError, CharacterView,
};

mod character;
mod character_view;
mod exalt;
mod mortal;

pub(crate) use exalt::{ExaltMartialArtist, ExaltMartialArtistView};

pub(crate) use self::mortal::{MortalMartialArtist, MortalMartialArtistView};

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

/// An error when tryng to remove a Martial Arts style from a character.
#[derive(Debug, Error)]
pub enum RemoveMartialArtsStyleError {
    /// Can't remove a missing style
    #[error("Style not found")]
    NotFound,
}

/// An error when trying to set Martial Arts dots.
#[derive(Debug, Error)]
pub enum SetMartialArtsDotsError {
    /// Can't change dots on a missing style
    #[error("Style not found")]
    NotFound,
}

impl<'source> ExaltStateView<'source> {
    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => mortal.check_add_martial_arts_style(id, style),
            ExaltStateView::Exalt(exalt) => exalt.check_add_martial_arts_style(id, style),
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => {
                mortal.add_martial_arts_style(id, style)?;
            }
            ExaltStateView::Exalt(exalt) => {
                exalt.add_martial_arts_style(id, style)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => mortal.check_remove_martial_arts_style(id),
            ExaltStateView::Exalt(exalt) => exalt.check_remove_martial_arts_style(id),
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => {
                mortal.remove_martial_arts_style(id)?;
            }
            ExaltStateView::Exalt(exalt) => {
                exalt.remove_martial_arts_style(id)?;
            }
        }
        Ok(self)
    }

    fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => mortal.check_set_martial_arts_dots(id, dots),
            ExaltStateView::Exalt(exalt) => exalt.check_set_martial_arts_dots(id, dots),
        }
    }

    fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltStateView::Mortal(mortal) => {
                mortal.set_martial_arts_dots(id, dots)?;
            }
            ExaltStateView::Exalt(exalt) => {
                exalt.set_martial_arts_dots(id, dots)?;
            }
        }
        Ok(self)
    }
}

impl ExaltState {
    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => mortal.check_add_martial_arts_style(id, style),
            ExaltState::Exalt(exalt) => exalt.check_add_martial_arts_style(id, style),
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => {
                mortal.add_martial_arts_style(id, style)?;
            }
            ExaltState::Exalt(exalt) => {
                exalt.add_martial_arts_style(id, style)?;
            }
        }
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => mortal.check_remove_martial_arts_style(id),
            ExaltState::Exalt(exalt) => exalt.check_remove_martial_arts_style(id),
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => {
                mortal.remove_martial_arts_style(id)?;
            }
            ExaltState::Exalt(exalt) => {
                exalt.remove_martial_arts_style(id)?;
            }
        }
        Ok(self)
    }

    fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => mortal.check_set_martial_arts_dots(id, dots),
            ExaltState::Exalt(exalt) => exalt.check_set_martial_arts_dots(id, dots),
        }
    }

    fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self {
            ExaltState::Mortal(mortal) => {
                mortal.set_martial_arts_dots(id, dots)?;
            }
            ExaltState::Exalt(exalt) => {
                exalt.set_martial_arts_dots(id, dots)?;
            }
        }
        Ok(self)
    }
}

/// All of the character's Martial Arts styles.
pub struct MartialArtsView<'view, 'source>(&'view CharacterView<'source>);

impl<'view, 'source> MartialArtsView<'view, 'source> {
    /// The details of a particular Martial Arts style.
    pub fn style(&self, id: MartialArtsStyleId) -> Option<MartialArtistView<'view, 'source>> {
        self.0.exalt_state.martial_artist(id)
    }

    /// Iterates over the Ids for all Martial Arts the character knows.
    pub fn iter(&self) -> impl Iterator<Item = MartialArtsStyleId> {
        self.0.exalt_state.martial_arts_id_iter()
    }
}

/// All of the character's Martial Arts styles.
pub struct MartialArts<'char>(&'char Character);

impl<'char> MartialArts<'char> {
    /// The details of a particular Martial Arts style.
    pub fn style(&self, id: MartialArtsStyleId) -> Option<MartialArtist<'char>> {
        self.0.exalt_state.martial_artist(id)
    }

    /// Iterates over the Ids for all Martial Arts the character knows.
    pub fn iter(&self) -> impl Iterator<Item = MartialArtsStyleId> {
        self.0.exalt_state.martial_arts_id_iter()
    }
}

/// A specific Martial Arts style as known by a character.
pub struct MartialArtistView<'view, 'source>(MartialArtistViewSwitch<'view, 'source>);

impl<'view, 'source> MartialArtistView<'view, 'source> {
    /// The style's name.
    pub fn name(&self) -> &'source str {
        self.0.name()
    }

    /// The book reference for the style.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    /// The style's description.
    pub fn description(&self) -> &'source str {
        self.0.description()
    }

    /// All of the base weapon Ids usable by the style. This is the base weapon
    /// (e.g. "sword" or "daiklave"), not any specific unique artifact weapon.
    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.0.usable_weapon_ids()
    }

    /// If the style is usable with armor, gives the heaviest weight category
    /// allowed.
    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        self.0.max_armor_weight()
    }

    /// The number of ability dots the character possesses in the skill.
    pub fn dots(&self) -> u8 {
        self.0.dots()
    }

    /// Any specialties the character has in this Martial Arts style.
    pub fn specialties(&self) -> impl Iterator<Item = &'source str> {
        self.0.specialties()
    }

    /// All of the Charms the character has for this style.
    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'source MartialArtsCharm)> + '_ {
        self.0.charms()
    }
}

/// A specific Martial Arts style as known by a character.
pub struct MartialArtist<'char>(MartialArtistSwitch<'char>);

impl<'char> MartialArtist<'char> {
    /// The style's name.
    pub fn name(&self) -> &'char str {
        self.0.name()
    }

    /// The book reference for the style.
    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    /// The style's description.
    pub fn description(&self) -> &'char str {
        self.0.description()
    }

    /// All of the base weapon Ids usable by the style. This is the base weapon
    /// (e.g. "sword" or "daiklave"), not any specific unique artifact weapon.
    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.0.usable_weapon_ids()
    }

    /// If the style is usable with armor, gives the heaviest weight category
    /// allowed.
    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        self.0.max_armor_weight()
    }

    /// The number of ability dots the character possesses in the skill.
    pub fn dots(&self) -> u8 {
        self.0.dots()
    }

    /// Any specialties the character has in this Martial Arts style.
    pub fn specialties(&self) -> impl Iterator<Item = &'char str> {
        self.0.specialties()
    }

    /// All of the Charms the character has for this style.
    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'char MartialArtsCharm)> + '_ {
        self.0.charms()
    }
}

pub(crate) enum MartialArtistViewSwitch<'view, 'source> {
    Mortal(&'view MortalMartialArtistView<'source>),
    Exalt(&'view ExaltMartialArtistView<'source>),
}

impl<'view, 'source> MartialArtistViewSwitch<'view, 'source> {
    pub fn name(&self) -> &'source str {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.style.name(),
            MartialArtistViewSwitch::Exalt(view) => view.style.name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.style.book_reference(),
            MartialArtistViewSwitch::Exalt(view) => view.style.book_reference(),
        }
    }

    pub fn description(&self) -> &'source str {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.style.description(),
            MartialArtistViewSwitch::Exalt(view) => view.style.description(),
        }
    }

    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.style.usable_weapon_ids(),
            MartialArtistViewSwitch::Exalt(view) => view.style.usable_weapon_ids(),
        }
    }

    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.style.max_armor_weight(),
            MartialArtistViewSwitch::Exalt(view) => view.style.max_armor_weight(),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.ability.dots(),
            MartialArtistViewSwitch::Exalt(view) => view.ability.dots(),
        }
    }

    pub fn specialties(&self) -> impl Iterator<Item = &'source str> {
        match self {
            MartialArtistViewSwitch::Mortal(view) => view.ability.specialties(),
            MartialArtistViewSwitch::Exalt(view) => view.ability.specialties(),
        }
    }

    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'source MartialArtsCharm)> + '_ {
        match self {
            MartialArtistViewSwitch::Mortal(_) => Vec::new().into_iter(),
            MartialArtistViewSwitch::Exalt(view) => view
                .charms
                .iter()
                .map(|(k, v)| (*k, *v))
                .collect::<Vec<(MartialArtsCharmId, &'source MartialArtsCharm)>>()
                .into_iter(),
        }
    }
}

pub(crate) enum MartialArtistSwitch<'char> {
    Mortal(&'char MortalMartialArtist),
    Exalt(&'char ExaltMartialArtist),
}

impl<'char> MartialArtistSwitch<'char> {
    pub fn name(&self) -> &'char str {
        match self {
            MartialArtistSwitch::Mortal(view) => view.style.name(),
            MartialArtistSwitch::Exalt(view) => view.style.name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            MartialArtistSwitch::Mortal(view) => view.style.book_reference(),
            MartialArtistSwitch::Exalt(view) => view.style.book_reference(),
        }
    }

    pub fn description(&self) -> &'char str {
        match self {
            MartialArtistSwitch::Mortal(view) => view.style.description(),
            MartialArtistSwitch::Exalt(view) => view.style.description(),
        }
    }

    pub fn usable_weapon_ids(&self) -> impl Iterator<Item = WeaponId> + '_ {
        match self {
            MartialArtistSwitch::Mortal(view) => view.style.usable_weapon_ids(),
            MartialArtistSwitch::Exalt(view) => view.style.usable_weapon_ids(),
        }
    }

    pub fn max_armor_weight(&self) -> Option<ArmorWeight> {
        match self {
            MartialArtistSwitch::Mortal(view) => view.style.max_armor_weight(),
            MartialArtistSwitch::Exalt(view) => view.style.max_armor_weight(),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            MartialArtistSwitch::Mortal(view) => view.ability.dots(),
            MartialArtistSwitch::Exalt(view) => view.ability.dots(),
        }
    }

    pub fn specialties(&self) -> impl Iterator<Item = &'char str> {
        match self {
            MartialArtistSwitch::Mortal(view) => view.ability.specialties(),
            MartialArtistSwitch::Exalt(view) => view.ability.specialties(),
        }
    }

    pub fn charms(
        &self,
    ) -> impl Iterator<Item = (MartialArtsCharmId, &'char MartialArtsCharm)> + '_ {
        match self {
            MartialArtistSwitch::Mortal(_) => Vec::new().into_iter(),
            MartialArtistSwitch::Exalt(view) => view
                .charms
                .iter()
                .map(|(k, v)| (*k, v))
                .collect::<Vec<(MartialArtsCharmId, &'char MartialArtsCharm)>>()
                .into_iter(),
        }
    }
}

impl<'view, 'source> ExaltStateView<'source> {
    fn martial_artist(
        &'view self,
        id: MartialArtsStyleId,
    ) -> Option<MartialArtistView<'view, 'source>> {
        match self {
            ExaltStateView::Mortal(mortal) => Some(MartialArtistView(
                MartialArtistViewSwitch::Mortal(mortal.martial_arts_styles.get(&id)?),
            )),
            ExaltStateView::Exalt(exalt) => Some(MartialArtistView(
                MartialArtistViewSwitch::Exalt(exalt.martial_arts_styles.get(&id)?),
            )),
        }
    }

    fn martial_arts_id_iter(&'view self) -> impl Iterator<Item = MartialArtsStyleId> {
        match self {
            ExaltStateView::Mortal(mortal) => mortal
                .martial_arts_styles
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
            ExaltStateView::Exalt(exalt) => exalt
                .martial_arts_styles
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
        }
    }
}

impl<'char> ExaltState {
    fn martial_artist(&'char self, id: MartialArtsStyleId) -> Option<MartialArtist<'char>> {
        match self {
            ExaltState::Mortal(mortal) => Some(MartialArtist(MartialArtistSwitch::Mortal(
                mortal.martial_arts_styles.get(&id)?,
            ))),
            ExaltState::Exalt(exalt) => Some(MartialArtist(MartialArtistSwitch::Exalt(
                exalt.martial_arts_styles.get(&id)?,
            ))),
        }
    }

    fn martial_arts_id_iter(&'char self) -> impl Iterator<Item = MartialArtsStyleId> {
        match self {
            ExaltState::Mortal(mortal) => mortal
                .martial_arts_styles
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
            ExaltState::Exalt(exalt) => exalt
                .martial_arts_styles
                .keys()
                .copied()
                .collect::<Vec<MartialArtsStyleId>>()
                .into_iter(),
        }
    }
}
