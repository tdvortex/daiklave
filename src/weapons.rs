use std::collections::{HashSet};

use crate::RangeBand;
use eyre::{eyre, Result};

// Weapons are constructed and displayed as a collection of Tags
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
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
    Special(String),
    Subtle,
    Thrown(RangeBand),
    TwoHanded,
    Worn,
}
// Weapon API
pub struct Weapon(Handedness);

impl Weapon {
    pub fn new(name: String, tags: HashSet<Tag>) -> Result<Weapon> {
        let mut two_handed = None::<bool>;
        let mut weight_class = None::<WeightClass>;
        let mut damage_type = None::<DamageType>;
        let mut archery = None::<RangeBand>;
        let mut thrown = None::<RangeBand>;
        let mut brawl = false;
        let mut melee = false;
        let mut martial_arts_styles = HashSet::<String>::new();
        let mut special_property = None::<String>;
        let mut other_tags = HashSet::<OtherTag>::new();

        for tag in tags {
            match tag {
                Tag::Archery(range) => {
                    if let Some(old_range) = archery {
                        archery = Some(old_range.max(range));
                    } else {
                        archery = Some(range);
                    }
                }
                Tag::Artifact => {
                    other_tags.insert(OtherTag::Artifact);
                }
                Tag::Balanced => {
                    other_tags.insert(OtherTag::Balanced);
                }
                Tag::Bashing => {
                    if let Some(other_type) = damage_type {
                        if other_type != DamageType::Bashing {
                            return Err(eyre!("weapons must be exactly one of Bashing or Lethal"));
                        }
                    } else {
                        damage_type = Some(DamageType::Bashing);
                    }
                }
                Tag::Brawl => {
                    brawl = true;
                }
                Tag::Chopping => {
                    other_tags.insert(OtherTag::Chopping);
                }
                Tag::Concealable => {
                    other_tags.insert(OtherTag::Concealable);
                }
                Tag::Crossbow => {
                    other_tags.insert(OtherTag::Crossbow);
                }
                Tag::Cutting => {
                    other_tags.insert(OtherTag::Cutting);
                }
                Tag::Disarming => {
                    other_tags.insert(OtherTag::Disarming);
                }
                Tag::Exceptional => {
                    other_tags.insert(OtherTag::Exceptional);
                }
                Tag::Flame => {
                    other_tags.insert(OtherTag::Flame);
                }
                Tag::Flexible => {
                    other_tags.insert(OtherTag::Flexible);
                }
                Tag::Grappling => {
                    other_tags.insert(OtherTag::Grappling);
                }
                Tag::Heavy => {
                    if let Some(other_class) = weight_class {
                        if other_class != WeightClass::Heavy {
                            return Err(eyre!(
                                "weapons must be exactly one of Light, Medium, or Heavy"
                            ));
                        }
                    } else {
                        weight_class = Some(WeightClass::Heavy);
                    }
                }
                Tag::Improvised => {
                    other_tags.insert(OtherTag::Improvised);
                }
                Tag::Lethal => {
                    if let Some(other_type) = damage_type {
                        if other_type != DamageType::Lethal {
                            return Err(eyre!("weapons must be exactly one of Bashing or Lethal"));
                        }
                    } else {
                        damage_type = Some(DamageType::Lethal);
                    }
                }
                Tag::Light => {
                    if let Some(other_class) = weight_class {
                        if other_class != WeightClass::Light {
                            return Err(eyre!(
                                "weapons must be exactly one of Light, Medium, or Heavy"
                            ));
                        } else {
                            weight_class = Some(WeightClass::Light);
                        }
                    }
                }
                Tag::MartialArts(style) => {
                    martial_arts_styles.insert(style);
                }
                Tag::Medium => {
                    if let Some(other_class) = weight_class {
                        if other_class != WeightClass::Medium {
                            return Err(eyre!(
                                "weapons must be exactly one of Light, Medium, or Heavy"
                            ));
                        } else {
                            weight_class = Some(WeightClass::Medium);
                        }
                    }
                }
                Tag::Melee => {
                    melee = true;
                }
                Tag::Mounted => {
                    other_tags.insert(OtherTag::Mounted);
                }
                Tag::OneHanded => {
                    if let Some(two) = two_handed {
                        if two {
                            return Err(eyre!(
                                "weapons must be exactly one of OneHanded or TwoHanded"
                            ));
                        }
                    } else {
                        two_handed = Some(false);
                    }
                }
                Tag::Natural => {
                    other_tags.insert(OtherTag::Natural);
                }
                Tag::Piercing => {
                    other_tags.insert(OtherTag::Piercing);
                }
                Tag::Poisonable => {
                    other_tags.insert(OtherTag::Poisonable);
                }
                Tag::Powerful => {
                    other_tags.insert(OtherTag::Powerful);
                }
                Tag::Reaching => {
                    other_tags.insert(OtherTag::Reaching);
                }
                Tag::Shield => {
                    other_tags.insert(OtherTag::Shield);
                }
                Tag::Slow => {
                    other_tags.insert(OtherTag::Slow);
                }
                Tag::Smashing => {
                    other_tags.insert(OtherTag::Smashing);
                }
                Tag::Special(property) => {
                    if let Some(other_property) = special_property {
                        return Err(eyre!("weapons can have no more than one Special tag"));
                    } else {
                        special_property = Some(property);
                    }
                }
                Tag::Subtle => {
                    other_tags.insert(OtherTag::Subtle);
                }
                Tag::Thrown(range) => {
                    if let Some(old_range) = thrown {
                        thrown = Some(old_range.max(range));
                    } else {
                        thrown = Some(range);
                    }
                }
                Tag::TwoHanded => {
                    if let Some(two) = two_handed {
                        if !two {
                            return Err(eyre!(
                                "weapons must be exactly one of OneHanded or TwoHanded"
                            ));
                        }
                    } else {
                        two_handed = Some(true);
                    }
                }
                Tag::Worn => {
                    other_tags.insert(OtherTag::Worn);
                }
            }
        }

        if let None = two_handed {
            return Err(eyre!(
                "weapons must be exactly one of OneHanded or TwoHanded"
            ));
        }

        if let None = weight_class {
            return Err(eyre!(
                "weapons must be exactly one of Light, Medium, or Heavy"
            ));
        }

        if let None = damage_type {
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

        let details = WeaponDetails::new(
            name,
            weight_class.unwrap(),
            damage_type.unwrap(),
            main_attack_method,
            martial_arts_styles,
            special_property,
            other_tags,
        );
        if two_handed.unwrap() {
            Ok(Weapon(Handedness::TwoHanded(details)))
        } else {
            Ok(Weapon(Handedness::OneHanded(details)))
        }
    }

    fn details(&self) -> &WeaponDetails {
        match self.0 {
            Handedness::OneHanded(inner) | Handedness::TwoHanded(inner) => &inner,
        }
    }

    pub fn name(&self) -> &str {
        self.details().name.as_str()
    }

    pub fn accuracy(&self, range: RangeBand) -> Option<i8> {
        let base_accuracy: i8 = match (range, self.details().main_attack_method, self.details().weight_class) {
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

        let exceptional_bonus = i8::from(self.details().other_tags.contains(&OtherTag::Artifact) || self.details().other_tags.contains(&OtherTag::Exceptional));
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
        todo!()
    }

    pub fn defense(&self) -> i8 {
        todo!()
    }

    pub fn attunement(&self) -> u8 {
        todo!()
    }

    pub fn tags(&self) -> HashSet<Tag> {
        let mut output = HashSet::<Tag>::new();

        match self.0 {
            Handedness::OneHanded(_) => {
                output.insert(Tag::OneHanded);
            }
            Handedness::TwoHanded(_) => {
                output.insert(Tag::TwoHanded);
            }
        }

        let details = self.details();
        match details.weight_class {
            WeightClass::Light => {
                output.insert(Tag::Light);
            }
            WeightClass::Medium => {
                output.insert(Tag::Medium);
            }
            WeightClass::Heavy => {
                output.insert(Tag::Heavy);
            }
        }
        match details.damage_type {
            DamageType::Bashing => {
                output.insert(Tag::Bashing);
            }
            DamageType::Lethal => {
                output.insert(Tag::Lethal);
            }
        }
        match details.main_attack_method {
            MainAttackMethod::Archery(range) => {
                output.insert(Tag::Archery(range));
            }
            MainAttackMethod::Brawl => {
                output.insert(Tag::Brawl);
            }
            MainAttackMethod::Melee => {
                output.insert(Tag::Melee);
            }
            MainAttackMethod::MeleeAndThrown(range) => {
                output.insert(Tag::Melee);
                output.insert(Tag::Thrown(range));
            }
            MainAttackMethod::MartialArtsOnly => {}
            MainAttackMethod::ThrownOnly(range) => {
                output.insert(Tag::Thrown(range));
            }
        }

        for style in details.martial_arts_styles {
            output.insert(Tag::MartialArts(style));
        }

        if let Some(property) = details.special_property {
            output.insert(Tag::Special(property));
        }

        for other_tag in details.other_tags {
            let tag = match other_tag {
                OtherTag::Artifact => Tag::Artifact,
                OtherTag::Balanced => Tag::Balanced,
                OtherTag::Chopping => Tag::Chopping,
                OtherTag::Concealable => Tag::Concealable,
                OtherTag::Crossbow => Tag::Crossbow,
                OtherTag::Cutting => Tag::Cutting,
                OtherTag::Disarming => Tag::Disarming,
                OtherTag::Exceptional => Tag::Exceptional,
                OtherTag::Flame => Tag::Flame,
                OtherTag::Flexible => Tag::Flexible,
                OtherTag::Grappling => Tag::Grappling,
                OtherTag::Improvised => Tag::Improvised,
                OtherTag::Mounted => Tag::Mounted,
                OtherTag::Natural => Tag::Natural,
                OtherTag::Piercing => Tag::Piercing,
                OtherTag::Poisonable => Tag::Poisonable,
                OtherTag::Powerful => Tag::Powerful,
                OtherTag::Reaching => Tag::Reaching,
                OtherTag::Shield => Tag::Shield,
                OtherTag::Slow => Tag::Slow,
                OtherTag::Smashing => Tag::Smashing,
                OtherTag::Subtle => Tag::Subtle,
                OtherTag::Worn => Tag::Worn,
            };
            output.insert(tag);
        }

        output
    }
}

struct WeaponDetails {
    name: String,
    weight_class: WeightClass,
    damage_type: DamageType,
    main_attack_method: MainAttackMethod,
    martial_arts_styles: HashSet<String>,
    special_property: Option<String>,
    other_tags: HashSet<OtherTag>,
}

impl WeaponDetails {
    fn new(
        name: String,
        weight_class: WeightClass,
        damage_type: DamageType,
        main_attack_method: MainAttackMethod,
        martial_arts_styles: HashSet<String>,
        special_property: Option<String>,
        other_tags: HashSet<OtherTag>,
    ) -> Self {
        Self {
            name,
            weight_class,
            damage_type,
            main_attack_method,
            martial_arts_styles,
            special_property,
            other_tags,
        }
    }
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
    Subtle,
    Worn,
}

enum Handedness {
    OneHanded(WeaponDetails),
    TwoHanded(WeaponDetails),
}
