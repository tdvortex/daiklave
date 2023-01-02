mod weapon_id;

use std::collections::{HashSet, HashMap};

pub use weapon_id::BaseWeaponId;

use crate::book_reference::BookReference;

use self::weapon_id::ArtifactWeaponId;

enum RangeBand {
    Close,
    Short,
    Medium,
    Long,
    Extreme
}

enum WeaponWeightClass {
    Light,
    Medium,
    Heavy,
}

enum PrimaryAttack {
    Brawl,
    Melee,
    MeleeOrThrown(RangeBand),
    ThrownOnly(RangeBand),
    Archery(RangeBand),
}

enum WeaponDamageType {
    Bashing,
    Lethal,
}

enum WeaponTag {
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
    MartialArts,
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


struct BaseWeapon<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    primary_attack: PrimaryAttack,
    weight_class: WeaponWeightClass,
    damage_type: WeaponDamageType,
    tags: HashSet<WeaponTag>,
}

struct OneHandedNonArtifactWeapon<'source>(BaseWeapon<'source>);

struct TwoHandedNonArtifactWeapon<'source>(BaseWeapon<'source>);

enum NonArtifactWeapon<'source> {
    OneHanded(OneHandedNonArtifactWeapon<'source>),
    TwoHanded(TwoHandedNonArtifactWeapon<'source>),
}

struct OneHandedBaseArtifactWeapon<'source>(BaseWeapon<'source>);

struct TwoHandedBaseArtifactWeapon<'source>(BaseWeapon<'source>);

enum MagicMaterial {
    RedJade,
    BlueJade,
    WhiteJade,
    GreenJade,
    BlackJade,
    Orichalcum,
    Moonsilver,
    Soulsteel,
    Starmetal,
}

struct OneHandedArtifactWeapon<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    merit_dots: u8,
    base_weapon_id: BaseWeaponId,
    base_weapon: OneHandedBaseArtifactWeapon<'source>,
    magic_material: MagicMaterial,
    lore: Option<&'source str>,
    powers: Option<&'source str>,
    hearthstone_slots: Vec<Option<OwnedHearthstone<'source>>>,
}

struct TwoHandedArtifactWeapon<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    merit_dots: u8,
    base_weapon_id: BaseWeaponId,
    base_weapon: TwoHandedBaseArtifactWeapon<'source>,
    magic_material: MagicMaterial,
    lore: Option<&'source str>,
    powers: Option<&'source str>,
    hearthstone_slots: Vec<Option<OwnedHearthstone<'source>>>,
}

enum GeomancyLevel {
    Standard,
    Greater,
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

struct Hearthstone<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    category: HearthstoneCategory,
    keywords: Vec<HearthstoneKeyword>,
    geomancy_level: GeomancyLevel,
    lore: Option<&'source str>,
    powers: Option<&'source str>,
}

struct OwnedHearthstone<'source> {
    hearthstone: Hearthstone<'source>,
    manse: Option<&'source str>,
}

pub struct MortalWeapons<'source> {
    equipped: MortalWeaponsEquipped<'source>,
    unequipped_nonartifacts: Vec<(BaseWeaponId, NonArtifactWeapon<'source>)>,
    unequipped_artifacts: HashMap<ArtifactWeaponId, UnattunedArtifactWeapon<'source>>,
}

pub struct ExaltWeapons<'source> {
    equipped: ExaltWeaponsEquipped<'source>,
    unequipped_nonartifacts: Vec<(BaseWeaponId, NonArtifactWeapon<'source>)>,
    unequipped_artifacts: HashMap<ArtifactWeaponId, ArtifactWeapon<'source>>,
}


enum MortalWeaponsEquipped<'source> {
    None,
    MainHandOnly(OneHandedUnattunedWeapon<'source>),
    OffHandOnly(OneHandedUnattunedWeapon<'source>),
    TwoHanded(TwoHandedUnattunedWeapon<'source>),
    Both(OneHandedUnattunedWeapon<'source>, OneHandedUnattunedWeapon<'source>),
}

enum ExaltWeaponsEquipped<'source> {
    None,
    MainHandOnly(OneHandedWeapon<'source>),
    OffHandOnly(OneHandedWeapon<'source>),
    TwoHanded(TwoHandedUnattunedWeapon<'source>),
    Both(OneHandedWeapon<'source>, OneHandedWeapon<'source>),
}


enum OneHandedUnattunedWeapon<'source> {
    NonArtifact(OneHandedNonArtifactWeapon<'source>),
    Artifact(OneHandedArtifactWeapon<'source>),
}

enum TwoHandedUnattunedWeapon<'source> {
    NonArtifact(TwoHandedNonArtifactWeapon<'source>),
    Artifact(TwoHandedArtifactWeapon<'source>),
}

enum UnattunedArtifactWeapon<'source> {
    OneHanded(OneHandedArtifactWeapon<'source>),
    TwoHanded(TwoHandedArtifactWeapon<'source>),
}

struct AttunedArtifactWeapon<'source> {
    personal_committed: u8,
    weapon: UnattunedArtifactWeapon<'source>,
}

enum ArtifactWeapon<'source> {
    Attuned(AttunedArtifactWeapon<'source>),
    Unattuned(UnattunedArtifactWeapon<'source>),
}

enum OneHandedWeapon<'source> {
    NonArtifact(OneHandedNonArtifactWeapon<'source>),
    UnattunedArtifact(OneHandedArtifactWeapon<'source>),
    AttunedArtifact(u8, OneHandedArtifactWeapon<'source>),
}

enum TwoHandedWeapon<'source> {
    NonArtifact(TwoHandedNonArtifactWeapon<'source>),
    UnattunedArtifact(TwoHandedArtifactWeapon<'source>),
    AttunedArtifact(u8, TwoHandedArtifactWeapon<'source>),
}