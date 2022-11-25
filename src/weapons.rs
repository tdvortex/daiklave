use crate::range_bands::RangeBand;
use std::collections::HashSet;

#[derive(PartialEq, Eq)]
enum Quality {
    Improvised,
    Mundane,
    Exceptional,
    Artifact,
}

enum DamageType {
    Bashing,
    Lethal,
}

enum Handedness {
    HandsFree,
    OneHanded,
    TwoHanded,
}

enum WeightClass {
    Light,
    Medium,
    Heavy,
}

#[derive(PartialEq, Eq, Hash)]
enum SpecialTag {
    Balanced,
    Chopping,
    Concealable,
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

struct Weapon {
    name: String,
    quality: Quality,
    weight_class: WeightClass,
    damage_type: DamageType,
    attack_methods: AttackMethods,
    handedness: Handedness,
    special_tags: HashSet<SpecialTag>,
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

struct AttackMethods {
    default_attack_method: AttackMethod,
    alternate_attack_methods: HashSet<AttackMethod>,
}

impl AttackMethods {
    fn contains(&self, attack_method: &AttackMethod) -> bool {
        self.default_attack_method == *attack_method || self.alternate_attack_methods.contains(attack_method)
    }
}

impl Weapon {
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
            (_, AttackMethod::Thrown(RangeBand::Close)) | (_, AttackMethod::MartialArtsThrown(_, RangeBand::Close)) => 4,
            (_, AttackMethod::Thrown(RangeBand::Short)) | (_, AttackMethod::MartialArtsThrown(_, RangeBand::Short)) => 3,
            (_, AttackMethod::Thrown(RangeBand::Medium)) | (_, AttackMethod::MartialArtsThrown(_, RangeBand::Medium)) => 2,
            (_, AttackMethod::Thrown(RangeBand::Long)) | (_, AttackMethod::MartialArtsThrown(_, RangeBand::Long)) => -1,
            (_, AttackMethod::Thrown(RangeBand::Extreme)) | (_, AttackMethod::MartialArtsThrown(_, RangeBand::Extreme)) => -3,
            (_, AttackMethod::Archery(RangeBand::Close)) | (_, AttackMethod::MartialArtsArchery(_, RangeBand::Close)) => -2,
            (_, AttackMethod::Archery(RangeBand::Short)) | (_, AttackMethod::MartialArtsArchery(_, RangeBand::Short)) => 4,
            (_, AttackMethod::Archery(RangeBand::Medium)) | (_, AttackMethod::MartialArtsArchery(_, RangeBand::Medium)) => 2,
            (_, AttackMethod::Archery(RangeBand::Long)) | (_, AttackMethod::MartialArtsArchery(_, RangeBand::Long)) => 0,
            (_, AttackMethod::Archery(RangeBand::Extreme)) | (_, AttackMethod::MartialArtsArchery(_, RangeBand::Extreme)) => -2,
        };

        let exceptional_bonus = i8::from(self.quality == Quality::Exceptional || self.quality == Quality::Artifact);
        let flame_bonus = match (self.special_tags.contains(&SpecialTag::Flame), attack_method) {
            (true, AttackMethod::Archery(RangeBand::Close)) | (true, AttackMethod::MartialArtsArchery(_, RangeBand::Close)) => 2,
            (_, _) => 0,
        };

        Ok(base_accuracy + exceptional_bonus + flame_bonus)
    }
}