use super::{ExperiencePoints, Willpower};
use eyre::Result;
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
    sorcery::{ShapingRitual, SorceryCircle}, weapons::{Weapon, ArtifactWeapon, EquipHand}, Character,
};

pub enum CharacterMutation {
    // Load
    Load(Character),
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

impl Character {
    pub fn apply_mutation(&mut self, mutation: &CharacterMutation) -> Result<&mut Self> {
        match mutation {
            CharacterMutation::Load(character) => {*self = character.clone();}
            CharacterMutation::SetAbility(ability_name_vanilla, dots) => {
                self.abilities.set_dots(*ability_name_vanilla, *dots);
            }
            CharacterMutation::AddSpecialty(ability_name_vanilla, specialty) => {
                self.abilities.add_specialty(*ability_name_vanilla, specialty.clone())?;
            }
            CharacterMutation::RemoveSpecialty(ability_name_vanilla, specialty) => {
                self.abilities.remove_specialty(*ability_name_vanilla, specialty.clone())?;
            }
            CharacterMutation::SetAnimaLevel(anima_level) => {
                self.exalt_type.set_anima_level(*anima_level)?; 
            }
            CharacterMutation::AddNonArtifactArmor(_) => todo!(),
            CharacterMutation::AddArtifactArmor(_) => todo!(),
            CharacterMutation::RemoveNonArtifactArmor(_) => todo!(),
            CharacterMutation::RemoveArtifactArmor(_) => todo!(),
            CharacterMutation::EquipNonArtifactArmor(_) => todo!(),
            CharacterMutation::EquipArtifactArmor(_) => todo!(),
            CharacterMutation::UnequipArmor => todo!(),
            CharacterMutation::AttuneArtifactArmor(_, _, _) => todo!(),
            CharacterMutation::UnattuneArtifactArmor(_) => todo!(),
            CharacterMutation::SetAttribute(attribute_name, value) => {
                self.attributes.set(*attribute_name, *value)?;
            }
            CharacterMutation::SetCampaign(campaign) => {
                self.campaign = Some(campaign.clone());
            }
            CharacterMutation::RemoveCampaign => {
                self.campaign = None;
            }
            CharacterMutation::SetName(name) => {
                self.name = name.clone();
            }
            CharacterMutation::SetConcept(concept) => {
                self.concept = Some(concept.clone());
            }
            CharacterMutation::RemoveConcept => {
                self.concept = None;
            }
            CharacterMutation::SetWillpower(willpower) => {
                self.willpower = *willpower;
            }
            CharacterMutation::SetExperience(experience) => {
                self.experience = *experience;
            }
            CharacterMutation::SetCraftAbility(focus, dots) => {
                self.craft_abilities.set_dots(focus.as_str(), *dots);
            }
            CharacterMutation::AddCraftSpecialty(focus, specialty) => {
                self.craft_abilities.add_specialty(focus.as_str(), specialty.clone())?;
            }
            CharacterMutation::RemoveCraftSpecialty(focus, specialty) => {
                self.craft_abilities.remove_specialty(focus.as_str(), specialty.as_str())?;
            }
            CharacterMutation::SetEssenceRating(essence) => {
                self.exalt_type.set_essence_rating(*essence)?;
            }
            CharacterMutation::SetPeripheralMotes(peripheral) => {
                self.exalt_type.set_peripheral_motes(*peripheral)?;
            }
            CharacterMutation::SetPersonalMotes(personal) => {
                self.exalt_type.set_personal_motes(*personal)?;
            }
            CharacterMutation::SetDamage(bashing, lethal, aggravated) => {
                self.health.set_damage(*bashing, *lethal, *aggravated);
            }
            CharacterMutation::SetBoxes(_) => todo!(),
            CharacterMutation::AddHearthstone(_) => todo!(),
            CharacterMutation::RemoveHearthstone(_) => todo!(),
            CharacterMutation::SlotHeartstoneIntoArmor(_, _) => todo!(),
            CharacterMutation::SlotHeartstoneIntoWarstrider(_, _) => todo!(),
            CharacterMutation::SlotHeartstoneIntoWeapon(_, _) => todo!(),
            CharacterMutation::SlotHearthstoneIntoWonder(_, _) => todo!(),
            CharacterMutation::UnslotHearthstone(_) => todo!(),
            CharacterMutation::SetIntimacy(_) => todo!(),
            CharacterMutation::RemoveIntimacy(_) => todo!(),
            CharacterMutation::SetLimitTrack(_) => todo!(),
            CharacterMutation::SetLimitTrigger(_) => todo!(),
            CharacterMutation::AddMartialArtsStyle(_) => todo!(),
            CharacterMutation::SetMartialArtsDots(_, _) => todo!(),
            CharacterMutation::AddMartialArtsSpecialty(_, _) => todo!(),
            CharacterMutation::RemoveMartialArtsSpecialty(_, _) => todo!(),
            CharacterMutation::AddMartialArtsCharm(_) => todo!(),
            CharacterMutation::RemoveMartialArtsCharm(_) => todo!(),
            CharacterMutation::AddDetailedMerit(_, _) => todo!(),
            CharacterMutation::AddNonDetailedMerit(_) => todo!(),
            CharacterMutation::RemoveMerit(_) => todo!(),
            CharacterMutation::AddFlaw(_) => todo!(),
            CharacterMutation::RemoveFlaw(_) => todo!(),
            CharacterMutation::ConvertToMortal => todo!(),
            CharacterMutation::SetPlayer(player) => {
                self.player = player.clone();
            }
            CharacterMutation::SetSolarCaste(_, _, _) => todo!(),
            CharacterMutation::AddSolarCharm(_) => todo!(),
            CharacterMutation::RemoveSolarCharm(_) => todo!(),
            CharacterMutation::SetSolarExperience(_) => todo!(),
            CharacterMutation::AddSorceryCircle(_, _, _) => todo!(),
            CharacterMutation::RemoveSorceryCircle(_) => todo!(),
            CharacterMutation::AddSpell(_) => todo!(),
            CharacterMutation::RemoveSpell(_) => todo!(),
            CharacterMutation::AddWarstrider(_) => todo!(),
            CharacterMutation::EquipWarstrider(_) => todo!(),
            CharacterMutation::UnequipWarstrider => todo!(),
            CharacterMutation::AttuneWarstrider(_, _, _) => todo!(),
            CharacterMutation::UnattuneWarstrider => todo!(),
            CharacterMutation::AddNonArtifactWeapon(_) => todo!(),
            CharacterMutation::AddArtifactWeapon(_) => todo!(),
            CharacterMutation::RemoveNonArtifactWeapon(_) => todo!(),
            CharacterMutation::RemoveArtifactWeapon(_) => todo!(),
            CharacterMutation::EquipNonArtifactWeapon(_, _) => todo!(),
            CharacterMutation::EquipArtifactWeapon(_, _) => todo!(),
            CharacterMutation::AttuneArtifactWeapon(_, _, _) => todo!(),
            CharacterMutation::UnattuneArtifactWeapon(_) => todo!(),
            CharacterMutation::AddWonder(_) => todo!(),
            CharacterMutation::RemoveWonder(_) => todo!(),
            CharacterMutation::AttuneWonder(_, _, _) => todo!(),
            CharacterMutation::UnattuneWonder(_) => todo!(),
        }
        Ok(self)
    }
}