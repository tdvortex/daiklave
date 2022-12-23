use super::{ExperiencePoints, Willpower};
use crate::{
    abilities::{AbilityNameNoSubskill, AbilityNameVanilla},
    anima::{AnimaEffect, AnimaLevel},
    armor::{ArmorItem, ArtifactArmorItem},
    artifact::{Hearthstone, Warstrider, Wonder},
    attributes::AttributeName,
    campaign::Campaign,
    charms::{MartialArtsCharm, SolarCharm, Spell},
    essence::MotePool,
    health::WoundPenalty,
    id::{
        ArmorItemId, ArtifactArmorItemId, ArtifactWeaponId, FlawId, HearthstoneId, IntimacyId,
        MartialArtsCharmId, MartialArtsStyleId, MeritId, SpellId, WarstriderId, WonderId, WeaponId, SolarCharmId,
    },
    intimacies::Intimacy,
    martial_arts::MartialArtsStyle,
    merits::{Flaw, MeritTemplate},
    player::Player,
    solar::SolarCaste,
    sorcery::{ShapingRitual, SorceryCircle}, weapons::{Weapon, ArtifactWeapon, EquipHand},
};

pub enum CharacterMutation {
    // Abilities
    SetAbility(AbilityNameVanilla, u8),
    AddSpecialty(AbilityNameVanilla, String),
    RemoveSpecialty(AbilityNameVanilla, String),
    // Anima
    SetAnimaLevel(AnimaLevel),
    // Armor
    AddNonArtifactArmor(ArmorItem),
    AddArtifactArmor(ArtifactArmorItem),
    RemoveNonArtifactArmor(ArmorItemId),
    RemoveArtifactArmor(ArtifactArmorItemId),
    EquipNonArtifactArmor(ArmorItemId),
    EquipArtifactArmor(ArtifactArmorItemId),
    UnequipArmor,
    AttuneArtifactArmor(ArtifactArmorItemId, u8, u8),
    UnattuneArtifactArmor(ArtifactArmorItemId),
    // Attributes
    SetAttribute(AttributeName, u8),
    // Campaign
    SetCampaign(Campaign),
    RemoveCampaign,
    // Character
    SetName(String),
    SetConcept(String),
    RemoveConcept,
    SetWillpower(Willpower),
    SetExperience(ExperiencePoints),
    // Craft
    SetCraftAbility(String, u8),
    AddCraftSpecialty(String, String),    // Focus, Specialty
    RemoveCraftSpecialty(String, String), // Focus, Specialty
    // Essence
    SetEssenceRating(u8),
    SetPeripheralMotes(MotePool),
    SetPersonalMotes(MotePool),
    // Health
    SetDamage(u8, u8, u8),
    SetBoxes(Vec<WoundPenalty>),
    // Hearthstones
    AddHearthstone(Hearthstone),
    RemoveHearthstone(HearthstoneId),
    SlotHeartstoneIntoArmor(HearthstoneId, ArtifactArmorItemId),
    SlotHeartstoneIntoWarstrider(HearthstoneId, WarstriderId),
    SlotHeartstoneIntoWeapon(HearthstoneId, ArtifactWeaponId),
    SlotHearthstoneIntoWonder(HearthstoneId, WonderId),
    UnslotHearthstone(HearthstoneId),
    // Intimacies
    SetIntimacy(Intimacy),
    RemoveIntimacy(IntimacyId),
    // Limit
    SetLimitTrack(u8),
    SetLimitTrigger(String),
    // Martial Arts
    AddMartialArtsStyle(MartialArtsStyle),
    SetMartialArtsDots(MartialArtsStyleId, u8),
    AddMartialArtsSpecialty(MartialArtsStyleId, String),
    RemoveMartialArtsSpecialty(MartialArtsStyleId, String),
    AddMartialArtsCharm(MartialArtsCharm),
    RemoveMartialArtsCharm(MartialArtsCharmId),
    // Merits
    AddDetailedMerit(MeritTemplate, String),
    AddNonDetailedMerit(MeritTemplate),
    RemoveMerit(MeritId),
    AddFlaw(Flaw),
    RemoveFlaw(FlawId),
    // Mortal
    ConvertToMortal,
    // Player
    SetPlayer(Player),
    // Solar
    SetSolarCaste(SolarCaste, [AnimaEffect; 5], [AbilityNameNoSubskill; 5]),
    AddSolarCharm(SolarCharm),
    RemoveSolarCharm(SolarCharmId),
    SetSolarExperience(ExperiencePoints),
    // Sorcery
    AddSorceryCircle(SorceryCircle, ShapingRitual, Spell),
    RemoveSorceryCircle(SorceryCircle),
    AddSpell(Spell),
    RemoveSpell(SpellId),
    // Warstriders
    AddWarstrider(Warstrider),
    EquipWarstrider(WarstriderId),
    UnequipWarstrider,
    AttuneWarstrider(WarstriderId, u8, u8),
    UnattuneWarstrider,
    // Weapons
    AddNonArtifactWeapon(Weapon),
    AddArtifactWeapon(ArtifactWeapon),
    RemoveNonArtifactWeapon(WeaponId),
    RemoveArtifactWeapon(ArtifactWeaponId),
    EquipNonArtifactWeapon(WeaponId, EquipHand),
    EquipArtifactWeapon(ArtifactWeaponId, EquipHand),
    AttuneArtifactWeapon(ArtifactWeaponId, u8, u8),
    UnattuneArtifactWeapon(ArtifactWeaponId),
    // Wonders
    AddWonder(Wonder),
    RemoveWonder(WonderId),
    AttuneWonder(WonderId, u8, u8),
    UnattuneWonder(WonderId),
}
