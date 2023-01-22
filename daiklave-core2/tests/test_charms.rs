use std::num::NonZeroU8;

use daiklave_core2::{
    abilities::{AbilityName, AbilityNameVanilla},
    book_reference::{Book, BookReference},
    charms::{
        charm::{Charm, CharmId, CharmMutation, evocation::{Evocation, EvokableId, EvocationKeyword, EvocationId}},
        CharmActionType, CharmCostType,
    },
    exaltation::exalt::exalt_type::solar::{
        caste::TwilightAbility,
        charm::{SolarCharm, SolarCharmAbility, SolarCharmId, SolarCharmKeyword},
        Solar,
    },
    unique_id::UniqueId,
    CharacterEventSource, CharacterMutation, weapons::weapon::{Weapon, OptionalWeaponTag, WeaponWeightClass, BaseWeaponId, ArtifactWeaponId}, artifact::{MagicMaterial, Artifact, ArtifactId, wonders::WonderId}, hearthstones::{hearthstone::{Hearthstone, HearthstoneCategory, GeomancyLevel}, HearthstoneId},
};

#[test]
fn test_solar_charms() {
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character().unwrap();
    assert!(character.charms().iter().next().is_none());

    // Mortals cannot add Solar Charms, even if they meet the ability prerequisites
    event_source
        .apply_mutation(CharacterMutation::SetAbilityDots(
            AbilityNameVanilla::Archery,
            5,
        ))
        .unwrap();
    let wise_arrow = SolarCharm::builder("Wise Arrow".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 255))
        .cost(CharmCostType::Motes, NonZeroU8::new(1).unwrap())
        .essence_required(NonZeroU8::new(1).unwrap())
        .ability_required(SolarCharmAbility::Archery, 2)
        .action_type(CharmActionType::Supplemental)
        .keyword(SolarCharmKeyword::Uniform)
        .duration("Instant".to_owned())
        .description("With skill and effor, the Exalt guides her arrow to its mark[...]".to_owned())
        .summary("Shoot around cover".to_owned())
        .build();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(
            SolarCharmId(UniqueId::Placeholder(1)),
            wise_arrow.clone()
        )))
        .is_err());

    // Solars must meet ability requirements
    [
        AbilityNameVanilla::Archery,
        AbilityNameVanilla::Dodge,
        AbilityNameVanilla::Thrown,
        AbilityNameVanilla::War,
        AbilityNameVanilla::Stealth,
    ]
    .into_iter()
    .for_each(|ability_name| {
        event_source
            .apply_mutation(CharacterMutation::SetAbilityDots(ability_name, 1))
            .unwrap();
    });

    let new_solar = Solar::builder()
        .twilight()
        .caste_ability(TwilightAbility::Bureaucracy)
        .caste_ability(TwilightAbility::Integrity)
        .caste_ability(TwilightAbility::Craft)
        .caste_ability(TwilightAbility::Investigation)
        .supernal_ability(TwilightAbility::Lore)
        .favored_ability(AbilityName::Archery)
        .favored_ability(AbilityName::Dodge)
        .favored_ability(AbilityName::War)
        .favored_ability(AbilityName::Thrown)
        .favored_ability(AbilityName::Stealth)
        .limit_trigger("A limit trigger".to_owned())
        .build()
        .unwrap();

    event_source
        .apply_mutation(CharacterMutation::SetSolar(new_solar))
        .unwrap();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(
            SolarCharmId(UniqueId::Placeholder(1)),
            wise_arrow.clone()
        )))
        .is_err());

    event_source
        .apply_mutation(CharacterMutation::SetAbilityDots(
            AbilityNameVanilla::Archery,
            3,
        ))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(
            SolarCharmId(UniqueId::Placeholder(1)),
            wise_arrow.clone(),
        )))
        .unwrap();
    let Charm::Solar(owned_wise_arrow) = character.charms().get(CharmId::Solar(SolarCharmId(UniqueId::Placeholder(1)))).unwrap() else {panic!("Wrong charm type");};
    assert_eq!(owned_wise_arrow, &wise_arrow);

    let character = event_source
        .apply_mutation(CharacterMutation::SetAbilityDots(
            AbilityNameVanilla::Archery,
            1,
        ))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmId::Solar(SolarCharmId(UniqueId::Placeholder(1))))
        .is_none());

    event_source.undo().unwrap();

    // Solars must meet Essence requirements
    let some_expensive_charm = SolarCharm::builder("Some expensive charm".to_owned())
        .essence_required(NonZeroU8::new(2).unwrap())
        .ability_required(SolarCharmAbility::War, 2)
        .action_type(CharmActionType::Permanent)
        .duration("Indefinite".to_owned())
        .description("Some description".to_owned())
        .build();
    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(
            SolarCharmId(UniqueId::Placeholder(2)),
            some_expensive_charm
        )))
        .is_err());

    // ...unless they have the ability as a Supernal ability
    event_source
        .apply_mutation(CharacterMutation::SetAbilityDots(
            AbilityNameVanilla::Lore,
            5,
        ))
        .unwrap();

    let order_affirming_blow = SolarCharm::builder("Order-Affirming Blow".to_owned())
    .book_reference(BookReference::new(Book::CoreRulebook, 334))
    .cost(CharmCostType::Motes, NonZeroU8::new(15).unwrap())
    .cost(CharmCostType::Willpower, NonZeroU8::new(1).unwrap())
    .essence_required(NonZeroU8::new(3).unwrap())
    .ability_required(SolarCharmAbility::Lore, 5)
    .action_type(CharmActionType::Simple)
    .duration("Instant".to_owned())
    .description("The ravages of the Wyld and alterations to the Loom of Fate can be reversed by the Solar Exalted[...]".to_owned())
    .build();

    event_source
        .apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(
            SolarCharmId(UniqueId::Placeholder(3)),
            order_affirming_blow,
        )))
        .unwrap();

    // Solars must meet Charm tree requirements
    event_source
        .apply_mutation(CharacterMutation::RemoveCharm(CharmId::Solar(
            SolarCharmId(UniqueId::Placeholder(1)),
        )))
        .unwrap();

    let sight_without_eyes = SolarCharm::builder("Sight Without Eyes".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 255))
        .charm_prerequisite(SolarCharmId(UniqueId::Placeholder(1)))
        .essence_required(NonZeroU8::new(1).unwrap())
        .ability_required(SolarCharmAbility::Archery, 3)
        .cost(CharmCostType::Motes, NonZeroU8::new(1).unwrap())
        .action_type(CharmActionType::Reflexive)
        .duration("One tick".to_owned())
        .description("The Exalt opens her eyes not to the visual world[...]".to_owned())
        .summary("Ignore vision penalties".to_owned())
        .build();

    assert!(event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(SolarCharmId(UniqueId::Placeholder(4)), sight_without_eyes.clone()))).is_err());

    event_source.undo().unwrap();
    event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(SolarCharmId(UniqueId::Placeholder(4)), sight_without_eyes))).unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::RemoveCharm(CharmId::Solar(
            SolarCharmId(UniqueId::Placeholder(1)),
        )))
        .unwrap();
    assert!(character.charms().get(CharmId::Solar(SolarCharmId(UniqueId::Placeholder(4)))).is_none());
}

#[test]
fn test_evocations() {
    let mut event_source = CharacterEventSource::default();
    // Mortals cannot add Evocations, even if they have the artifact/hearthstone
    let spring_razor_id = ArtifactWeaponId(UniqueId::Placeholder(1));
    let spring_razor = Weapon::artifact("Spring Razor".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 619))
        .lore(
            "Long ago, in the remote jungles of the Silent Crescent,\
        there once dwelt a hateful dragon named Vasshir.".to_owned(),
        )
        .powers(
            "A Solar or Dragon-Blooded who attunes to Spring Razor \
            gains Howling Lotus Strike at no cost.".to_owned(),
        )
        .base_artifact(
            BaseWeaponId(UniqueId::Placeholder(1)),
            Weapon::base("Daiklave".to_owned())
                .book_reference(BookReference::new(Book::CoreRulebook, 595))
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .lethal()
                .tag(OptionalWeaponTag::Balanced)
                .melee()
                .build_artifact(),
        )
        .material(MagicMaterial::GreenJade)
        .merit_dots(3)
        .hearthstone_slots(2)
        .build();

    event_source.apply_mutation(CharacterMutation::AddArtifact(Artifact::Weapon(spring_razor_id, spring_razor))).unwrap();
    
    let carbuncle = Hearthstone::builder("Candent Carbuncle".to_owned())
    .book_reference(BookReference::new(Book::CoreRulebook, 607))
    .category(HearthstoneCategory::Fire)
    .geomancy_level(GeomancyLevel::Greater)
    .powers("This irregularly rounded stone looks like a glowing coal, \
    and is always warm to the touch.".to_owned())
    .build();

    event_source.apply_mutation(CharacterMutation::AddHearthstone(HearthstoneId(UniqueId::Placeholder(1)), carbuncle)).unwrap();

    let howling_lotus_strike_id = EvocationId(UniqueId::Placeholder(1));
    let howling_lotus_strike = Evocation::builder(EvokableId::Artifact(ArtifactId::Weapon(ArtifactWeaponId(UniqueId::Placeholder(1)))), "Howling Lotus Strike".to_owned())
    .book_reference(BookReference::new(Book::CoreRulebook, 620))
    .cost(CharmCostType::Motes, NonZeroU8::new(3).unwrap())
    .essence_required(NonZeroU8::new(1).unwrap())
    .action_type(CharmActionType::Supplemental)
    .keyword(EvocationKeyword::DecisiveOnly)
    .duration("Instant".to_owned())
    .description("Spring Razor's edge burns with venomous emerald Essence as it delivers a fateful, poisonous strike.".to_owned())
    .summary("Adds poison to a decisive attack".to_owned())
    .build();

    let burning_coal_fist = Evocation::builder(EvokableId::Hearthstone(HearthstoneId(UniqueId::Placeholder(1))), "Burning Coal Fist".to_owned())
    .book_reference(BookReference::new(Book::CoreRulebook, 607))
    .cost(CharmCostType::Motes, NonZeroU8::new(3).unwrap())
    .essence_required(NonZeroU8::new(1).unwrap())
    .action_type(CharmActionType::Supplemental)
    .keyword(EvocationKeyword::Dual)
    .duration("Instant".to_owned())
    .description("The Exalt may draw the illimitable heat of Creation into \
    her body, momentarily wreathing an attacking fist or leg \
    in flame. ".to_owned())
    .summary("Fire punch".to_owned()).build();

    assert!(event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(howling_lotus_strike_id, howling_lotus_strike.clone()))).is_err());
    assert!(event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(EvocationId(UniqueId::Placeholder(2)), burning_coal_fist.clone()))).is_err());

    // Exalts must meet Essence requirements
    let new_solar = Solar::builder()
        .twilight()
        .caste_ability(TwilightAbility::Bureaucracy)
        .caste_ability(TwilightAbility::Integrity)
        .caste_ability(TwilightAbility::Craft)
        .caste_ability(TwilightAbility::Investigation)
        .supernal_ability(TwilightAbility::Lore)
        .favored_ability(AbilityName::Archery)
        .favored_ability(AbilityName::Dodge)
        .favored_ability(AbilityName::War)
        .favored_ability(AbilityName::Thrown)
        .favored_ability(AbilityName::Stealth)
        .limit_trigger("A limit trigger".to_owned())
        .build()
        .unwrap();

    event_source
        .apply_mutation(CharacterMutation::SetSolar(new_solar))
        .unwrap();

    event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(howling_lotus_strike_id, howling_lotus_strike.clone()))).unwrap();
    let character = event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(EvocationId(UniqueId::Placeholder(2)), burning_coal_fist.clone()))).unwrap();

    assert!(character.charms().get(CharmId::Evocation(EvocationId(UniqueId::Placeholder(1)))).is_some());
    assert!(character.charms().get(CharmId::Evocation(EvocationId(UniqueId::Placeholder(2)))).is_some());

    let incandescent_lance = Evocation::builder(EvokableId::Hearthstone(HearthstoneId(UniqueId::Placeholder(1))), "Incandescent Lance".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 607))
        .cost(CharmCostType::Motes, NonZeroU8::new(7).unwrap())
        .cost(CharmCostType::Initiative, NonZeroU8::new(3).unwrap())
        .cost(CharmCostType::Willpower, NonZeroU8::new(1).unwrap())
        .essence_required(NonZeroU8::new(2).unwrap())
        .action_type(CharmActionType::Simple)
        .keyword(EvocationKeyword::Dual)
        .duration("Instant".to_owned())
        .description("Drawing on the geomantic power seething in the \
        hearthstone, the Exalt sets her Essence ablaze and hurls \
        it at a foe.".to_owned())
        .summary("Ranged fire punch".to_owned())
        .build();

    assert!(event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(EvocationId(UniqueId::Placeholder(3)), incandescent_lance.clone()))).is_err());

    event_source.apply_mutation(CharacterMutation::SetEssenceRating(2)).unwrap();
    let character = event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(EvocationId(UniqueId::Placeholder(3)), incandescent_lance))).unwrap();
    assert!(character.charms().get(CharmId::Evocation(EvocationId(UniqueId::Placeholder(3)))).is_some());

    let character = event_source.apply_mutation(CharacterMutation::SetEssenceRating(1)).unwrap();
    assert!(character.charms().get(CharmId::Evocation(EvocationId(UniqueId::Placeholder(3)))).is_none());

    // Exalts must have the right artifact or hearthstone
    let wrong_evocation = Evocation::builder(EvokableId::Artifact(ArtifactId::Wonder(WonderId(UniqueId::Placeholder(666)))), "Invalid Evocation".to_owned())
    .essence_required(NonZeroU8::new(1).unwrap())
    .action_type(CharmActionType::Permanent)
    .duration("Indefinite".to_owned())
    .description("A description".to_owned())
    .build();

    assert!(event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(EvocationId(UniqueId::Placeholder(123)), wrong_evocation))).is_err());

    let character = event_source.apply_mutation(CharacterMutation::RemoveHearthstone(HearthstoneId(UniqueId::Placeholder(1)))).unwrap();
    assert!(character.charms().get(CharmId::Evocation(EvocationId(UniqueId::Placeholder(2)))).is_none());

    // Exalts must meet tree requirements
    let venom_intensifying_strike_id = EvocationId(UniqueId::Placeholder(4));
    let venom_intensifying_strike = Evocation::builder(EvokableId::Artifact(ArtifactId::Weapon(ArtifactWeaponId(UniqueId::Placeholder(1)))), "Seven Widows Venom".to_owned())
    .book_reference(BookReference::new(Book::CoreRulebook, 620))
    .cost(CharmCostType::Motes, NonZeroU8::new(3).unwrap())
    .essence_required(NonZeroU8::new(1).unwrap())
    .action_type(CharmActionType::Supplemental)
    .keyword(EvocationKeyword::Stackable)
    .keyword(EvocationKeyword::Uniform)
    .duration("Instant".to_owned())
    .evocation_prerequisite(EvocationId(UniqueId::Placeholder(1)))
    .description("A rippling haze of emerald-and-scarlet Essence roils \
    around Spring Razor’s edge as the wielder slashes at her \
    foes.".to_owned())
    .summary("Intensifies existing poisons on hit".to_owned())
    .build();
    
    let seven_widows_venom_id = EvocationId(UniqueId::Placeholder(5));
    let seven_widows_venom = Evocation::builder(EvokableId::Artifact(ArtifactId::Weapon(ArtifactWeaponId(UniqueId::Placeholder(1)))), "Seven Widows Venom".to_owned())
    .book_reference(BookReference::new(Book::CoreRulebook, 620))
    .essence_required(NonZeroU8::new(1).unwrap()) // Simplifying test, is actually 2 in the book
    .action_type(CharmActionType::Permanent)
    .duration("Permanent".to_owned())
    .evocation_prerequisite(venom_intensifying_strike_id)
    .description("This Evocation permanently intensifies the poison \
    produced by Howling Lotus Strike, ensuring that no matter \
    how well the target rolls to resist the poison, its \
    duration cannot be reduced below one round.".to_owned())
    .summary("Sets minimum poison duration".to_owned())
    .build();

    assert!(event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(seven_widows_venom_id, seven_widows_venom.clone()))).is_err());
    event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(venom_intensifying_strike_id, venom_intensifying_strike))).unwrap();
    let character = event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(seven_widows_venom_id, seven_widows_venom))).unwrap();
    assert!(character.charms().get(CharmId::Evocation(seven_widows_venom_id)).is_some());

    let character = event_source.apply_mutation(CharacterMutation::RemoveCharm(CharmId::Evocation(howling_lotus_strike_id))).unwrap();
    assert!(character.charms().get(CharmId::Evocation(howling_lotus_strike_id)).is_some());
    assert!(character.charms().get(CharmId::Evocation(venom_intensifying_strike_id)).is_some());
    assert!(character.charms().get(CharmId::Evocation(seven_widows_venom_id)).is_some());

    // Upgrade-type Evocations require the upgraded Charm
    let integrity_protecting_prana_id = SolarCharmId(UniqueId::Placeholder(1));
    let integrity_protecting_prana = SolarCharm::builder("Integrity-Protecting Prana".to_owned())
    .book_reference(BookReference::new(Book::CoreRulebook, 303))
    .essence_required(NonZeroU8::new(1).unwrap())
    .ability_required(SolarCharmAbility::Integrity, 3)
    .action_type(CharmActionType::Permanent)
    .duration("Permanent".to_owned())
    .description("Exposure to Wyld energies can cause hallucinations, addiction, \
    insanity, and even terrible mutations. When the Exalt is exposed to such energies[...]".to_owned())
    .summary("Immunize against Wyld mutations".to_owned())
    .build();

    let rainwalker_id = ArtifactWeaponId(UniqueId::Placeholder(2));
    let rainwalker = Weapon::artifact("Rainwalker".to_owned())
    .base_artifact(BaseWeaponId(UniqueId::Placeholder(2)), 
        Weapon::base("Razor Parasol".to_owned())
        .book_reference(BookReference::new(Book::ArmsOfTheChosen, 39))
        .weight_class(WeaponWeightClass::Medium)
        .one_handed()
        .lethal()
        .melee()
        .tag(OptionalWeaponTag::Concealable)
        .tag(OptionalWeaponTag::Disarming)
        .tag(OptionalWeaponTag::Shield)
        .build_artifact()
    )
    .book_reference(BookReference::new(Book::ArmsOfTheChosen, 39))
    .lore("The Amethyst Lord was a wicked prince of the Fair Folk[...]".to_owned())
    .powers("Rainwalker's wielder ignores environmental penalties from \
    rainfall or other precipitation.".to_owned())
    .material(MagicMaterial::BlueJade)
    .merit_dots(3)
    .hearthstone_slots(1)
    .build();

    let glamour_sloughing_parasol_id = EvocationId(UniqueId::Placeholder(6));
    let glamour_sloughing_parasol = Evocation::builder(EvokableId::Artifact(ArtifactId::Weapon(rainwalker_id)), "Glamour-Sloughing Parasol".to_owned())
    .book_reference(BookReference::new(Book::ArmsOfTheChosen, 40))
    .essence_required(NonZeroU8::new(1).unwrap())
    .action_type(CharmActionType::Permanent)
    .duration("Permanent".to_owned())
    .description("This Evocation upgrades Integrity-Protecting Prana lowering \
    the cost of using it to one mote and one point of Willpower.".to_owned())
    .summary("Discount for Integrity-Protecting Prana".to_owned())
    .upgrades(CharmId::Solar(integrity_protecting_prana_id)) // Simplifying test, ignoring Breeze-Catching Descent
    .build();

    event_source.apply_mutation(CharacterMutation::AddArtifact(Artifact::Weapon(rainwalker_id, rainwalker))).unwrap();
    assert!(event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(glamour_sloughing_parasol_id, glamour_sloughing_parasol.clone()))).is_err());
    event_source.apply_mutation(CharacterMutation::SetAbilityDots(AbilityNameVanilla::Integrity, 3)).unwrap();
    event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(integrity_protecting_prana_id, integrity_protecting_prana))).unwrap();
    let character = event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Evocation(glamour_sloughing_parasol_id, glamour_sloughing_parasol.clone()))).unwrap();
    assert!(character.charms().get(CharmId::Evocation(glamour_sloughing_parasol_id)).is_some());

    let character = event_source.apply_mutation(CharacterMutation::SetAbilityDots(AbilityNameVanilla::Integrity, 1)).unwrap();
    assert!(character.charms().get(CharmId::Solar(integrity_protecting_prana_id)).is_none());
    assert!(character.charms().get(CharmId::Evocation(glamour_sloughing_parasol_id)).is_none());
}

#[test]
fn test_spells() {
    // Non-sorcerers cannot add Spells
    // Mortal sorcerers can add Terrestrial Spells
    // Solar Terrestrial sorcerers can add Terrestrial Spells
    // ...but not Celestial Spells or Solar circle spells
    // Solar Celestial sorcerer can add Terrestrial or Celestial spells
    // ...but not Solar circle spells
    // Solar Solar sorcerers can add any spell
}

#[test]
fn test_martial_arts_charms() {
    // Mortals cannot add MA charms, even if they have the right style
    // Exalts must have the right MA style
    // Exalts must meet the MA ability requirements of charms
    // Exalts must meet the Essence requirements of charms
    // ...unless they are Dawn Solars with Martial Arts Supernal
}

#[test]
fn test_eclipse_charms() {
    // Mortals cannot add Eclipse charms
    // Non-Eclipse Solars cannot add Eclipse charms
    // Eclipse Solars must meet the Essence requirement of Eclipse Charms
}
