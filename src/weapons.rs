use crate::range_bands::RangeBand;
use std::collections::HashSet;

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

#[derive(Hash, PartialEq, Eq)]
enum WeaponSkill {
    Archery,
    Brawl,
    MartialArts(String),
    Melee,
    Thrown,
}

enum Handedness {
    OneHanded,
    TwoHanded,
}

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
    quality: Quality,
    max_range: RangeBand,
    damage_type: DamageType,
    primary_skill: WeaponSkill,
    secondary_skills: HashSet<WeaponSkill>,
    handedness: Handedness,
    special_tags: HashSet<SpecialTag>,
}

impl Weapon {
    fn usable_with(&self, skill: &WeaponSkill) -> bool {
        self.primary_skill == *skill || self.secondary_skills.contains(skill)
    }

    fn in_range(&self, skill: &WeaponSkill, range: &RangeBand) -> bool {
        if !self.usable_with(skill) {
            return false;
        }

        match (skill, range, &self.max_range) {
            (_, RangeBand::Close, _) => true, // All weapons usable at close range
            (WeaponSkill::Brawl, _, _) => false, // Brawl only usable at close range
            (WeaponSkill::Melee, _, _) => false, // Melee only usable at close range
            (_, range, max_range) => max_range >= range, // All other weapons useable up to their max_range
        }
    }
}