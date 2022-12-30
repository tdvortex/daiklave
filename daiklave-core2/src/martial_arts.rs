use std::{ops::Deref, collections::{HashSet, HashMap}};

use serde::{Serialize, Deserialize};

use crate::{id::UniqueId, book_reference::BookReference, weapons::WeaponId, armor::ArmorWeight, abilities::{Ability, AbilityView}, charms::{CharmKeyword, CharmCost, CharmActionType}, CharacterView, CharacterMutationError};

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

impl<'source> CharacterView<'source> {
    /// Checks if a Martial Arts style can be added to the character.
    pub fn check_add_martial_arts_style(&self, id: MartialArtsStyleId, style: &MartialArtsStyle) -> Result<(), CharacterMutationError> {
        todo!()
    }

    /// Checks if a Martial Arts style can be added to the character.
    pub fn add_martial_arts_style(&mut self, id: MartialArtsStyleId, style: &MartialArtsStyle) -> Result<&mut Self, CharacterMutationError> {
        todo!()
    }
}