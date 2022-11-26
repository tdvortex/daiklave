use crate::range_bands::RangeBand;
use std::{collections::HashSet, iter::FusedIterator};
use eyre::{eyre, Result};

#[derive(Debug, PartialEq, Eq)]
enum Quality {
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
enum WeightClass {
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
    fn martial_arts(style: String) -> Self {
        Self::MartialArts(style)
    }
}

#[derive(Debug)]
struct AttackMethods {
    default_attack_method: AttackMethod,
    alternate_attack_methods: HashSet<AttackMethod>,
}

impl AttackMethods {
    fn contains(&self, attack_method: &AttackMethod) -> bool {
        self.default_attack_method == *attack_method
            || self.alternate_attack_methods.contains(attack_method)
    }
}

pub trait Weapon {
    fn name(&self) -> &str;
    fn accuracy(&self, attack_method: &AttackMethod) -> Result<i8>;
    fn damage(&self, attack_method: &AttackMethod) -> i8;
    fn damage_type(&self) -> &DamageType;
    fn overwhelming(&self) -> i8;
    fn attunement(&self) -> i8;
    fn defense(&self) -> Option<i8>;
    fn has_tag(&self, tag: &Tag) -> bool;
}

#[derive(Debug)]
pub struct WeaponDetails {
    name: String,
    quality: Quality,
    weight_class: WeightClass,
    damage_type: DamageType,
    attack_methods: AttackMethods,
    tags: HashSet<Tag>,
}

impl Weapon for WeaponDetails {
    fn name(&self) -> &str {
        self.name.as_str()
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

    fn damage_type(&self) -> &DamageType {
        &self.damage_type
    }

    fn has_tag(&self, tag: &Tag) -> bool {
        self.tags.contains(tag)
    }
}

#[derive(Debug)]
pub struct OneHandedWeapon(WeaponDetails);

impl Weapon for OneHandedWeapon {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn accuracy(&self, attack_method: &AttackMethod) -> Result<i8> {
        self.0.accuracy(attack_method)
    }

    fn attunement(&self) -> i8 {
        self.0.attunement()
    }

    fn damage(&self, attack_method: &AttackMethod) -> i8 {
        self.0.damage(attack_method)
    }

    fn damage_type(&self) -> &DamageType {
        self.0.damage_type()
    }

    fn overwhelming(&self) -> i8 {
        self.0.overwhelming()
    }

    fn defense(&self) -> Option<i8> {
        self.0.defense()
    }

    fn has_tag(&self, tag: &Tag) -> bool {
        self.0.has_tag(tag)
    }
}
#[derive(Debug)]
pub struct TwoHandedWeapon(WeaponDetails);

impl Weapon for TwoHandedWeapon {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn accuracy(&self, attack_method: &AttackMethod) -> Result<i8> {
        self.0.accuracy(attack_method)
    }

    fn attunement(&self) -> i8 {
        self.0.attunement()
    }

    fn damage(&self, attack_method: &AttackMethod) -> i8 {
        self.0.damage(attack_method)
    }

    fn damage_type(&self) -> &DamageType {
        self.0.damage_type()
    }

    fn overwhelming(&self) -> i8 {
        self.0.overwhelming()
    }

    fn defense(&self) -> Option<i8> {
        self.0.defense()
    }

    fn has_tag(&self, tag: &Tag) -> bool {
        self.0.has_tag(tag)
    }
}

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

pub struct EquippedIter<'a> {
    first: Option<&'a dyn Weapon>,
    second: Option<&'a dyn Weapon>,
}

impl Equipped {
    fn iter(&self) -> EquippedIter {
        match self {
            Equipped::HandsFree => EquippedIter { first: None, second: None },
            Equipped::MainHandOnly(weapon) => EquippedIter { first: Some(weapon), second: None },
            Equipped::TwoDifferent(main, off) => EquippedIter { first: Some(main), second: Some(off) },
            Equipped::Paired(each) => EquippedIter { first: Some(each), second: Some(each) },
            Equipped::TwoHanded(both) => EquippedIter { first: Some(both), second: None},
        }
    }
}

impl<'a> Iterator for EquippedIter<'a> {
    type Item = &'a dyn Weapon;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.first;
        self.first = self.second;
        self.second = None;
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
            attack_methods: AttackMethods {
                default_attack_method: AttackMethod::Brawl,
                alternate_attack_methods: [
                    "Air Dragon Style".to_owned(),
                    "Black Claw Style".to_owned(),
                    "Centipede Style".to_owned(),
                    "Crane Style".to_owned(),
                    "Earth Dragon Style".to_owned(),
                    "Falcon Style".to_owned(),
                    "Fire Dragon Style".to_owned(),
                    "Laughing Monster Style".to_owned(),
                    "Snake Style".to_owned(),
                    "Swaying Grass Dance Style".to_owned(),
                    "Tiger Style".to_owned(),
                    "Water Dragon Style".to_owned(),
                    "White Reaper Style".to_owned(),
                    "Wood Dragon Style".to_owned(),
                ]
                .into_iter()
                .map(AttackMethod::martial_arts)
                .collect(),
            },
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
            unequipped_one_handed_iter: self.unequipped_one_handed.iter(), 
            unequipped_two_handed_iter: self.unequipped_two_handed.iter(), 
        }
    }
}


pub struct WeaponsIter<'a> {
    in_hands_iter: EquippedIter<'a>,
    unequipped_one_handed_iter: std::slice::Iter<'a, OneHandedWeapon>,
    unequipped_two_handed_iter: std::slice::Iter<'a, TwoHandedWeapon>,
}

impl<'a> Iterator for WeaponsIter<'a> {
    type Item = &'a dyn Weapon;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(weapon) = self.in_hands_iter.next() {
            Some(weapon)
        } else if let Some(weapon) = self.unequipped_one_handed_iter.next() {
            Some(&*weapon)
        } else if let Some(weapon) = self.unequipped_two_handed_iter.next() {
            Some(&*weapon)
        } else {
            None
        }
    }
}