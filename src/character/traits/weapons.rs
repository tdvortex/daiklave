use std::collections::HashSet;

use eyre::{eyre, Result};
use slab::Slab;

use super::range_bands::RangeBand;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum WeightClass {
    Light,
    Medium,
    Heavy,
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

#[derive(Debug, Clone)]
pub struct Weapon {
    id: Option<i32>,
    name: String,
    weight_class: WeightClass,
    is_two_handed: bool,
    damage_type: DamageType,
    main_attack_method: MainAttackMethod,
    martial_arts_styles: HashSet<String>,
    other_tags: HashSet<OtherTag>,
    creator_id: Option<i32>,
}

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

        Ok(Weapon {
            id,
            name,
            weight_class: weight_class.unwrap(),
            is_two_handed: two_handed.unwrap(),
            damage_type: damage_type.unwrap(),
            main_attack_method,
            martial_arts_styles,
            other_tags,
            creator_id,
        })
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn creator_id(&self) -> Option<i32> {
        self.creator_id
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

    pub fn tags(&self) -> HashSet<WeaponTag> {
        let mut output = HashSet::<WeaponTag>::new();

        if self.is_two_handed {
            output.insert(WeaponTag::TwoHanded);
        } else {
            output.insert(WeaponTag::OneHanded);
        }

        let details = self;
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
}

#[derive(Debug, Default)]
struct OneHandedWeapon(pub usize);
#[derive(Debug, Default)]
struct TwoHandedWeapon(pub usize);

#[derive(Debug, Clone, Copy)]
pub enum EquipHand {
    Main,
    Off,
    Both,
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

#[derive(Debug, Default)]
pub struct Weapons(WeaponsPrivate);

#[derive(Debug)]
enum WeaponsPrivate {
    NoEquipped(Slab<Weapon>),
    MainHandOnly(OneHandedWeapon, Slab<Weapon>),
    OffHandOnly(OneHandedWeapon, Slab<Weapon>),
    Paired(OneHandedWeapon, Slab<Weapon>),
    TwoDifferent(OneHandedWeapon, OneHandedWeapon, Slab<Weapon>),
    TwoHanded(TwoHandedWeapon, Slab<Weapon>),
}

impl Default for WeaponsPrivate {
    fn default() -> Self {
        Self::NoEquipped(Slab::default())
    }
}

impl Weapons {
    fn inventory(&self) -> &Slab<Weapon> {
        match &self.0 {
            WeaponsPrivate::NoEquipped(slab)
            | WeaponsPrivate::MainHandOnly(_, slab)
            | WeaponsPrivate::OffHandOnly(_, slab)
            | WeaponsPrivate::Paired(_, slab)
            | WeaponsPrivate::TwoDifferent(_, _, slab)
            | WeaponsPrivate::TwoHanded(_, slab) => slab,
        }
    }

    fn inventory_mut(&mut self) -> &mut Slab<Weapon> {
        match &mut self.0 {
            WeaponsPrivate::NoEquipped(slab)
            | WeaponsPrivate::MainHandOnly(_, slab)
            | WeaponsPrivate::OffHandOnly(_, slab)
            | WeaponsPrivate::Paired(_, slab)
            | WeaponsPrivate::TwoDifferent(_, _, slab)
            | WeaponsPrivate::TwoHanded(_, slab) => slab,
        }
    }

    fn equipped_keys(&self) -> (Option<usize>, Option<usize>) {
        match &self.0 {
            WeaponsPrivate::NoEquipped(_) => (None, None),
            WeaponsPrivate::MainHandOnly(weapon, _) => (Some(weapon.0), None),
            WeaponsPrivate::OffHandOnly(weapon, _) => (None, Some(weapon.0)),
            WeaponsPrivate::Paired(weapon, _) => (Some(weapon.0), Some(weapon.0)),
            WeaponsPrivate::TwoDifferent(main_weapon, off_weapon, _) => {
                (Some(main_weapon.0), Some(off_weapon.0))
            }
            WeaponsPrivate::TwoHanded(weapon, _) => (Some(weapon.0), Some(weapon.0)),
        }
    }

    pub fn get(&self, key: usize) -> Result<&Weapon> {
        self.inventory()
            .get(key)
            .ok_or_else(|| eyre!("weapon {} not found", key))
    }

    pub fn add_weapon(&mut self, weapon: Weapon) -> usize {
        self.inventory_mut().insert(weapon)
    }

    pub fn equip(&mut self, key: usize, hand: EquipHand) -> Result<()> {
        if !self.inventory().contains(key) {
            return Err(eyre!("weapon {} not found", key));
        }

        if self.inventory().get(key).unwrap().is_two_handed {
            *self = Weapons(WeaponsPrivate::TwoHanded(
                TwoHandedWeapon(key),
                std::mem::take(self.inventory_mut()),
            ));
        } else {
            *self = match (std::mem::take(&mut self.0), hand) {
                (WeaponsPrivate::NoEquipped(slab), EquipHand::Main)
                | (WeaponsPrivate::MainHandOnly(_, slab), EquipHand::Main)
                | (WeaponsPrivate::TwoHanded(_, slab), EquipHand::Main) => {
                    Weapons(WeaponsPrivate::MainHandOnly(OneHandedWeapon(key), slab))
                }
                (WeaponsPrivate::NoEquipped(slab), EquipHand::Off)
                | (WeaponsPrivate::OffHandOnly(_, slab), EquipHand::Off)
                | (WeaponsPrivate::TwoHanded(_, slab), EquipHand::Off) => {
                    Weapons(WeaponsPrivate::OffHandOnly(OneHandedWeapon(key), slab))
                }
                (WeaponsPrivate::NoEquipped(slab), EquipHand::Both) => {
                    Weapons(WeaponsPrivate::Paired(OneHandedWeapon(key), slab))
                }
                (WeaponsPrivate::MainHandOnly(main_weapon, slab), EquipHand::Off)
                | (WeaponsPrivate::Paired(main_weapon, slab), EquipHand::Off)
                | (WeaponsPrivate::TwoDifferent(main_weapon, _, slab), EquipHand::Off) => {
                    if main_weapon.0 == key {
                        Weapons(WeaponsPrivate::Paired(main_weapon, slab))
                    } else {
                        Weapons(WeaponsPrivate::TwoDifferent(
                            main_weapon,
                            OneHandedWeapon(key),
                            slab,
                        ))
                    }
                }
                (WeaponsPrivate::OffHandOnly(off_weapon, slab), EquipHand::Main)
                | (WeaponsPrivate::Paired(off_weapon, slab), EquipHand::Main)
                | (WeaponsPrivate::TwoDifferent(_, off_weapon, slab), EquipHand::Main) => {
                    if off_weapon.0 == key {
                        Weapons(WeaponsPrivate::Paired(off_weapon, slab))
                    } else {
                        Weapons(WeaponsPrivate::TwoDifferent(
                            OneHandedWeapon(key),
                            off_weapon,
                            slab,
                        ))
                    }
                }
                (WeaponsPrivate::MainHandOnly(_, slab), EquipHand::Both)
                | (WeaponsPrivate::Paired(_, slab), EquipHand::Both)
                | (WeaponsPrivate::OffHandOnly(_, slab), EquipHand::Both)
                | (WeaponsPrivate::TwoDifferent(_, _, slab), EquipHand::Both)
                | (WeaponsPrivate::TwoHanded(_, slab), EquipHand::Both) => {
                    Weapons(WeaponsPrivate::Paired(OneHandedWeapon(key), slab))
                }
            };
        }
        Ok(())
    }

    pub fn unequip_main(&mut self) {
        match &mut self.0 {
            WeaponsPrivate::NoEquipped(_) | WeaponsPrivate::OffHandOnly(_, _) => { /* do nothing */
            }
            WeaponsPrivate::MainHandOnly(_, slab) | WeaponsPrivate::TwoHanded(_, slab) => {
                *self = Weapons(WeaponsPrivate::NoEquipped(std::mem::take(slab)));
            }
            WeaponsPrivate::Paired(weapon, slab)
            | WeaponsPrivate::TwoDifferent(_, weapon, slab) => {
                *self = Weapons(WeaponsPrivate::OffHandOnly(
                    std::mem::take(weapon),
                    std::mem::take(slab),
                ));
            }
        }
    }

    pub fn unequip_off(&mut self) {
        match &mut self.0 {
            WeaponsPrivate::NoEquipped(_) | WeaponsPrivate::OffHandOnly(_, _) => { /* do nothing */
            }
            WeaponsPrivate::MainHandOnly(_, slab) | WeaponsPrivate::TwoHanded(_, slab) => {
                *self = Weapons(WeaponsPrivate::NoEquipped(std::mem::take(slab)));
            }
            WeaponsPrivate::Paired(weapon, slab)
            | WeaponsPrivate::TwoDifferent(_, weapon, slab) => {
                *self = Weapons(WeaponsPrivate::OffHandOnly(
                    std::mem::take(weapon),
                    std::mem::take(slab),
                ));
            }
        }
    }

    pub fn remove_weapon(&mut self, key: usize) -> bool {
        match self.equipped_keys() {
            (None, None) => { /* do nothing */ }
            (None, Some(off_key)) => {
                if off_key == key {
                    self.unequip_off();
                }
            }
            (Some(main_key), None) => {
                if main_key == key {
                    self.unequip_main();
                }
            }
            (Some(main_key), Some(off_key)) => {
                if main_key == key {
                    self.unequip_main();
                }
                if off_key == key {
                    self.unequip_off();
                }
            }
        }

        self.inventory_mut().try_remove(key).is_some()
    }

    pub fn iter(&self) -> WeaponsIter<'_> {
        WeaponsIter {
            weapons: self,
            slab_iter: self.inventory().iter(),
        }
    }
}

pub struct WeaponsIter<'a> {
    weapons: &'a Weapons,
    slab_iter: slab::Iter<'a, Weapon>,
}

impl<'a> Iterator for WeaponsIter<'a> {
    type Item = (usize, Option<EquipHand>, &'a Weapon);

    fn next(&mut self) -> Option<Self::Item> {
        let (key, weapon) = self.slab_iter.next()?;
        match self.weapons.equipped_keys() {
            (None, None) => Some((key, None, weapon)),
            (None, Some(equipped_key)) => {
                if key == equipped_key {
                    Some((key, Some(EquipHand::Off), weapon))
                } else {
                    Some((key, None, weapon))
                }
            }
            (Some(equipped_key), None) => {
                if key == equipped_key {
                    Some((key, Some(EquipHand::Main), weapon))
                } else {
                    Some((key, None, weapon))
                }
            }
            (Some(main_key), Some(off_key)) => match (main_key == key, off_key == key) {
                (true, true) => Some((key, Some(EquipHand::Both), weapon)),
                (true, false) => Some((key, Some(EquipHand::Main), weapon)),
                (false, true) => Some((key, Some(EquipHand::Off), weapon)),
                (false, false) => Some((key, None, weapon)),
            },
        }
    }
}
