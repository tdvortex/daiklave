pub(crate) mod update;
use serde::{Deserialize, Serialize};
pub use update::WeaponsDiff;
pub(crate) mod create;
pub(crate) mod destroy;
pub use destroy::destroy_weapons;
pub(crate) mod tables;
use std::hash::Hash;
use std::{cmp::Ordering, collections::HashSet};

use eyre::{eyre, Result};

use crate::data_source::{BookReference, DataSource};
use crate::id::Id;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
enum WeightClass {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
enum DamageType {
    Bashing,
    Lethal,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RangeBand {
    Close,
    Short,
    Medium,
    Long,
    Extreme,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
enum OtherTag {
    Artifact,
    Balanced,
    Chopping,
    Concealable,
    Crossbow,
    Cutting,
    Disarming,
    Exceptional,
    Flame,
    Flexible,
    Grappling,
    Improvised,
    Mounted,
    Natural,
    Piercing,
    Poisonable,
    Powerful,
    Reaching,
    Shield,
    Slow,
    Smashing,
    Special,
    Subtle,
    Worn,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WeaponTag {
    Archery(RangeBand),
    Artifact,
    Balanced,
    Bashing,
    Brawl,
    Chopping,
    Concealable,
    Crossbow,
    Cutting,
    Disarming,
    Exceptional,
    Flame,
    Flexible,
    Grappling,
    Heavy,
    Improvised,
    Lethal,
    Light,
    MartialArts(String),
    Medium,
    Melee,
    Mounted,
    Natural,
    OneHanded,
    Piercing,
    Poisonable,
    Powerful,
    Reaching,
    Shield,
    Slow,
    Smashing,
    Special,
    Subtle,
    Thrown(RangeBand),
    TwoHanded,
    Worn,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Weapon {
    id: Id,
    name: String,
    weight_class: WeightClass,
    is_two_handed: bool,
    damage_type: DamageType,
    main_attack_method: MainAttackMethod,
    martial_arts_styles: Vec<String>,
    other_tags: Vec<OtherTag>,
    data_source: DataSource,
}

impl PartialEq for Weapon {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Weapon {
    fn new(
        name: String,
        tags: HashSet<WeaponTag>,
        id: Id,
        data_source: DataSource,
    ) -> Result<Weapon> {
        let mut two_handed = None::<bool>;
        let mut weight_class = None::<WeightClass>;
        let mut damage_type = None::<DamageType>;
        let mut archery = None::<RangeBand>;
        let mut thrown = None::<RangeBand>;
        let mut brawl = false;
        let mut melee = false;
        let mut martial_arts_styles = Vec::<String>::new();
        let mut other_tags = Vec::<OtherTag>::new();

        for tag in tags {
            match tag {
                WeaponTag::Archery(range) => {
                    if let Some(old_range) = archery {
                        archery = Some(old_range.max(range));
                    } else {
                        archery = Some(range);
                    }
                }
                WeaponTag::Artifact => {
                    other_tags.push(OtherTag::Artifact);
                }
                WeaponTag::Balanced => {
                    other_tags.push(OtherTag::Balanced);
                }
                WeaponTag::Bashing => {
                    if let Some(other_type) = damage_type {
                        if other_type != DamageType::Bashing {
                            return Err(eyre!("weapons must be exactly one of Bashing or Lethal"));
                        }
                    }
                    damage_type = Some(DamageType::Bashing);
                }
                WeaponTag::Brawl => {
                    brawl = true;
                }
                WeaponTag::Chopping => {
                    other_tags.push(OtherTag::Chopping);
                }
                WeaponTag::Concealable => {
                    other_tags.push(OtherTag::Concealable);
                }
                WeaponTag::Crossbow => {
                    other_tags.push(OtherTag::Crossbow);
                }
                WeaponTag::Cutting => {
                    other_tags.push(OtherTag::Cutting);
                }
                WeaponTag::Disarming => {
                    other_tags.push(OtherTag::Disarming);
                }
                WeaponTag::Exceptional => {
                    other_tags.push(OtherTag::Exceptional);
                }
                WeaponTag::Flame => {
                    other_tags.push(OtherTag::Flame);
                }
                WeaponTag::Flexible => {
                    other_tags.push(OtherTag::Flexible);
                }
                WeaponTag::Grappling => {
                    other_tags.push(OtherTag::Grappling);
                }
                WeaponTag::Heavy => {
                    if let Some(other_class) = weight_class {
                        if other_class != WeightClass::Heavy {
                            return Err(eyre!(
                                "weapons must be exactly one of Light, Medium, or Heavy"
                            ));
                        }
                    }
                    weight_class = Some(WeightClass::Heavy);
                }
                WeaponTag::Improvised => {
                    other_tags.push(OtherTag::Improvised);
                }
                WeaponTag::Lethal => {
                    if let Some(other_type) = damage_type {
                        if other_type != DamageType::Lethal {
                            return Err(eyre!("weapons must be exactly one of Bashing or Lethal"));
                        }
                    }
                    damage_type = Some(DamageType::Lethal);
                }
                WeaponTag::Light => {
                    if let Some(other_class) = weight_class {
                        if other_class != WeightClass::Light {
                            return Err(eyre!(
                                "weapons must be exactly one of Light, Medium, or Heavy"
                            ));
                        }
                    }
                    weight_class = Some(WeightClass::Light);
                }
                WeaponTag::MartialArts(style) => {
                    martial_arts_styles.push(style);
                }
                WeaponTag::Medium => {
                    if let Some(other_class) = weight_class {
                        if other_class != WeightClass::Medium {
                            return Err(eyre!(
                                "weapons must be exactly one of Light, Medium, or Heavy"
                            ));
                        }
                    }
                    weight_class = Some(WeightClass::Medium);
                }
                WeaponTag::Melee => {
                    melee = true;
                }
                WeaponTag::Mounted => {
                    other_tags.push(OtherTag::Mounted);
                }
                WeaponTag::OneHanded => {
                    if let Some(two) = two_handed {
                        if two {
                            return Err(eyre!(
                                "weapons must be exactly one of OneHanded or TwoHanded"
                            ));
                        }
                    }
                    two_handed = Some(false);
                }
                WeaponTag::Natural => {
                    other_tags.push(OtherTag::Natural);
                }
                WeaponTag::Piercing => {
                    other_tags.push(OtherTag::Piercing);
                }
                WeaponTag::Poisonable => {
                    other_tags.push(OtherTag::Poisonable);
                }
                WeaponTag::Powerful => {
                    other_tags.push(OtherTag::Powerful);
                }
                WeaponTag::Reaching => {
                    other_tags.push(OtherTag::Reaching);
                }
                WeaponTag::Shield => {
                    other_tags.push(OtherTag::Shield);
                }
                WeaponTag::Slow => {
                    other_tags.push(OtherTag::Slow);
                }
                WeaponTag::Smashing => {
                    other_tags.push(OtherTag::Smashing);
                }
                WeaponTag::Special => {
                    other_tags.push(OtherTag::Special);
                }
                WeaponTag::Subtle => {
                    other_tags.push(OtherTag::Subtle);
                }
                WeaponTag::Thrown(range) => {
                    if let Some(old_range) = thrown {
                        thrown = Some(old_range.max(range));
                    } else {
                        thrown = Some(range);
                    }
                }
                WeaponTag::TwoHanded => {
                    if let Some(two) = two_handed {
                        if !two {
                            return Err(eyre!(
                                "weapons must be exactly one of OneHanded or TwoHanded"
                            ));
                        }
                    }
                    two_handed = Some(true);
                }
                WeaponTag::Worn => {
                    other_tags.push(OtherTag::Worn);
                }
            }
        }

        if two_handed.is_none() {
            return Err(eyre!(
                "weapons must be exactly one of OneHanded or TwoHanded"
            ));
        }

        if weight_class.is_none() {
            return Err(eyre!(
                "weapons must be exactly one of Light, Medium, or Heavy"
            ));
        }

        if damage_type.is_none() {
            return Err(eyre!("weapons must be exactly one of Bashing or Lethal"));
        }

        let main_attack_method = match (
            archery,
            thrown,
            brawl,
            melee,
            !martial_arts_styles.is_empty(),
        ) {
            (Some(_), Some(_), _, _, _) => {
                return Err(eyre!("weapons may not be both Archery and Thrown"));
            }
            (Some(_), _, true, _, _) | (Some(_), _, _, true, _) => {
                return Err(eyre!("Archery weapons may not be Brawl or Melee"));
            }
            (_, Some(_), true, _, _) => {
                return Err(eyre!("Brawl weapons may not be Thrown"));
            }
            (_, _, true, true, _) => {
                return Err(eyre!("weapons cannot be both Brawl and Melee"));
            }
            (None, None, false, false, false) => {
                return Err(eyre!("weapons must have at least one of Archery, Thrown, Brawl, Melee, or Martial Arts"));
            }
            (Some(range), None, false, false, _) => MainAttackMethod::Archery(range),
            (None, None, true, false, _) => MainAttackMethod::Brawl,
            (None, None, false, true, _) => MainAttackMethod::Melee,
            (None, Some(range), false, true, _) => MainAttackMethod::MeleeAndThrown(range),
            (None, Some(range), false, false, _) => MainAttackMethod::ThrownOnly(range),
            (None, None, false, false, true) => MainAttackMethod::MartialArtsOnly,
        };

        other_tags.sort();

        Ok(Weapon {
            id,
            name,
            weight_class: weight_class.unwrap(),
            is_two_handed: two_handed.unwrap(),
            damage_type: damage_type.unwrap(),
            main_attack_method,
            martial_arts_styles,
            other_tags,
            data_source,
        })
    }

    pub fn from_book(id: Id, book_title: String, page_number: i16) -> WeaponBuilder {
        WeaponBuilder {
            id,
            name: Default::default(),
            two_handed: Default::default(),
            is_lethal: Default::default(),
            weight_class_tag: Default::default(),
            attack_tags: Default::default(),
            other_tags: Default::default(),
            data_source: DataSource::Book(BookReference {
                book_title,
                page_number,
            }),
        }
    }

    pub fn custom(id: Id, creator_id: Id) -> WeaponBuilder {
        WeaponBuilder {
            id,
            name: Default::default(),
            two_handed: Default::default(),
            is_lethal: Default::default(),
            weight_class_tag: Default::default(),
            attack_tags: Default::default(),
            other_tags: Default::default(),
            data_source: DataSource::Custom(creator_id),
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn data_source(&self) -> &DataSource {
        &self.data_source
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn weight_class(&self) -> WeaponTag {
        match self.weight_class {
            WeightClass::Light => WeaponTag::Light,
            WeightClass::Medium => WeaponTag::Medium,
            WeightClass::Heavy => WeaponTag::Heavy,
        }
    }

    pub fn is_two_handed(&self) -> bool {
        self.is_two_handed
    }

    pub fn damage_type(&self) -> WeaponTag {
        match self.damage_type {
            DamageType::Bashing => WeaponTag::Bashing,
            DamageType::Lethal => WeaponTag::Lethal,
        }
    }

    pub fn accuracy(&self, range: RangeBand) -> Option<i8> {
        let base_accuracy: i8 = match (range, self.main_attack_method, self.weight_class) {
            (RangeBand::Close, MainAttackMethod::Archery(_), _) => Some(-2),
            (RangeBand::Close, MainAttackMethod::ThrownOnly(_), _) => Some(4),
            (RangeBand::Close, _, WeightClass::Light) => Some(4),
            (RangeBand::Close, _, WeightClass::Medium) => Some(2), // Assumption: Medium, MeleeAndThrown weapons are wielded as melee in Close
            (RangeBand::Close, _, WeightClass::Heavy) => Some(0),
            (RangeBand::Short, MainAttackMethod::Archery(range), _) => {
                if range >= RangeBand::Short {
                    Some(4)
                } else {
                    None
                }
            }
            (RangeBand::Medium, MainAttackMethod::Archery(range), _) => {
                if range >= RangeBand::Medium {
                    Some(2)
                } else {
                    None
                }
            }
            (RangeBand::Long, MainAttackMethod::Archery(range), _) => {
                if range >= RangeBand::Long {
                    Some(0)
                } else {
                    None
                }
            }
            (RangeBand::Extreme, MainAttackMethod::Archery(range), _) => {
                if range >= RangeBand::Extreme {
                    Some(-2)
                } else {
                    None
                }
            }
            (RangeBand::Short, MainAttackMethod::ThrownOnly(range), _)
            | (RangeBand::Short, MainAttackMethod::MeleeAndThrown(range), _) => {
                if range >= RangeBand::Short {
                    Some(3)
                } else {
                    None
                }
            }
            (RangeBand::Medium, MainAttackMethod::ThrownOnly(range), _)
            | (RangeBand::Medium, MainAttackMethod::MeleeAndThrown(range), _) => {
                if range >= RangeBand::Medium {
                    Some(2)
                } else {
                    None
                }
            }
            (RangeBand::Long, MainAttackMethod::ThrownOnly(range), _)
            | (RangeBand::Long, MainAttackMethod::MeleeAndThrown(range), _) => {
                if range >= RangeBand::Long {
                    Some(-1)
                } else {
                    None
                }
            }
            (RangeBand::Extreme, MainAttackMethod::ThrownOnly(range), _)
            | (RangeBand::Extreme, MainAttackMethod::MeleeAndThrown(range), _) => {
                if range >= RangeBand::Extreme {
                    Some(-3)
                } else {
                    None
                }
            }
            (_, _, _) => None,
        }?;

        let exceptional_bonus = i8::from(
            self.other_tags.contains(&OtherTag::Artifact)
                || self.other_tags.contains(&OtherTag::Exceptional),
        );
        let flame_bonus = if self.other_tags.contains(&OtherTag::Flame) {
            if let MainAttackMethod::Archery(_) = self.main_attack_method {
                2
            } else {
                0
            }
        } else {
            0
        };

        Some(base_accuracy + exceptional_bonus + flame_bonus)
    }

    pub fn damage(&self) -> i8 {
        // Ignoring Powerful tag to keep API simple--only applies for crossbows at close range
        let base_damage = match self.weight_class {
            WeightClass::Light => 7,
            WeightClass::Medium => 9,
            WeightClass::Heavy => 11,
        };

        let artifact_bonus = 3 * i8::from(self.other_tags.contains(&OtherTag::Artifact));
        let shield_penalty = -2 * i8::from(self.other_tags.contains(&OtherTag::Shield));

        base_damage + artifact_bonus + shield_penalty
    }

    pub fn defense(&self) -> Option<i8> {
        match self.main_attack_method {
            MainAttackMethod::Archery(_) | MainAttackMethod::ThrownOnly(_) => None,
            _ => match self.weight_class {
                WeightClass::Light => Some(0),
                WeightClass::Medium => Some(1),
                WeightClass::Heavy => {
                    if self.other_tags.contains(&OtherTag::Artifact) {
                        Some(0)
                    } else {
                        Some(-1)
                    }
                }
            },
        }
    }

    pub fn attunement(&self) -> u8 {
        if self.other_tags.contains(&OtherTag::Artifact) {
            5
        } else {
            0
        }
    }

    pub fn overwhelming(&self) -> i8 {
        match (
            self.other_tags.contains(&OtherTag::Artifact),
            self.weight_class,
        ) {
            (true, WeightClass::Light) => 3,
            (true, WeightClass::Medium) => 4,
            (true, WeightClass::Heavy) => 5,
            (false, _) => 1,
        }
    }

    pub fn tags(&self) -> Vec<WeaponTag> {
        let mut output = Vec::<WeaponTag>::new();

        if self.is_two_handed {
            output.push(WeaponTag::TwoHanded);
        } else {
            output.push(WeaponTag::OneHanded);
        }

        let details = self;
        match details.weight_class {
            WeightClass::Light => {
                output.push(WeaponTag::Light);
            }
            WeightClass::Medium => {
                output.push(WeaponTag::Medium);
            }
            WeightClass::Heavy => {
                output.push(WeaponTag::Heavy);
            }
        }
        match details.damage_type {
            DamageType::Bashing => {
                output.push(WeaponTag::Bashing);
            }
            DamageType::Lethal => {
                output.push(WeaponTag::Lethal);
            }
        }
        match details.main_attack_method {
            MainAttackMethod::Archery(range) => {
                output.push(WeaponTag::Archery(range));
            }
            MainAttackMethod::Brawl => {
                output.push(WeaponTag::Brawl);
            }
            MainAttackMethod::Melee => {
                output.push(WeaponTag::Melee);
            }
            MainAttackMethod::MeleeAndThrown(range) => {
                output.push(WeaponTag::Melee);
                output.push(WeaponTag::Thrown(range));
            }
            MainAttackMethod::MartialArtsOnly => {}
            MainAttackMethod::ThrownOnly(range) => {
                output.push(WeaponTag::Thrown(range));
            }
        }

        for style in details.martial_arts_styles.iter() {
            output.push(WeaponTag::MartialArts(style.clone()));
        }

        for other_tag in &details.other_tags {
            let tag = match other_tag {
                OtherTag::Artifact => WeaponTag::Artifact,
                OtherTag::Balanced => WeaponTag::Balanced,
                OtherTag::Chopping => WeaponTag::Chopping,
                OtherTag::Concealable => WeaponTag::Concealable,
                OtherTag::Crossbow => WeaponTag::Crossbow,
                OtherTag::Cutting => WeaponTag::Cutting,
                OtherTag::Disarming => WeaponTag::Disarming,
                OtherTag::Exceptional => WeaponTag::Exceptional,
                OtherTag::Flame => WeaponTag::Flame,
                OtherTag::Flexible => WeaponTag::Flexible,
                OtherTag::Grappling => WeaponTag::Grappling,
                OtherTag::Improvised => WeaponTag::Improvised,
                OtherTag::Mounted => WeaponTag::Mounted,
                OtherTag::Natural => WeaponTag::Natural,
                OtherTag::Piercing => WeaponTag::Piercing,
                OtherTag::Poisonable => WeaponTag::Poisonable,
                OtherTag::Powerful => WeaponTag::Powerful,
                OtherTag::Reaching => WeaponTag::Reaching,
                OtherTag::Shield => WeaponTag::Shield,
                OtherTag::Slow => WeaponTag::Slow,
                OtherTag::Smashing => WeaponTag::Smashing,
                OtherTag::Special => WeaponTag::Special,
                OtherTag::Subtle => WeaponTag::Subtle,
                OtherTag::Worn => WeaponTag::Worn,
            };
            output.push(tag);
        }

        output.sort();
        output
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipHand {
    Main,
    Off,
    Both,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
enum MainAttackMethod {
    Archery(RangeBand),
    Brawl,
    Melee,
    MeleeAndThrown(RangeBand),
    MartialArtsOnly,
    ThrownOnly(RangeBand),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Weapons {
    inventory: Vec<(Weapon, Option<EquipHand>)>,
}

impl Weapons {
    pub fn get_by_ref(&self, weapon: &Weapon) -> Option<(usize, &Weapon, Option<EquipHand>)> {
        self.inventory
            .iter()
            .enumerate()
            .find_map(|(index, (weapon_ref, maybe_equip_hand))| {
                if weapon_ref == weapon {
                    Some((index, weapon_ref, *maybe_equip_hand))
                } else {
                    None
                }
            })
    }

    fn get_by_index_unchecked(&self, index: usize) -> (usize, &Weapon, Option<EquipHand>) {
        let (weapon_ref, maybe_equip_hand) = &self.inventory[index];
        (index, weapon_ref, *maybe_equip_hand)
    }

    pub fn get_by_index(&self, index: usize) -> Option<(usize, &Weapon, Option<EquipHand>)> {
        if index >= self.inventory.len() {
            None
        } else {
            Some(self.get_by_index_unchecked(index))
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &Weapon, Option<EquipHand>)> {
        (0..self.inventory.len()).map(|index| self.get_by_index(index).unwrap())
    }

    pub fn add_weapon(&mut self, weapon: Weapon) -> usize {
        let insert_index = self
            .inventory
            .binary_search_by(|(inventory_weapon, hand)| {
                if inventory_weapon == &weapon {
                    if hand.is_none() {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                } else {
                    inventory_weapon.name().cmp(weapon.name())
                }
            })
            .map_or_else(|i| i, |i| i);
        self.inventory.insert(insert_index, (weapon, None));
        insert_index
    }

    pub fn remove_weapon(&mut self, index: usize) -> Result<(Weapon, Option<EquipHand>)> {
        if index >= self.inventory.len() {
            Err(eyre!("Weapon index {} not found", index))
        } else {
            Ok(self.inventory.remove(index))
        }
    }

    fn unequip_main(&mut self) {
        self.inventory
            .iter_mut()
            .for_each(|(weapon, maybe_equip_hand)| {
                match (weapon.is_two_handed(), std::mem::take(maybe_equip_hand)) {
                    (false, Some(EquipHand::Both)) => *maybe_equip_hand = Some(EquipHand::Off),
                    (false, Some(EquipHand::Main)) | (true, _) => {
                        *maybe_equip_hand = None;
                    }
                    (false, Some(EquipHand::Off)) | (_, None) => {}
                };
            });
    }

    fn unequip_off(&mut self) {
        self.inventory
            .iter_mut()
            .for_each(|(weapon, maybe_equip_hand)| {
                match (weapon.is_two_handed(), std::mem::take(maybe_equip_hand)) {
                    (false, Some(EquipHand::Both)) => *maybe_equip_hand = Some(EquipHand::Main),
                    (false, Some(EquipHand::Off)) | (true, _) => {
                        *maybe_equip_hand = None;
                    }
                    (false, Some(EquipHand::Main)) | (_, None) => {}
                };
            });
    }

    fn unequip_both(&mut self) {
        self.inventory.iter_mut().for_each(|(_, maybe_equip_hand)| {
            *maybe_equip_hand = None;
        });
    }

    pub fn unequip(&mut self, hand: EquipHand) {
        match hand {
            EquipHand::Main => self.unequip_main(),
            EquipHand::Off => self.unequip_off(),
            EquipHand::Both => self.unequip_both(),
        }
    }

    pub fn equip(&mut self, index: usize, hand: EquipHand) -> Result<()> {
        if index >= self.inventory.len() {
            Err(eyre!("Weapon index {} not found", index))
        } else if self.inventory[index].0.is_two_handed() && hand != EquipHand::Both {
            Err(eyre!("Weapon index {} requires two hands to wield", index))
        } else {
            self.unequip(hand);
            self.inventory[index].1 = Some(hand);
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct WeaponBuilder {
    id: Id,
    name: Option<String>,
    two_handed: bool,
    is_lethal: bool,
    weight_class_tag: Option<WeaponTag>,
    attack_tags: Vec<WeaponTag>,
    other_tags: Vec<WeaponTag>,
    data_source: DataSource,
}

impl WeaponBuilder {
    pub(crate) fn with_database_id(mut self, id: i32) -> Self {
        self.id = Id::Database(id);
        self
    }

    pub fn with_name(mut self, name: String) -> WeaponBuilder {
        self.name = Some(name);
        self
    }

    pub fn dealing_bashing(mut self) -> WeaponBuilder {
        self.is_lethal = false;
        self
    }

    pub fn dealing_lethal(mut self) -> WeaponBuilder {
        self.is_lethal = true;
        self
    }

    pub fn as_one_handed(mut self) -> WeaponBuilder {
        self.two_handed = false;
        self
    }

    pub fn as_two_handed(mut self) -> WeaponBuilder {
        self.two_handed = true;
        self
    }

    pub fn as_archery_with_range(mut self, max_range: RangeBand) -> WeaponBuilder {
        self.attack_tags = std::mem::take(&mut self.attack_tags)
            .into_iter()
            .filter_map(|tag| match tag {
                WeaponTag::MartialArts(style) => Some(WeaponTag::MartialArts(style)),
                _ => None,
            })
            .collect();

        self.attack_tags.push(WeaponTag::Archery(max_range));
        self
    }

    pub fn as_brawl(mut self) -> WeaponBuilder {
        self.attack_tags = std::mem::take(&mut self.attack_tags)
            .into_iter()
            .filter_map(|tag| match tag {
                WeaponTag::MartialArts(style) => Some(WeaponTag::MartialArts(style)),
                _ => None,
            })
            .collect();

        self.attack_tags.push(WeaponTag::Brawl);
        self
    }

    pub fn as_melee(mut self) -> WeaponBuilder {
        self.attack_tags = std::mem::take(&mut self.attack_tags)
            .into_iter()
            .filter_map(|tag| match tag {
                WeaponTag::MartialArts(style) => Some(WeaponTag::MartialArts(style)),
                WeaponTag::Thrown(range) => Some(WeaponTag::Thrown(range)),
                _ => None,
            })
            .collect();

        self.attack_tags.push(WeaponTag::Melee);
        self
    }

    pub fn with_thrown_range(mut self, max_range: RangeBand) -> WeaponBuilder {
        self.attack_tags = std::mem::take(&mut self.attack_tags)
            .into_iter()
            .filter_map(|tag| match tag {
                WeaponTag::MartialArts(style) => Some(WeaponTag::MartialArts(style)),
                WeaponTag::Melee => Some(WeaponTag::Melee),
                _ => None,
            })
            .collect();

        self.attack_tags.push(WeaponTag::Thrown(max_range));
        self
    }

    pub fn with_martial_arts(mut self, style: String) -> WeaponBuilder {
        self.attack_tags.push(WeaponTag::MartialArts(style));
        self
    }

    pub fn as_light(mut self) -> WeaponBuilder {
        self.weight_class_tag = Some(WeaponTag::Light);
        self
    }

    pub fn as_medium(mut self) -> WeaponBuilder {
        self.weight_class_tag = Some(WeaponTag::Medium);
        self
    }

    pub fn as_heavy(mut self) -> WeaponBuilder {
        self.weight_class_tag = Some(WeaponTag::Heavy);
        self
    }

    pub fn as_artifact(mut self) -> WeaponBuilder {
        self.other_tags.push(WeaponTag::Artifact);
        self
    }

    pub fn with_tag(mut self, tag: WeaponTag) -> WeaponBuilder {
        match tag {
            WeaponTag::Archery(range) => self.as_archery_with_range(range),
            WeaponTag::Artifact => self.as_artifact(),
            WeaponTag::Bashing => self.dealing_bashing(),
            WeaponTag::Brawl => self.as_brawl(),
            WeaponTag::Heavy => self.as_heavy(),
            WeaponTag::Lethal => self.dealing_lethal(),
            WeaponTag::Light => self.as_light(),
            WeaponTag::MartialArts(style) => self.with_martial_arts(style),
            WeaponTag::Medium => self.as_medium(),
            WeaponTag::Melee => self.as_melee(),
            WeaponTag::OneHanded => self.as_one_handed(),
            WeaponTag::Thrown(range) => self.with_thrown_range(range),
            WeaponTag::TwoHanded => self.as_two_handed(),
            other_tag => {
                self.other_tags.push(other_tag);
                self
            }
        }
    }

    pub fn build(self) -> Result<Weapon> {
        if self.name.is_none() {
            return Err(eyre!("weapon name is required"));
        }

        let mut tags = Vec::new();
        tags.push(
            self.weight_class_tag
                .ok_or_else(|| eyre!("weapons must be exactly one of Light, Medium, or Heavy"))?,
        );

        if self.two_handed {
            tags.push(WeaponTag::TwoHanded)
        } else {
            tags.push(WeaponTag::OneHanded)
        };

        if self.is_lethal {
            tags.push(WeaponTag::Lethal)
        } else {
            tags.push(WeaponTag::Bashing)
        };

        tags.extend(self.attack_tags.into_iter());
        tags.extend(self.other_tags.into_iter());

        Weapon::new(
            self.name.unwrap(),
            tags.into_iter().collect(),
            self.id,
            self.data_source,
        )
    }
}
