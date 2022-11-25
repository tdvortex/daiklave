use crate::range_bands::RangeBand;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Quality {
    Improvised,
    Mundane,
    Exceptional,
    Artifact,
}

#[derive(Debug)]
enum DamageType {
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
enum Tag {
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
enum AttackMethod {
    Brawl,
    Melee,
    Thrown(RangeBand),
    Archery(RangeBand),
    MartialArts(String),
    MartialArtsThrown(String, RangeBand), // ex Silver-Voiced Nightengale Style
    MartialArtsArchery(String, RangeBand), // ex Righteous Devil Style
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
    fn accuracy(&self, attack_method: &AttackMethod) -> Result<i8, String>;
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

    fn accuracy(&self, attack_method: &AttackMethod) -> Result<i8, String> {
        if !self.attack_methods.contains(attack_method) {
            return Err(format!("Invalid attack method: {:?}", attack_method));
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

    fn accuracy(&self, attack_method: &AttackMethod) -> Result<i8, String> {
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

    fn accuracy(&self, attack_method: &AttackMethod) -> Result<i8, String> {
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
pub struct ZeroHandedWeapon(WeaponDetails);

impl Weapon for ZeroHandedWeapon {
    fn name(&self) -> &str {
        self.0.name()
    }

    fn accuracy(&self, attack_method: &AttackMethod) -> Result<i8, String> {
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
enum EquippedState {
    HandsFree,
    MainHandOnly(OneHandedWeapon),
    TwoDifferent(OneHandedWeapon, OneHandedWeapon),
    Paired(OneHandedWeapon),
    TwoHanded(TwoHandedWeapon),
}

impl Default for EquippedState {
    fn default() -> Self {
        Self::HandsFree
    }
}

#[derive(Debug)]
pub struct Weapons {
    equipped: EquippedState,
    unequipped_one_handed: Vec<OneHandedWeapon>,
    unequipped_two_handed: Vec<OneHandedWeapon>,
    zero_handed: Vec<ZeroHandedWeapon>,
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
                alternate_attack_methods: HashSet::new(),
            },
            tags: [Tag::Grappling, Tag::Natural].into(),
        });

        let equipped = EquippedState::MainHandOnly(unarmed);

        Self {
            equipped,
            unequipped_one_handed: Vec::new(),
            unequipped_two_handed: Vec::new(),
            zero_handed: Vec::new(),
        }
    }
}
