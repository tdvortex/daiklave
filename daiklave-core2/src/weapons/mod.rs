use std::collections::{HashMap, HashSet};

use crate::{unique_id::UniqueId, book_reference::BookReference};

enum WeaponId {
    BaseWeaponId(BaseWeaponId),
    ArtifactWeaponId(ArtifactWeaponId),
}

mod weapon_id;

pub use weapon_id::BaseWeaponId;
struct ArtifactWeaponId(UniqueId);


struct MortalWeapons<'source> {
    equipped: MortalEquippedWeapons<'source>,
    unequipped: MortalUnequippedWeapons<'source>,
}

struct ExaltWeapons<'source> {
    equipped: ExaltEquippedWeapons<'source>,
    unequipped: ExaltUnequippedWeapons<'source>,
}

struct MortalEquippedWeapons<'source> {
    handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeaponNoAttunement<'source>>,
    hands: MortalHands<'source>,
}

struct MortalUnequippedWeapons<'source> {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeapon<'source>>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponNoAttunement<'source>>,
}

struct ExaltEquippedWeapons<'source> {
    handless_mundane: HashMap<BaseWeaponId, HandlessMundaneWeapon<'source>>,
    handless_artifact: HashMap<ArtifactWeaponId, HandlessArtifactWeapon<'source>>,
    hands: ExaltHands<'source>,
}

struct ExaltUnequippedWeapons<'source> {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeapon<'source>>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeapon<'source>>,
}

enum NonnaturalMundaneWeapon<'source> {
    Worn(WornMundaneWeapon<'source>),
    OneHanded(OneHandedMundaneWeapon<'source>),
    TwoHanded(TwoHandedMundaneWeapon<'source>),
}

enum NonnaturalArtifactWeaponNoAttunement<'source> {
    Worn(WornArtifactWeapon<'source>),
    OneHanded(OneHandedArtifactWeapon<'source>),
    TwoHanded(TwoHandedArtifactWeapon<'source>),
}

enum NonnaturalArtifactWeapon<'source> {
    Worn(WornArtifactWeapon<'source>, Option<u8>),
    OneHanded(OneHandedArtifactWeapon<'source>, Option<u8>),
    TwoHanded(TwoHandedArtifactWeapon<'source>, Option<u8>),
}

enum HandlessArtifactWeaponNoAttunement<'source> {
    Natural(NaturalArtifactWeapon<'source>),
    Worn(WornArtifactWeapon<'source>),
}

enum HandlessArtifactWeapon<'source> {
    Natural(NaturalArtifactWeapon<'source>, Option<u8>),
    Worn(WornArtifactWeapon<'source>, Option<u8>),

}




enum HandlessMundaneWeapon<'source> {
    Natural(NaturalMundaneWeapon<'source>),
    Worn(WornMundaneWeapon<'source>),
}



struct BaseWeapon<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    weight_class: WeaponWeightClass,
    range_bands: WeaponRange,
    primary_ability: WeaponAbility,
    damage_type: WeaponDamageType,
    tags: HashSet<WeaponTag>,
}

struct MundaneWeaponCore<'source>(BaseWeapon<'source>);
struct BaseArtifactWeapon<'source>(BaseWeapon<'source>);

struct NamedArtifactWeapon<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    merit_dots: u8,
    base_weapon_id: BaseWeaponId,
    base_weapon: BaseArtifactWeapon<'source>,
    lore: Option<String>,
    powers: Option<String>,
    hearthstone_slots: Vec<Option<OwnedHearthstone<'source>>>,
}

struct OwnedHearthstone<'source> {
    hearthstone: Hearthstone<'source>,
    manse: Option<&'source str>,
}

struct Hearthstone<'source> {
    name: &'source str,
    book_reference: Option<BookReference>,
    geomancy_level: GeomancyLevel,
    lore: Option<String>,
    powers: Option<String>,
}


enum MortalHands<'source> {
    Empty,
    MainHand(EquippedOneHandedWeaponNoAttunement<'source>),
    OffHand(EquippedOneHandedWeaponNoAttunement<'source>),
    Both([EquippedOneHandedWeaponNoAttunement<'source>; 2]),
    TwoHanded(EquippedTwoHandedWeaponNoAttunement<'source>),
}

enum ExaltHands<'source> {
    Empty,
    MainHand(EquippedOneHandedWeapon<'source>),
    OffHand(EquippedOneHandedWeapon<'source>),
    Both([EquippedOneHandedWeapon<'source>; 2]),
    TwoHanded(EquippedTwoHandedWeapon<'source>),
}

enum EquippedOneHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, OneHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeapon<'source>),
}

enum EquippedOneHandedWeapon<'source> {
    Mundane(BaseWeaponId, OneHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, OneHandedArtifactWeapon<'source>, Option<u8>),
}

enum EquippedTwoHandedWeaponNoAttunement<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon<'source>),
}

enum EquippedTwoHandedWeapon<'source> {
    Mundane(BaseWeaponId, TwoHandedMundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, TwoHandedArtifactWeapon<'source>, Option<u8>),
}

struct NaturalMundaneWeapon<'source>(MundaneWeaponCore<'source>);
struct WornMundaneWeapon<'source>(MundaneWeaponCore<'source>);

struct OneHandedMundaneWeapon<'source>(MundaneWeaponCore<'source>);
struct TwoHandedMundaneWeapon<'source>(MundaneWeaponCore<'source>);

enum MundaneWeapon<'source> {
    Natural(NaturalMundaneWeapon<'source>),
    Worn(WornMundaneWeapon<'source>),
    OneHanded(OneHandedMundaneWeapon<'source>),
    TwoHanded(TwoHandedMundaneWeapon<'source>),
}

struct NaturalArtifactWeapon<'source>(NamedArtifactWeapon<'source>);
struct WornArtifactWeapon<'source>(NamedArtifactWeapon<'source>);
struct OneHandedArtifactWeapon<'source>(NamedArtifactWeapon<'source>);
struct TwoHandedArtifactWeapon<'source>(NamedArtifactWeapon<'source>);

enum WeaponWeightClass {
    Light,
    Medium,
    Heavy,
}

enum WeaponRange {
    ContactOnly,
    Throwable(RangeBand),
    Archery(RangeBand),
}

enum WeaponAbility {
    Brawl,
    Melee,
    MeleeOrThrown,
    ThrownOnly,
    Archery,
    MartialArtsOnly,
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

enum RangeBand {
    Close,
    Short,
    Medium,
    Long,
    Extreme
}

enum GeomancyLevel {
    Standard,
    Greater,
}
