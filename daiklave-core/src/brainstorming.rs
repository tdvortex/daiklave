use crate::{id::{Id, EvocationId}, data_source::DataSource, weapons::Weapon, artifact::MagicMaterial, armor::{ArmorItem}, charms::{CharmActionType, CharmTraits}, health::Health};

struct Evocation {
    id: EvocationId,
    action_type: CharmActionType,
    essence_requirement: u8,
    traits: CharmTraits,
    prerequisite_evocations: Vec<EvocationId>,
}

enum HearthstoneCategory {
    Air,
    Earth,
    Fire,
    Water,
    Wood,
    Solar,
    Sidereal,
    Lunar,
    Abyssal,
}

enum HearthstoneKeyword {
    Linked,
    Steady,
    Dependent,
    ManseBorn,
    WildBorn,
}

struct Hearthstone {
    id: Id,
    data_source: DataSource,
    geomancy_level: GeomancyLevel,
    category: HearthstoneCategory,
    keywords: Vec<HearthstoneKeyword>,
    magic_item_traits: MagicItemTraits,
}

struct MagicItemTraits {
    name: String, 
    lore: Option<String>,
    powers: Option<String>,
    available_evocations: Vec<Evocation>,
}

struct ArtifactWeapon {
    id: Id,
    data_source: DataSource,
    merit_dots: u8,
    magic_item_traits: MagicItemTraits,
    base_weapon: Weapon,
    magic_material: MagicMaterial,
    hearthstone_slots: Vec<Option<Id>>,
}

struct NonArtifactWeapon(Weapon);

enum OneHandedWeapon {
    Artifact(ArtifactWeapon),
    NonArtifact(NonArtifactWeapon),
}

enum TwoHandedWeapon {
    Artifact(ArtifactWeapon),
    NonArtifact(NonArtifactWeapon),
}

enum EquipState {
    None,
    MainHand(OneHandedWeapon),
    OffHand(OneHandedWeapon),
    Paired(OneHandedWeapon),
    TwoDifferent(OneHandedWeapon, OneHandedWeapon),

}

struct Weapons {
    equipped: EquipState,
    unequipped_artifacts: Vec<ArtifactWeapon>,
    unequipped_nonartifacts: Vec<NonArtifactWeapon>,
}

struct ArtifactArmor {
    id: Id,
    data_source: DataSource,
    merit_dots: u8,
    magic_item_traits: MagicItemTraits,
    base_armor: ArmorItem,
    magic_material: MagicMaterial,
    hearthstone_slots: Vec<Option<Id>>,
}

struct NonArtifactArmor(ArmorItem);

struct Armor {
    worn: Option<ArmorItem>,
    unworn: Vec<ArmorItem>,
}


struct ArtifactWonder {
    id: Id, 
    data_source: DataSource,
    merit_dots: u8,
    magic_item_traits: MagicItemTraits,
    magic_material: Option<MagicMaterial>,
    hearthstone_slots: Vec<Option<Id>>,
}


struct Warstrider {
    id: Id,
    data_source: DataSource,
    merit_dots: u8,
    magic_item_traits: MagicItemTraits,
    attunement: u8,
    soak: u8,
    hardness: u8,
    speed_bonus: u8,
    damage_track: Health,
    hearthstone_slots: Vec<Option<Id>>,
}

enum GeomancyLevel {
    Standard,
    Greater
}

struct Manse {
    manse_detail: String,
    demense_detail: String,
    geomancy_level: GeomancyLevel,
    hearthstone: Hearthstone
}

struct Panoply {
    manses: Option<Vec<Manse>>,
    other_hearthstones: Option<Vec<Hearthstone>>,
    wonders: Option<Vec<ArtifactWonder>>,
    warstriders: Option<Vec<Warstrider>>,
}