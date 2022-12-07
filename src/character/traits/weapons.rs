use slab::Slab;
use std::{collections::HashSet, ops::Deref};

use super::range_bands::RangeBand;
use eyre::{eyre, Result};

// Weapons are constructed and displayed as a collection of Tags
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    OneHanded,
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
    Thrown(RangeBand),
    TwoHanded,
    Worn,
}
// Weapon API
#[derive(Debug)]
pub struct Weapon(Handedness);

impl Weapon {
    pub fn new(
        name: String,
        tags: HashSet<WeaponTag>,
        id: Option<i32>,
        creator_id: Option<i32>,
    ) -> Result<Weapon> {
        let mut two_handed = None::<bool>;
        let mut weight_class = None::<WeightClass>;
        let mut damage_type = None::<DamageType>;
        let mut archery = None::<RangeBand>;
        let mut thrown = None::<RangeBand>;
        let mut brawl = false;
        let mut melee = false;
        let mut martial_arts_styles = HashSet::<String>::new();
        let mut other_tags = HashSet::<OtherTag>::new();

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
                    other_tags.insert(OtherTag::Artifact);
                }
                WeaponTag::Balanced => {
                    other_tags.insert(OtherTag::Balanced);
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
                    other_tags.insert(OtherTag::Chopping);
                }
                WeaponTag::Concealable => {
                    other_tags.insert(OtherTag::Concealable);
                }
                WeaponTag::Crossbow => {
                    other_tags.insert(OtherTag::Crossbow);
                }
                WeaponTag::Cutting => {
                    other_tags.insert(OtherTag::Cutting);
                }
                WeaponTag::Disarming => {
                    other_tags.insert(OtherTag::Disarming);
                }
                WeaponTag::Exceptional => {
                    other_tags.insert(OtherTag::Exceptional);
                }
                WeaponTag::Flame => {
                    other_tags.insert(OtherTag::Flame);
                }
                WeaponTag::Flexible => {
                    other_tags.insert(OtherTag::Flexible);
                }
                WeaponTag::Grappling => {
                    other_tags.insert(OtherTag::Grappling);
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
                    other_tags.insert(OtherTag::Improvised);
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
                    martial_arts_styles.insert(style);
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
                    other_tags.insert(OtherTag::Mounted);
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
                    other_tags.insert(OtherTag::Natural);
                }
                WeaponTag::Piercing => {
                    other_tags.insert(OtherTag::Piercing);
                }
                WeaponTag::Poisonable => {
                    other_tags.insert(OtherTag::Poisonable);
                }
                WeaponTag::Powerful => {
                    other_tags.insert(OtherTag::Powerful);
                }
                WeaponTag::Reaching => {
                    other_tags.insert(OtherTag::Reaching);
                }
                WeaponTag::Shield => {
                    other_tags.insert(OtherTag::Shield);
                }
                WeaponTag::Slow => {
                    other_tags.insert(OtherTag::Slow);
                }
                WeaponTag::Smashing => {
                    other_tags.insert(OtherTag::Smashing);
                }
                WeaponTag::Special => {
                    other_tags.insert(OtherTag::Special);
                }
                WeaponTag::Subtle => {
                    other_tags.insert(OtherTag::Subtle);
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
                    other_tags.insert(OtherTag::Worn);
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

        let details = WeaponDetails {
            _id: id,
            name,
            weight_class: weight_class.unwrap(),
            damage_type: damage_type.unwrap(),
            main_attack_method,
            martial_arts_styles,
            other_tags,
            creator_id,
        };
        if two_handed.unwrap() {
            Ok(Weapon(Handedness::TwoHanded(details)))
        } else {
            Ok(Weapon(Handedness::OneHanded(details)))
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.0._id
    }

    fn details(&self) -> &WeaponDetails {
        &self.0
    }

    pub fn name(&self) -> &str {
        self.details().name.as_str()
    }

    pub fn accuracy(&self, range: RangeBand) -> Option<i8> {
        let base_accuracy: i8 = match (
            range,
            self.details().main_attack_method,
            self.details().weight_class,
        ) {
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
            self.details().other_tags.contains(&OtherTag::Artifact)
                || self.details().other_tags.contains(&OtherTag::Exceptional),
        );
        let flame_bonus = if self.details().other_tags.contains(&OtherTag::Flame) {
            if let MainAttackMethod::Archery(_) = self.details().main_attack_method {
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
        let base_damage = match self.details().weight_class {
            WeightClass::Light => 7,
            WeightClass::Medium => 9,
            WeightClass::Heavy => 11,
        };

        let artifact_bonus = 3 * i8::from(self.details().other_tags.contains(&OtherTag::Artifact));
        let shield_penalty = -2 * i8::from(self.details().other_tags.contains(&OtherTag::Shield));

        base_damage + artifact_bonus + shield_penalty
    }

    pub fn defense(&self) -> Option<i8> {
        match self.details().main_attack_method {
            MainAttackMethod::Archery(_) | MainAttackMethod::ThrownOnly(_) => None,
            _ => match self.details().weight_class {
                WeightClass::Light => Some(0),
                WeightClass::Medium => Some(1),
                WeightClass::Heavy => {
                    if self.details().other_tags.contains(&OtherTag::Artifact) {
                        Some(0)
                    } else {
                        Some(-1)
                    }
                }
            },
        }
    }

    pub fn attunement(&self) -> u8 {
        if self.details().other_tags.contains(&OtherTag::Artifact) {
            5
        } else {
            0
        }
    }

    pub fn overwhelming(&self) -> i8 {
        match (
            self.details().other_tags.contains(&OtherTag::Artifact),
            self.details().weight_class,
        ) {
            (true, WeightClass::Light) => 3,
            (true, WeightClass::Medium) => 4,
            (true, WeightClass::Heavy) => 5,
            (false, _) => 1,
        }
    }

    pub fn tags(&self) -> HashSet<WeaponTag> {
        let mut output = HashSet::<WeaponTag>::new();

        match self.0 {
            Handedness::OneHanded(_) => {
                output.insert(WeaponTag::OneHanded);
            }
            Handedness::TwoHanded(_) => {
                output.insert(WeaponTag::TwoHanded);
            }
        }

        let details = self.details();
        match details.weight_class {
            WeightClass::Light => {
                output.insert(WeaponTag::Light);
            }
            WeightClass::Medium => {
                output.insert(WeaponTag::Medium);
            }
            WeightClass::Heavy => {
                output.insert(WeaponTag::Heavy);
            }
        }
        match details.damage_type {
            DamageType::Bashing => {
                output.insert(WeaponTag::Bashing);
            }
            DamageType::Lethal => {
                output.insert(WeaponTag::Lethal);
            }
        }
        match details.main_attack_method {
            MainAttackMethod::Archery(range) => {
                output.insert(WeaponTag::Archery(range));
            }
            MainAttackMethod::Brawl => {
                output.insert(WeaponTag::Brawl);
            }
            MainAttackMethod::Melee => {
                output.insert(WeaponTag::Melee);
            }
            MainAttackMethod::MeleeAndThrown(range) => {
                output.insert(WeaponTag::Melee);
                output.insert(WeaponTag::Thrown(range));
            }
            MainAttackMethod::MartialArtsOnly => {}
            MainAttackMethod::ThrownOnly(range) => {
                output.insert(WeaponTag::Thrown(range));
            }
        }

        for style in details.martial_arts_styles.iter() {
            output.insert(WeaponTag::MartialArts(style.clone()));
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
            output.insert(tag);
        }

        output
    }

    pub fn creator_id(&self) -> Option<i32> {
        self.0.creator_id
    }
}

#[derive(Debug)]
struct WeaponDetails {
    _id: Option<i32>,
    name: String,
    weight_class: WeightClass,
    damage_type: DamageType,
    main_attack_method: MainAttackMethod,
    martial_arts_styles: HashSet<String>,
    other_tags: HashSet<OtherTag>,
    creator_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum WeightClass {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum MainAttackMethod {
    Archery(RangeBand),
    Brawl,
    Melee,
    MeleeAndThrown(RangeBand),
    MartialArtsOnly,
    ThrownOnly(RangeBand),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum DamageType {
    Bashing,
    Lethal,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug)]
enum Handedness {
    OneHanded(WeaponDetails),
    TwoHanded(WeaponDetails),
}

impl Deref for Handedness {
    type Target = WeaponDetails;

    fn deref(&self) -> &Self::Target {
        match self {
            Handedness::OneHanded(details) | Handedness::TwoHanded(details) => details,
        }
    }
}

#[derive(Debug)]
enum Equipped {
    None,
    MainHandOnly(usize),
    OffHandOnly(usize),
    Paired(usize),
    TwoDifferent(usize, usize),
    TwoHanded(usize),
}

impl Default for Equipped {
    fn default() -> Self {
        Self::None
    }
}

pub enum EquipHand {
    Main,
    Off,
    Both,
}

#[derive(Debug, Default)]
pub struct Weapons {
    equipped: Equipped,
    owned: Slab<Weapon>,
}

struct EquippedIter<'a> {
    weapons: &'a Weapons,
    first: Option<usize>,
    second: Option<usize>,
}

impl<'a> Iterator for EquippedIter<'a> {
    type Item = (usize, &'a Weapon);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.first?;
        self.first = self.second;
        self.second = None;
        Some((key, self.weapons.owned.get(key).unwrap()))
    }
}

impl Weapons {
    pub fn equipped_iter(&self) -> impl Iterator<Item = (usize, &Weapon)> {
        let (first, second) = match self.equipped {
            Equipped::None => (None, None),
            Equipped::MainHandOnly(key) => (Some(key), None),
            Equipped::OffHandOnly(key) => (Some(key), None),
            Equipped::Paired(key) => (Some(key), Some(key)),
            Equipped::TwoDifferent(key1, key2) => (Some(key1), Some(key2)),
            Equipped::TwoHanded(key) => (Some(key), None),
        };

        EquippedIter {
            weapons: self,
            first,
            second,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &Weapon)> {
        self.owned.iter()
    }

    pub fn get(&self, key: usize) -> Result<&Weapon> {
        self.owned
            .get(key)
            .ok_or_else(|| eyre!("weapon {} not found", key))
    }

    pub fn equip(&mut self, key: usize, hand: EquipHand) -> Result<()> {
        let weapon = self
            .owned
            .get(key)
            .ok_or_else(|| eyre!("weapon {} not found", key))?;
        self.equipped = match (&weapon.0, hand, &self.equipped) {
            (Handedness::OneHanded(_), EquipHand::Main, Equipped::None)
            | (Handedness::OneHanded(_), EquipHand::Main, Equipped::MainHandOnly(_))
            | (Handedness::OneHanded(_), EquipHand::Main, Equipped::TwoHanded(_)) => {
                Equipped::MainHandOnly(key)
            }
            (Handedness::OneHanded(_), EquipHand::Off, Equipped::None)
            | (Handedness::OneHanded(_), EquipHand::Off, Equipped::OffHandOnly(_))
            | (Handedness::OneHanded(_), EquipHand::Off, Equipped::TwoHanded(_)) => {
                Equipped::OffHandOnly(key)
            }
            (Handedness::OneHanded(_), EquipHand::Main, Equipped::OffHandOnly(other))
            | (Handedness::OneHanded(_), EquipHand::Main, Equipped::Paired(other))
            | (Handedness::OneHanded(_), EquipHand::Main, Equipped::TwoDifferent(_, other)) => {
                if key == *other {
                    Equipped::Paired(key)
                } else {
                    Equipped::TwoDifferent(key, *other)
                }
            }
            (Handedness::OneHanded(_), EquipHand::Off, Equipped::MainHandOnly(other))
            | (Handedness::OneHanded(_), EquipHand::Off, Equipped::Paired(other))
            | (Handedness::OneHanded(_), EquipHand::Off, Equipped::TwoDifferent(other, _)) => {
                if key == *other {
                    Equipped::Paired(key)
                } else {
                    Equipped::TwoDifferent(*other, key)
                }
            }
            (Handedness::OneHanded(_), EquipHand::Both, _) => Equipped::Paired(key),
            (Handedness::TwoHanded(_), _, _) => Equipped::TwoHanded(key),
        };
        Ok(())
    }

    pub fn unequip(&mut self, hand: EquipHand) {
        match (&self.equipped, hand) {
            (Equipped::None, _)
            | (Equipped::MainHandOnly(_), EquipHand::Off)
            | (Equipped::OffHandOnly(_), EquipHand::Main) => {}
            (_, EquipHand::Both)
            | (Equipped::TwoHanded(_), _)
            | (Equipped::MainHandOnly(_), EquipHand::Main)
            | (Equipped::OffHandOnly(_), EquipHand::Off) => {
                self.equipped = Equipped::None;
            }
            (Equipped::Paired(key), EquipHand::Main)
            | (Equipped::TwoDifferent(_, key), EquipHand::Main) => {
                self.equipped = Equipped::OffHandOnly(*key);
            }
            (Equipped::Paired(key), EquipHand::Off)
            | (Equipped::TwoDifferent(key, _), EquipHand::Off) => {
                self.equipped = Equipped::MainHandOnly(*key);
            }
        }
    }

    pub fn add_weapon(&mut self, weapon: Weapon) -> usize {
        self.owned.insert(weapon)
    }

    pub fn remove_weapon(&mut self, key: usize) -> Result<()> {
        if !self.owned.contains(key) {
            return Err(eyre!("weapon {} not found", key));
        }

        match self.equipped {
            Equipped::None => {}
            Equipped::MainHandOnly(curr) | Equipped::TwoHanded(curr) => {
                if curr == key {
                    self.unequip(EquipHand::Main);
                }
            }
            Equipped::OffHandOnly(curr) => {
                if curr == key {
                    self.unequip(EquipHand::Off);
                }
            }
            Equipped::Paired(curr) => {
                if curr == key {
                    self.unequip(EquipHand::Both);
                }
            }
            Equipped::TwoDifferent(main, off) => {
                if key == main {
                    self.unequip(EquipHand::Main);
                }
                if key == off {
                    self.unequip(EquipHand::Off)
                }
            }
        }

        self.owned.remove(key);

        Ok(())
    }
}
