use crate::range_bands::RangeBand;
use eyre::{eyre, Result};
use std::{collections::HashSet, hash::Hash, iter::FusedIterator, ops::Deref};

pub trait HasWeapons {
    fn weapons_iter(&self) -> WeaponsIter;
    fn get_weapon_at_position(&self, position: WeaponPosition) -> Result<&WeaponDetails>;
    fn add_weapon(&mut self, weapon: WeaponDetails, two_handed: bool);
    fn remove_weapon(&mut self, position: WeaponPosition) -> Result<()>;
    fn equip_weapon(&mut self, hand: Hand, position: WeaponPosition) -> Result<()>;
    fn unequip_weapon(&mut self, hand: Hand) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Quality {
    Improvised,
    Mundane,
    Exceptional,
    Artifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamageType {
    Bashing,
    Lethal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightClass {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Tag {
    Balanced,
    Chopping,
    Concealable,
    Crossbow,
    Cutting,
    Disarming,
    Flame,
    Flexible,
    Grappling,
    Mounted,
    Natural,
    Piercing,
    Powerful,
    Reaching,
    Shield,
    Slow,
    Smashing,
    Subtle,
    Thrown,
    Worn,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum AttackMethod {
    Brawl,
    Melee,
    Thrown(RangeBand),
    Archery(RangeBand),
    MartialArts(String),
    MartialArtsThrown(String, RangeBand), // ex Silver-Voiced Nightengale Style
    MartialArtsArchery(String, RangeBand), // ex Righteous Devil Style
}

impl AttackMethod {
    pub fn brawl() -> std::vec::IntoIter<AttackMethod> {
        vec![Self::Brawl].into_iter()
    }

    pub fn melee() -> std::vec::IntoIter<AttackMethod> {
        vec![Self::Melee].into_iter()
    }

    pub fn thrown(max_range: RangeBand) -> std::vec::IntoIter<AttackMethod> {
        match max_range {
            RangeBand::Close => vec![Self::Thrown(RangeBand::Close)].into_iter(),
            RangeBand::Short => vec![
                Self::Thrown(RangeBand::Close),
                Self::Thrown(RangeBand::Short),
            ]
            .into_iter(),
            RangeBand::Medium => vec![
                Self::Thrown(RangeBand::Close),
                Self::Thrown(RangeBand::Short),
                Self::Thrown(RangeBand::Medium),
            ]
            .into_iter(),
            RangeBand::Long => vec![
                Self::Thrown(RangeBand::Close),
                Self::Thrown(RangeBand::Short),
                Self::Thrown(RangeBand::Medium),
                Self::Thrown(RangeBand::Long),
            ]
            .into_iter(),
            RangeBand::Extreme => vec![
                Self::Thrown(RangeBand::Close),
                Self::Thrown(RangeBand::Short),
                Self::Thrown(RangeBand::Medium),
                Self::Thrown(RangeBand::Long),
                Self::Thrown(RangeBand::Extreme),
            ]
            .into_iter(),
        }
    }

    pub fn archery(max_range: RangeBand) -> std::vec::IntoIter<AttackMethod> {
        match max_range {
            RangeBand::Close => vec![Self::Archery(RangeBand::Close)].into_iter(),
            RangeBand::Short => vec![
                Self::Archery(RangeBand::Close),
                Self::Archery(RangeBand::Short),
            ]
            .into_iter(),
            RangeBand::Medium => vec![
                Self::Archery(RangeBand::Close),
                Self::Archery(RangeBand::Short),
                Self::Archery(RangeBand::Medium),
            ]
            .into_iter(),
            RangeBand::Long => vec![
                Self::Archery(RangeBand::Close),
                Self::Archery(RangeBand::Short),
                Self::Archery(RangeBand::Medium),
                Self::Archery(RangeBand::Long),
            ]
            .into_iter(),
            RangeBand::Extreme => vec![
                Self::Archery(RangeBand::Close),
                Self::Archery(RangeBand::Short),
                Self::Archery(RangeBand::Medium),
                Self::Archery(RangeBand::Long),
                Self::Archery(RangeBand::Extreme),
            ]
            .into_iter(),
        }
    }

    pub fn martial_arts(style: String) -> std::vec::IntoIter<AttackMethod> {
        vec![Self::MartialArts(style)].into_iter()
    }

    pub fn martial_arts_thrown(
        style: String,
        max_range: RangeBand,
    ) -> std::vec::IntoIter<AttackMethod> {
        match max_range {
            RangeBand::Close => vec![Self::MartialArtsThrown(style, RangeBand::Close)].into_iter(),
            RangeBand::Short => vec![
                Self::MartialArtsThrown(style.clone(), RangeBand::Close),
                Self::MartialArtsThrown(style, RangeBand::Short),
            ]
            .into_iter(),
            RangeBand::Medium => vec![
                Self::MartialArtsThrown(style.clone(), RangeBand::Close),
                Self::MartialArtsThrown(style.clone(), RangeBand::Short),
                Self::MartialArtsThrown(style, RangeBand::Medium),
            ]
            .into_iter(),
            RangeBand::Long => vec![
                Self::MartialArtsThrown(style.clone(), RangeBand::Close),
                Self::MartialArtsThrown(style.clone(), RangeBand::Short),
                Self::MartialArtsThrown(style.clone(), RangeBand::Medium),
                Self::MartialArtsThrown(style, RangeBand::Long),
            ]
            .into_iter(),
            RangeBand::Extreme => vec![
                Self::MartialArtsThrown(style.clone(), RangeBand::Close),
                Self::MartialArtsThrown(style.clone(), RangeBand::Short),
                Self::MartialArtsThrown(style.clone(), RangeBand::Medium),
                Self::MartialArtsThrown(style.clone(), RangeBand::Long),
                Self::MartialArtsThrown(style, RangeBand::Extreme),
            ]
            .into_iter(),
        }
    }

    pub fn martial_arts_archery(
        style: String,
        max_range: RangeBand,
    ) -> std::vec::IntoIter<AttackMethod> {
        match max_range {
            RangeBand::Close => vec![Self::MartialArtsArchery(style, RangeBand::Close)].into_iter(),
            RangeBand::Short => vec![
                Self::MartialArtsArchery(style.clone(), RangeBand::Close),
                Self::MartialArtsArchery(style, RangeBand::Short),
            ]
            .into_iter(),
            RangeBand::Medium => vec![
                Self::MartialArtsArchery(style.clone(), RangeBand::Close),
                Self::MartialArtsArchery(style.clone(), RangeBand::Short),
                Self::MartialArtsArchery(style, RangeBand::Medium),
            ]
            .into_iter(),
            RangeBand::Long => vec![
                Self::MartialArtsArchery(style.clone(), RangeBand::Close),
                Self::MartialArtsArchery(style.clone(), RangeBand::Short),
                Self::MartialArtsArchery(style.clone(), RangeBand::Medium),
                Self::MartialArtsArchery(style, RangeBand::Long),
            ]
            .into_iter(),
            RangeBand::Extreme => vec![
                Self::MartialArtsArchery(style.clone(), RangeBand::Close),
                Self::MartialArtsArchery(style.clone(), RangeBand::Short),
                Self::MartialArtsArchery(style.clone(), RangeBand::Medium),
                Self::MartialArtsArchery(style.clone(), RangeBand::Long),
                Self::MartialArtsArchery(style, RangeBand::Extreme),
            ]
            .into_iter(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttackMethods {
    default_attack_method: AttackMethod,
    alternate_attack_methods: HashSet<AttackMethod>,
}

impl AttackMethods {
    fn contains(&self, attack_method: &AttackMethod) -> bool {
        self.default_attack_method == *attack_method
            || self.alternate_attack_methods.contains(attack_method)
    }

    fn try_from_iter<T: Iterator<Item = AttackMethod>>(mut iter: T) -> Result<Self> {
        if let Some(default_attack_method) = iter.next() {
            let alternate_attack_methods: HashSet<AttackMethod> = iter.collect();
            Ok(Self {
                default_attack_method,
                alternate_attack_methods,
            })
        } else {
            Err(eyre!("must have at least one attack method"))
        }
    }
}

pub trait Weapon: Deref<Target = WeaponDetails> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponDetails {
    name: String,
    quality: Quality,
    weight_class: WeightClass,
    damage_type: DamageType,
    attack_methods: AttackMethods,
    tags: HashSet<Tag>,
}

impl Default for WeaponDetails {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            quality: Quality::Mundane,
            weight_class: WeightClass::Light,
            damage_type: DamageType::Bashing,
            attack_methods: AttackMethods {
                default_attack_method: AttackMethod::Brawl,
                alternate_attack_methods: HashSet::new(),
            },
            tags: HashSet::new(),
        }
    }
}

impl WeaponDetails {
    pub fn new(
        name: String,
        quality: Quality,
        weight_class: WeightClass,
        damage_type: DamageType,
        attack_methods: AttackMethods,
        tags: HashSet<Tag>,
    ) -> Self {
        Self {
            name,
            quality,
            weight_class,
            damage_type,
            attack_methods,
            tags,
        }
    }

    fn has_tag(&self, tag: &Tag) -> bool {
        self.tags.contains(tag)
    }

    fn accuracy(&self, attack_method: &AttackMethod) -> Result<i8> {
        if !self.attack_methods.contains(attack_method) {
            return Err(eyre!("Invalid attack method: {:?}", attack_method));
        }

        let base_accuracy = match (&self.weight_class, attack_method) {
            (WeightClass::Light, AttackMethod::Brawl)
            | (WeightClass::Light, AttackMethod::Melee)
            | (WeightClass::Light, AttackMethod::MartialArts(_)) => 4,
            (WeightClass::Medium, AttackMethod::Brawl)
            | (WeightClass::Medium, AttackMethod::Melee)
            | (WeightClass::Medium, AttackMethod::MartialArts(_)) => 2,
            (WeightClass::Heavy, AttackMethod::Brawl)
            | (WeightClass::Heavy, AttackMethod::Melee)
            | (WeightClass::Heavy, AttackMethod::MartialArts(_)) => 0,
            (_, AttackMethod::Thrown(RangeBand::Close))
            | (_, AttackMethod::MartialArtsThrown(_, RangeBand::Close)) => 4,
            (_, AttackMethod::Thrown(RangeBand::Short))
            | (_, AttackMethod::MartialArtsThrown(_, RangeBand::Short)) => 3,
            (_, AttackMethod::Thrown(RangeBand::Medium))
            | (_, AttackMethod::MartialArtsThrown(_, RangeBand::Medium)) => 2,
            (_, AttackMethod::Thrown(RangeBand::Long))
            | (_, AttackMethod::MartialArtsThrown(_, RangeBand::Long)) => -1,
            (_, AttackMethod::Thrown(RangeBand::Extreme))
            | (_, AttackMethod::MartialArtsThrown(_, RangeBand::Extreme)) => -3,
            (_, AttackMethod::Archery(RangeBand::Close))
            | (_, AttackMethod::MartialArtsArchery(_, RangeBand::Close)) => -2,
            (_, AttackMethod::Archery(RangeBand::Short))
            | (_, AttackMethod::MartialArtsArchery(_, RangeBand::Short)) => 4,
            (_, AttackMethod::Archery(RangeBand::Medium))
            | (_, AttackMethod::MartialArtsArchery(_, RangeBand::Medium)) => 2,
            (_, AttackMethod::Archery(RangeBand::Long))
            | (_, AttackMethod::MartialArtsArchery(_, RangeBand::Long)) => 0,
            (_, AttackMethod::Archery(RangeBand::Extreme))
            | (_, AttackMethod::MartialArtsArchery(_, RangeBand::Extreme)) => -2,
        };

        let exceptional_bonus =
            i8::from(self.quality == Quality::Exceptional || self.quality == Quality::Artifact);
        let flame_bonus = match (self.tags.contains(&Tag::Flame), attack_method) {
            (true, AttackMethod::Archery(RangeBand::Close))
            | (true, AttackMethod::MartialArtsArchery(_, RangeBand::Close)) => 2,
            (_, _) => 0,
        };

        Ok(base_accuracy + exceptional_bonus + flame_bonus)
    }

    fn damage(&self, attack_method: &AttackMethod) -> i8 {
        let effective_weight = match (self.tags.contains(&Tag::Powerful), attack_method) {
            (true, AttackMethod::Archery(RangeBand::Close))
            | (true, AttackMethod::MartialArtsArchery(_, RangeBand::Close)) => &WeightClass::Heavy,
            (_, _) => &self.weight_class,
        };

        let base_damage = match effective_weight {
            WeightClass::Light => 7,
            WeightClass::Medium => 9,
            WeightClass::Heavy => 11,
        };

        let artifact_bonus = 3 * i8::from(self.quality == Quality::Artifact);

        let shield_penalty = match (self.tags.contains(&Tag::Shield), attack_method) {
            (true, AttackMethod::Brawl)
            | (true, AttackMethod::Melee)
            | (true, AttackMethod::MartialArts(_)) => -2,
            _ => 0,
        };

        base_damage + artifact_bonus + shield_penalty
    }

    fn overwhelming(&self) -> i8 {
        let balanced_bonus = 2 * i8::from(self.tags.contains(&Tag::Balanced));

        let base_overwhelming = match (&self.quality, &self.weight_class) {
            (Quality::Artifact, WeightClass::Light) => 3,
            (Quality::Artifact, WeightClass::Medium) => 4,
            (Quality::Artifact, WeightClass::Heavy) => 5,
            (_, _) => 1,
        };

        base_overwhelming + balanced_bonus
    }

    fn attunement(&self) -> i8 {
        5 * i8::from(self.quality == Quality::Artifact)
    }

    fn defense(&self) -> Option<i8> {
        match (
            &self.attack_methods.default_attack_method,
            &self.weight_class,
            &self.quality,
        ) {
            (AttackMethod::Archery(_), _, _)
            | (AttackMethod::MartialArtsArchery(_, _), _, _)
            | (AttackMethod::Thrown(_), _, _)
            | (AttackMethod::MartialArtsThrown(_, _), _, _) => None,
            (_, WeightClass::Light, _) | (_, WeightClass::Heavy, Quality::Artifact) => Some(0),
            (_, WeightClass::Medium, _) => Some(1),
            (_, WeightClass::Heavy, _) => Some(-1),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct OneHandedWeapon(WeaponDetails);

impl From<WeaponDetails> for OneHandedWeapon {
    fn from(details: WeaponDetails) -> Self {
        OneHandedWeapon(details)
    }
}

impl Deref for OneHandedWeapon {
    type Target = WeaponDetails;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Weapon for OneHandedWeapon {}

#[derive(Default, Debug)]
pub struct TwoHandedWeapon(WeaponDetails);

impl From<WeaponDetails> for TwoHandedWeapon {
    fn from(details: WeaponDetails) -> Self {
        TwoHandedWeapon(details)
    }
}

impl Deref for TwoHandedWeapon {
    type Target = WeaponDetails;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Weapon for TwoHandedWeapon {}

#[derive(Debug)]
enum Equipped {
    HandsFree,
    MainHandOnly(OneHandedWeapon),
    TwoDifferent(OneHandedWeapon, OneHandedWeapon),
    Paired(OneHandedWeapon),
    TwoHanded(TwoHandedWeapon),
}

impl Default for Equipped {
    fn default() -> Self {
        Self::HandsFree
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Hand {
    Main,
    Off,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum WeaponPosition {
    Equipped(Hand),
    UnequippedOneHanded(usize),
    UnequippedTwoHanded(usize),
}

pub struct EquippedIter {
    first: Option<WeaponPosition>,
    second: Option<WeaponPosition>,
}

impl Equipped {
    fn iter(&self) -> EquippedIter {
        match self {
            Equipped::HandsFree => EquippedIter {
                first: None,
                second: None,
            },
            Equipped::MainHandOnly(_) | Equipped::TwoHanded(_) => EquippedIter {
                first: Some(WeaponPosition::Equipped(Hand::Main)),
                second: None,
            },
            Equipped::TwoDifferent(_, _) | Equipped::Paired(_) => EquippedIter {
                first: Some(WeaponPosition::Equipped(Hand::Main)),
                second: Some(WeaponPosition::Equipped(Hand::Off)),
            },
        }
    }
}

impl Iterator for EquippedIter {
    type Item = WeaponPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.first;
        if let Some(weapon_position) = self.second {
            self.first = Some(weapon_position);
            self.second = None;
        }
        out
    }
}

impl FusedIterator for EquippedIter {}
#[derive(Debug)]
pub struct Weapons {
    equipped: Equipped,
    unequipped_one_handed: Vec<OneHandedWeapon>,
    unequipped_two_handed: Vec<TwoHandedWeapon>,
}

impl Default for Weapons {
    fn default() -> Self {
        let unarmed = OneHandedWeapon(WeaponDetails {
            name: "Unarmed".to_owned(),
            quality: Quality::Mundane,
            weight_class: WeightClass::Light,
            damage_type: DamageType::Bashing,
            attack_methods: AttackMethods::try_from_iter(
                AttackMethod::brawl().chain(
                    [
                        "Air Dragon Style",
                        "Black Claw Style",
                        "Centipede Style",
                        "Crane Style",
                        "Earth Dragon Style",
                        "Falcon Style",
                        "Fire Dragon Style",
                        "Laughing Monster Style",
                        "Snake Style",
                        "Swaying Grass Dance Style",
                        "Tiger Style",
                        "Water Dragon Style",
                        "White Reaper Style",
                        "Wood Dragon Style",
                    ]
                    .map(|s| s.to_owned())
                    .into_iter()
                    .flat_map(AttackMethod::martial_arts),
                ),
            )
            .unwrap(),
            tags: [Tag::Grappling, Tag::Natural].into(),
        });

        Self {
            equipped: Equipped::default(),
            unequipped_one_handed: vec![unarmed],
            unequipped_two_handed: Vec::new(),
        }
    }
}

impl Weapons {
    fn equipped_iter(&self) -> EquippedIter {
        self.equipped.iter()
    }

    fn positions_iter(&self) -> WeaponPositionsIter {
        WeaponPositionsIter {
            in_hands_iter: self.equipped.iter(),
            unequipped_one_handed_index_iter: 0..self.unequipped_one_handed.len(),
            unequipped_two_handed_index_iter: 0..self.unequipped_two_handed.len(),
        }
    }

    pub fn iter(&self) -> WeaponsIter {
        WeaponsIter {
            weapons: self,
            positions_iter: self.positions_iter(),
        }
    }

    pub fn get_at_position(&self, position: WeaponPosition) -> Result<&WeaponDetails> {
        match position {
            WeaponPosition::Equipped(Hand::Main) => match &self.equipped {
                Equipped::HandsFree => Err(eyre!("no weapon in main hand")),
                Equipped::MainHandOnly(weapon)
                | Equipped::TwoDifferent(weapon, _)
                | Equipped::Paired(weapon) => Ok(weapon.deref()),
                Equipped::TwoHanded(weapon) => Ok(weapon.deref()),
            },
            WeaponPosition::Equipped(Hand::Off) => match &self.equipped {
                Equipped::HandsFree | Equipped::MainHandOnly(_) => {
                    Err(eyre!("no weapon in off hand"))
                }
                Equipped::TwoDifferent(_, weapon) | Equipped::Paired(weapon) => Ok(weapon.deref()),
                Equipped::TwoHanded(weapon) => Ok(weapon.deref()),
            },
            WeaponPosition::UnequippedOneHanded(index) => {
                if index >= self.unequipped_one_handed.len() {
                    Err(eyre!(
                        "out of bounds index for one-handed weapons: {}",
                        index
                    ))
                } else {
                    Ok(&self.unequipped_one_handed[index])
                }
            }
            WeaponPosition::UnequippedTwoHanded(index) => {
                if index >= self.unequipped_two_handed.len() {
                    Err(eyre!(
                        "out of bounds index for two-handed weapons: {}",
                        index
                    ))
                } else {
                    Ok(&self.unequipped_two_handed[index])
                }
            }
        }
    }

    pub fn add_weapon(&mut self, weapon: WeaponDetails, two_handed: bool) {
        if two_handed {
            self.unequipped_two_handed.push(TwoHandedWeapon(weapon));
        } else {
            self.unequipped_one_handed.push(OneHandedWeapon(weapon));
        }
    }

    pub fn unequip_weapon(&mut self, hand: Hand) -> Result<()> {
        match (hand, &mut self.equipped) {
            (_, Equipped::HandsFree) => Err(eyre!("no equipped weapons to unequip")),
            (Hand::Off, Equipped::MainHandOnly(_)) => {
                Err(eyre!("no equipped off-hand weapon to unequip"))
            }
            (Hand::Main, Equipped::TwoDifferent(main_weapon, off_weapon)) => {
                self.unequipped_one_handed.push(std::mem::take(main_weapon));
                self.equipped = Equipped::MainHandOnly(std::mem::take(off_weapon));
                Ok(())
            }
            (Hand::Off, Equipped::TwoDifferent(main_weapon, off_weapon)) => {
                self.unequipped_one_handed.push(std::mem::take(off_weapon));
                self.equipped = Equipped::MainHandOnly(std::mem::take(main_weapon));
                Ok(())
            }
            (_, Equipped::Paired(each)) => {
                self.unequipped_one_handed.push(each.clone());
                self.equipped = Equipped::MainHandOnly(std::mem::take(each));
                Ok(())
            }
            (_, Equipped::TwoHanded(both)) => {
                self.unequipped_two_handed.push(std::mem::take(both));
                self.equipped = Equipped::HandsFree;
                Ok(())
            }
            (Hand::Main, Equipped::MainHandOnly(main_weapon)) => {
                self.unequipped_one_handed.push(std::mem::take(main_weapon));
                self.equipped = Equipped::HandsFree;
                Ok(())
            }
        }
    }

    pub fn equip_weapon(&mut self, hand: Hand, position: WeaponPosition) -> Result<()> {
        match (hand, position, &mut self.equipped) {
            (_, WeaponPosition::Equipped(_), Equipped::HandsFree) => {
                Err(eyre!("no equipped weapons to re-equip"))
            }
            (_, WeaponPosition::Equipped(Hand::Off), Equipped::MainHandOnly(_)) => {
                Err(eyre!("no off-hand weapon to re-equip"))
            }
            (Hand::Off, WeaponPosition::Equipped(Hand::Main), Equipped::MainHandOnly(_))
            | (Hand::Off, WeaponPosition::UnequippedOneHanded(_), Equipped::HandsFree) => {
                Err(eyre!("single one-handed weapon must be in main hand"))
            }
            (Hand::Main, WeaponPosition::Equipped(Hand::Main), _)
            | (_, WeaponPosition::Equipped(_), Equipped::Paired(_))
            | (Hand::Off, WeaponPosition::Equipped(Hand::Off), Equipped::TwoDifferent(_, _))
            | (_, WeaponPosition::Equipped(_), Equipped::TwoHanded(_)) => {
                Err(eyre!("weapon is already equipped"))
            }
            (
                Hand::Off,
                WeaponPosition::Equipped(Hand::Main),
                Equipped::TwoDifferent(main, off),
            )
            | (
                Hand::Main,
                WeaponPosition::Equipped(Hand::Off),
                Equipped::TwoDifferent(main, off),
            ) => {
                std::mem::swap(main, off);
                Ok(())
            }
            (_, WeaponPosition::UnequippedTwoHanded(index), Equipped::HandsFree) => {
                if index >= self.unequipped_two_handed.len() {
                    Err(eyre!(
                        "out of bounds index for two-handed weapons: {}",
                        index
                    ))
                } else {
                    let weapon = self.unequipped_two_handed.remove(index);
                    self.equipped = Equipped::TwoHanded(weapon);
                    Ok(())
                }
            }
            (_, WeaponPosition::UnequippedTwoHanded(_), _) => {
                Err(eyre!("need both hands free to equip two-handed weapon"))
            }
            (Hand::Main, WeaponPosition::UnequippedOneHanded(index), Equipped::HandsFree) => {
                if index >= self.unequipped_one_handed.len() {
                    Err(eyre!(
                        "out of bounds index for one-handed weapons: {}",
                        index
                    ))
                } else {
                    let weapon = self.unequipped_one_handed.remove(index);
                    self.equipped = Equipped::MainHandOnly(weapon);
                    Ok(())
                }
            }
            (Hand::Main, _, _) => Err(eyre!("main hand already occupied, unequip first")),
            (Hand::Off, _, Equipped::Paired(_))
            | (Hand::Off, _, Equipped::TwoDifferent(_, _))
            | (Hand::Off, _, Equipped::TwoHanded(_)) => {
                Err(eyre!("off hand already occupied, unequip first"))
            }
            (
                Hand::Off,
                WeaponPosition::UnequippedOneHanded(index),
                Equipped::MainHandOnly(main),
            ) => {
                if index >= self.unequipped_one_handed.len() {
                    Err(eyre!(
                        "out of bounds index for one-handed weapons: {}",
                        index
                    ))
                } else {
                    let main = std::mem::take(main);
                    let off = self.unequipped_one_handed.remove(index);
                    if *main == *off {
                        self.equipped = Equipped::Paired(main);
                    } else {
                        self.equipped = Equipped::TwoDifferent(main, off);
                    }
                    Ok(())
                }
            }
        }
    }

    pub fn remove_weapon(&mut self, position: WeaponPosition) -> Result<()> {
        match position {
            WeaponPosition::Equipped(Hand::Main) => match self.equipped {
                Equipped::HandsFree => Err(eyre!("no weapon in main hand to remove")),
                Equipped::MainHandOnly(_) | Equipped::TwoDifferent(_, _) | Equipped::Paired(_) => {
                    self.unequip_weapon(Hand::Main).unwrap();
                    self.unequipped_one_handed.pop().unwrap();
                    Ok(())
                }
                Equipped::TwoHanded(_) => {
                    self.unequip_weapon(Hand::Main).unwrap();
                    self.unequipped_two_handed.pop().unwrap();
                    Ok(())
                }
            },
            WeaponPosition::Equipped(Hand::Off) => match self.equipped {
                Equipped::HandsFree | Equipped::MainHandOnly(_) => {
                    Err(eyre!("no weapon in off hand to remove"))
                }
                Equipped::TwoDifferent(_, _) | Equipped::Paired(_) => {
                    self.unequip_weapon(Hand::Off).unwrap();
                    self.unequipped_one_handed.pop().unwrap();
                    Ok(())
                }
                Equipped::TwoHanded(_) => {
                    self.unequip_weapon(Hand::Off).unwrap();
                    self.unequipped_two_handed.pop().unwrap();
                    Ok(())
                }
            },
            WeaponPosition::UnequippedOneHanded(index) => {
                if index >= self.unequipped_one_handed.len() {
                    Err(eyre!(
                        "out of bounds index for one-handed weapons: {}",
                        index
                    ))
                } else {
                    self.unequipped_one_handed.remove(index);
                    Ok(())
                }
            }
            WeaponPosition::UnequippedTwoHanded(index) => {
                if index >= self.unequipped_two_handed.len() {
                    Err(eyre!(
                        "out of bounds index for two-handed weapons: {}",
                        index
                    ))
                } else {
                    self.unequipped_two_handed.remove(index);
                    Ok(())
                }
            }
        }
    }
}

struct WeaponPositionsIter {
    in_hands_iter: EquippedIter,
    unequipped_one_handed_index_iter: std::ops::Range<usize>,
    unequipped_two_handed_index_iter: std::ops::Range<usize>,
}

impl Iterator for WeaponPositionsIter {
    type Item = WeaponPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(weapon) = self.in_hands_iter.next() {
            Some(weapon)
        } else if let Some(index) = self.unequipped_one_handed_index_iter.next() {
            Some(WeaponPosition::UnequippedOneHanded(index))
        } else {
            self.unequipped_two_handed_index_iter
                .next()
                .map(WeaponPosition::UnequippedTwoHanded)
        }
    }
}

pub struct WeaponsIter<'a> {
    weapons: &'a Weapons,
    positions_iter: WeaponPositionsIter,
}

impl<'a> Iterator for WeaponsIter<'a> {
    type Item = (WeaponPosition, &'a WeaponDetails);

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.positions_iter.next()?;
        Some((position, self.weapons.get_at_position(position).unwrap()))
    }
}
