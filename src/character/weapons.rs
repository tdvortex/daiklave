use crate::range_bands::RangeBand;
use eyre::{eyre, Result};
use std::{
    collections::HashSet,
    hash::Hash,
    iter::{Enumerate, FusedIterator},
    ops::Deref,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Quality {
    Improvised,
    Mundane,
    Exceptional,
    Artifact,
}

#[derive(Debug)]
pub enum DamageType {
    Bashing,
    Lethal,
}

#[derive(Debug)]
pub enum WeightClass {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(PartialEq, Eq, Hash, Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct WeaponDetails {
    name: String,
    quality: Quality,
    weight_class: WeightClass,
    damage_type: DamageType,
    attack_methods: AttackMethods,
    tags: HashSet<Tag>,
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Clone, Copy)]
pub struct WeaponPosition<'a> {
    equipped: bool,
    one_handed: bool,
    index: usize,
    details: &'a WeaponDetails,
}

pub struct EquippedIter<'a> {
    first: Option<WeaponPosition<'a>>,
    second: Option<WeaponPosition<'a>>,
}

impl Equipped {
    fn iter(&self) -> EquippedIter {
        match self {
            Equipped::HandsFree => EquippedIter {
                first: None,
                second: None,
            },
            Equipped::MainHandOnly(weapon) => EquippedIter {
                first: Some(WeaponPosition {
                    equipped: true,
                    one_handed: true,
                    index: 0,
                    details: weapon.deref(),
                }),
                second: None,
            },
            Equipped::TwoDifferent(main, off) => EquippedIter {
                first: Some(WeaponPosition {
                    equipped: true,
                    one_handed: true,
                    index: 0,
                    details: main.deref(),
                }),
                second: Some(WeaponPosition {
                    equipped: true,
                    one_handed: true,
                    index: 1,
                    details: off.deref(),
                }),
            },
            Equipped::Paired(each) => EquippedIter {
                first: Some(WeaponPosition {
                    equipped: true,
                    one_handed: true,
                    index: 0,
                    details: each.deref(),
                }),
                second: Some(WeaponPosition {
                    equipped: true,
                    one_handed: true,
                    index: 1,
                    details: each.deref(),
                }),
            },
            Equipped::TwoHanded(both) => EquippedIter {
                first: Some(WeaponPosition {
                    equipped: true,
                    one_handed: true,
                    index: 0,
                    details: both.deref(),
                }),
                second: None,
            },
        }
    }
}

impl<'a> Iterator for EquippedIter<'a> {
    type Item = WeaponPosition<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.first;
        if let Some(weapon_position) = self.second {
            self.first = Some(weapon_position);
            self.second = None;
        }
        out
    }
}

impl<'a> FusedIterator for EquippedIter<'a> {}
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
    pub fn equipped_iter(&self) -> EquippedIter<'_> {
        self.equipped.iter()
    }

    pub fn iter(&self) -> WeaponsIter<'_> {
        WeaponsIter {
            in_hands_iter: self.equipped.iter(),
            unequipped_one_handed_enumerate: self.unequipped_one_handed.iter().enumerate(),
            unequipped_two_handed_enumerate: self.unequipped_two_handed.iter().enumerate(),
        }
    }

    fn add_weapon(&mut self, weapon: WeaponDetails, two_handed: bool) {
        if two_handed {
            self.unequipped_two_handed.push(TwoHandedWeapon(weapon));
        } else {
            self.unequipped_one_handed.push(OneHandedWeapon(weapon));
        }
    }
}

pub struct WeaponsIter<'a> {
    in_hands_iter: EquippedIter<'a>,
    unequipped_one_handed_enumerate: Enumerate<std::slice::Iter<'a, OneHandedWeapon>>,
    unequipped_two_handed_enumerate: Enumerate<std::slice::Iter<'a, TwoHandedWeapon>>,
}

impl<'a> Iterator for WeaponsIter<'a> {
    type Item = WeaponPosition<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(weapon) = self.in_hands_iter.next() {
            Some(weapon)
        } else if let Some((index, one_handed_weapon)) = self.unequipped_one_handed_enumerate.next()
        {
            Some(WeaponPosition {
                equipped: false,
                one_handed: true,
                index,
                details: one_handed_weapon.deref(),
            })
        } else if let Some((index, two_handed_weapon)) = self.unequipped_two_handed_enumerate.next()
        {
            Some(WeaponPosition {
                equipped: false,
                one_handed: true,
                index,
                details: two_handed_weapon.deref(),
            })
        } else {
            None
        }
    }
}
