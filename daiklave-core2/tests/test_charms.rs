use std::num::NonZeroU8;

use daiklave_core2::{
    abilities::{AbilityName, AbilityNameVanilla},
    armor::armor_item::ArmorWeightClass,
    artifact::{AddArtifact, ArtifactNameMutation, MagicMaterial},
    book_reference::{Book, BookReference},
    charms::{
        charm::{
            evocation::{Evocation, EvocationKeyword, EvokableNameMutation},
            spirit::SpiritCharm,
            AddCharm, Charm, CharmName, CharmNameMutation, SpiritCharmKeyword,
        },
        CharmActionType, CharmCostType,
    },
    exaltation::exalt::exalt_type::solar::{
        caste::{
            DawnCasteAbility, DawnSupernalAbility, EclipseAbility, NightAbility, TwilightAbility,
            ZenithAbility,
        },
        charm::{SolarCharm, SolarCharmAbility, SolarCharmKeyword},
        Solar,
    },
    hearthstones::hearthstone::{GeomancyLevel, Hearthstone, HearthstoneCategory},
    martial_arts::{
        charm::{MartialArtsCharmDetails, MartialArtsCharmKeyword},
        style::MartialArtsStyle,
    },
    sorcery::{
        spell::{Spell, SpellKeyword},
        ShapingRitual, Sorcery, SorceryArchetype, SorceryCircle,
    },
    weapons::weapon::{OptionalWeaponTag, Weapon, WeaponWeightClass},
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_solar_charms() {
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character().unwrap();
    assert!(character.charms().iter().next().is_none());

    // Mortals cannot add Solar Charms, even if they meet the ability prerequisites
    event_source
        .apply_mutation(CharacterMutation::SetAbility(
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
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Solar(
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
            .apply_mutation(CharacterMutation::SetAbility(ability_name, 1))
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
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Solar(
            wise_arrow.clone()
        )))
        .is_err());

    event_source
        .apply_mutation(CharacterMutation::SetAbility(
            AbilityNameVanilla::Archery,
            3,
        ))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Solar(
            wise_arrow.clone(),
        )))
        .unwrap();
    let Charm::Solar(owned_wise_arrow) = character.charms().get(CharmName::Solar("Wise Arrow")).unwrap() else {panic!("Wrong charm type");};
    assert_eq!(owned_wise_arrow, &wise_arrow.1);

    let character = event_source
        .apply_mutation(CharacterMutation::SetAbility(
            AbilityNameVanilla::Archery,
            1,
        ))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Solar("Wise Arrow"))
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
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Solar(
            some_expensive_charm
        )))
        .is_err());

    // ...unless they have the ability as a Supernal ability
    event_source
        .apply_mutation(CharacterMutation::SetAbility(
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
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Solar(
            order_affirming_blow,
        )))
        .unwrap();

    // Solars must meet Charm tree requirements
    event_source
        .apply_mutation(CharacterMutation::RemoveCharm(CharmNameMutation::Solar(
            "Wise Arrow".to_owned(),
        )))
        .unwrap();

    let sight_without_eyes = SolarCharm::builder("Sight Without Eyes".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 255))
        .charm_prerequisite("Wise Arrow".to_owned())
        .essence_required(NonZeroU8::new(1).unwrap())
        .ability_required(SolarCharmAbility::Archery, 3)
        .cost(CharmCostType::Motes, NonZeroU8::new(1).unwrap())
        .action_type(CharmActionType::Reflexive)
        .duration("One tick".to_owned())
        .description("The Exalt opens her eyes not to the visual world[...]".to_owned())
        .summary("Ignore vision penalties".to_owned())
        .build();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Solar(
            sight_without_eyes.clone()
        )))
        .is_err());

    event_source.undo().unwrap();
    event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Solar(
            sight_without_eyes,
        )))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::RemoveCharm(CharmNameMutation::Solar(
            "Wise Arrow".to_owned(),
        )))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Solar("Sight Without Eyes"))
        .is_none());
}

#[test]
fn test_evocations() {
    let mut event_source = CharacterEventSource::default();
    // Mortals cannot add Evocations, even if they have the artifact/hearthstone
    let spring_razor = Weapon::artifact("Spring Razor".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 619))
        .lore(
            "Long ago, in the remote jungles of the Silent Crescent,\
        there once dwelt a hateful dragon named Vasshir."
                .to_owned(),
        )
        .powers(
            "A Solar or Dragon-Blooded who attunes to Spring Razor \
            gains Howling Lotus Strike at no cost."
                .to_owned(),
        )
        .base_artifact(
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

    event_source
        .apply_mutation(CharacterMutation::AddArtifact(AddArtifact::Weapon(
            spring_razor,
        )))
        .unwrap();

    let carbuncle = Hearthstone::builder("Candent Carbuncle".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 607))
        .category(HearthstoneCategory::Fire)
        .geomancy_level(GeomancyLevel::Greater)
        .powers(
            "This irregularly rounded stone looks like a glowing coal, \
    and is always warm to the touch."
                .to_owned(),
        )
        .build();

    event_source
        .apply_mutation(CharacterMutation::AddHearthstone(carbuncle))
        .unwrap();

    let howling_lotus_strike = Evocation::builder(EvokableNameMutation::Artifact(ArtifactNameMutation::Weapon("Spring Razor".to_owned())), "Howling Lotus Strike".to_owned())
    .book_reference(BookReference::new(Book::CoreRulebook, 620))
    .cost(CharmCostType::Motes, NonZeroU8::new(3).unwrap())
    .essence_required(NonZeroU8::new(1).unwrap())
    .action_type(CharmActionType::Supplemental)
    .keyword(EvocationKeyword::DecisiveOnly)
    .duration("Instant".to_owned())
    .description("Spring Razor's edge burns with venomous emerald Essence as it delivers a fateful, poisonous strike.".to_owned())
    .summary("Adds poison to a decisive attack".to_owned())
    .build();

    let burning_coal_fist = Evocation::builder(
        EvokableNameMutation::Hearthstone("Candent Carbuncle".to_owned()),
        "Burning Coal Fist".to_owned(),
    )
    .book_reference(BookReference::new(Book::CoreRulebook, 607))
    .cost(CharmCostType::Motes, NonZeroU8::new(3).unwrap())
    .essence_required(NonZeroU8::new(1).unwrap())
    .action_type(CharmActionType::Supplemental)
    .keyword(EvocationKeyword::Dual)
    .duration("Instant".to_owned())
    .description(
        "The Exalt may draw the illimitable heat of Creation into \
    her body, momentarily wreathing an attacking fist or leg \
    in flame. "
            .to_owned(),
    )
    .summary("Fire punch".to_owned())
    .build();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            howling_lotus_strike.clone()
        )))
        .is_err());
    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            burning_coal_fist.clone()
        )))
        .is_err());

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

    event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            howling_lotus_strike.clone(),
        )))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            burning_coal_fist.clone(),
        )))
        .unwrap();

    assert!(character
        .charms()
        .get(CharmName::Evocation("Burning Coal Fist"))
        .is_some());
    assert!(character
        .charms()
        .get(CharmName::Evocation("Howling Lotus Strike"))
        .is_some());

    let incandescent_lance = Evocation::builder(
        EvokableNameMutation::Hearthstone("Candent Carbuncle".to_owned()),
        "Incandescent Lance".to_owned(),
    )
    .book_reference(BookReference::new(Book::CoreRulebook, 607))
    .cost(CharmCostType::Motes, NonZeroU8::new(7).unwrap())
    .cost(CharmCostType::Initiative, NonZeroU8::new(3).unwrap())
    .cost(CharmCostType::Willpower, NonZeroU8::new(1).unwrap())
    .essence_required(NonZeroU8::new(2).unwrap())
    .action_type(CharmActionType::Simple)
    .keyword(EvocationKeyword::Dual)
    .duration("Instant".to_owned())
    .description(
        "Drawing on the geomantic power seething in the \
        hearthstone, the Exalt sets her Essence ablaze and hurls \
        it at a foe."
            .to_owned(),
    )
    .summary("Ranged fire punch".to_owned())
    .build();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            incandescent_lance.clone()
        )))
        .is_err());

    event_source
        .apply_mutation(CharacterMutation::SetEssenceRating(2))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            incandescent_lance,
        )))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Evocation("Incandescent Lance"))
        .is_some());

    let character = event_source
        .apply_mutation(CharacterMutation::SetEssenceRating(1))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Evocation("Incandescent Lance"))
        .is_none());

    // Exalts must have the right artifact or hearthstone
    let wrong_evocation = Evocation::builder(
        EvokableNameMutation::Artifact(ArtifactNameMutation::Wonder("A missing wonder".to_owned())),
        "Invalid Evocation".to_owned(),
    )
    .essence_required(NonZeroU8::new(1).unwrap())
    .action_type(CharmActionType::Permanent)
    .duration("Indefinite".to_owned())
    .description("A description".to_owned())
    .build();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            wrong_evocation
        )))
        .is_err());

    let character = event_source
        .apply_mutation(CharacterMutation::RemoveHearthstone(
            "Candent Carbuncle".to_owned(),
        ))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Evocation("Burning Coal Fist"))
        .is_none());

    // Exalts must meet tree requirements
    let venom_intensifying_strike = Evocation::builder(
        EvokableNameMutation::Artifact(ArtifactNameMutation::Weapon("Spring Razor".to_owned())),
        "Venom Intensifying Strike".to_owned(),
    )
    .book_reference(BookReference::new(Book::CoreRulebook, 620))
    .cost(CharmCostType::Motes, NonZeroU8::new(3).unwrap())
    .essence_required(NonZeroU8::new(1).unwrap())
    .action_type(CharmActionType::Supplemental)
    .keyword(EvocationKeyword::Stackable)
    .keyword(EvocationKeyword::Uniform)
    .duration("Instant".to_owned())
    .evocation_prerequisite("Howling Lotus Strike".to_owned())
    .description(
        "A rippling haze of emerald-and-scarlet Essence roils \
    around Spring Razor’s edge as the wielder slashes at her \
    foes."
            .to_owned(),
    )
    .summary("Intensifies existing poisons on hit".to_owned())
    .build();

    let seven_widows_venom = Evocation::builder(
        EvokableNameMutation::Artifact(ArtifactNameMutation::Weapon("Spring Razor".to_owned())),
        "Seven Widows Venom".to_owned(),
    )
    .book_reference(BookReference::new(Book::CoreRulebook, 620))
    .essence_required(NonZeroU8::new(1).unwrap()) // Simplifying test, is actually 2 in the book
    .action_type(CharmActionType::Permanent)
    .duration("Permanent".to_owned())
    .evocation_prerequisite("Venom Intensifying Strike".to_owned())
    .description(
        "This Evocation permanently intensifies the poison \
    produced by Howling Lotus Strike, ensuring that no matter \
    how well the target rolls to resist the poison, its \
    duration cannot be reduced below one round."
            .to_owned(),
    )
    .summary("Sets minimum poison duration".to_owned())
    .build();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            seven_widows_venom.clone()
        )))
        .is_err());
    event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            venom_intensifying_strike,
        )))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            seven_widows_venom,
        )))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Evocation("Seven Widows Venom"))
        .is_some());

    let character = event_source
        .apply_mutation(CharacterMutation::RemoveCharm(
            CharmNameMutation::Evocation("Howling Lotus Strike".to_owned()),
        ))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Evocation("Howling Lotus Strike"))
        .is_none());
    assert!(character
        .charms()
        .get(CharmName::Evocation("Venom Intensifying Strike"))
        .is_none());
    assert!(character
        .charms()
        .get(CharmName::Evocation("Seven Widows Venom"))
        .is_none());

    // Upgrade-type Evocations require the upgraded Charm
    let integrity_protecting_prana = SolarCharm::builder("Integrity-Protecting Prana".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 303))
        .essence_required(NonZeroU8::new(1).unwrap())
        .ability_required(SolarCharmAbility::Integrity, 3)
        .action_type(CharmActionType::Permanent)
        .duration("Permanent".to_owned())
        .description(
            "Exposure to Wyld energies can cause hallucinations, addiction, \
    insanity, and even terrible mutations. When the Exalt is exposed to such energies[...]"
                .to_owned(),
        )
        .summary("Immunize against Wyld mutations".to_owned())
        .build();

    let rainwalker = Weapon::artifact("Rainwalker".to_owned())
        .base_artifact(
            Weapon::base("Razor Parasol".to_owned())
                .book_reference(BookReference::new(Book::ArmsOfTheChosen, 39))
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .lethal()
                .melee()
                .tag(OptionalWeaponTag::Concealable)
                .tag(OptionalWeaponTag::Disarming)
                .tag(OptionalWeaponTag::Shield)
                .build_artifact(),
        )
        .book_reference(BookReference::new(Book::ArmsOfTheChosen, 39))
        .lore("The Amethyst Lord was a wicked prince of the Fair Folk[...]".to_owned())
        .powers(
            "Rainwalker's wielder ignores environmental penalties from \
    rainfall or other precipitation."
                .to_owned(),
        )
        .material(MagicMaterial::BlueJade)
        .merit_dots(3)
        .hearthstone_slots(1)
        .build();

    let glamour_sloughing_parasol = Evocation::builder(
        EvokableNameMutation::Artifact(ArtifactNameMutation::Weapon("Rainwalker".to_owned())),
        "Glamour-Sloughing Parasol".to_owned(),
    )
    .book_reference(BookReference::new(Book::ArmsOfTheChosen, 40))
    .essence_required(NonZeroU8::new(1).unwrap())
    .action_type(CharmActionType::Permanent)
    .duration("Permanent".to_owned())
    .description(
        "This Evocation upgrades Integrity-Protecting Prana lowering \
    the cost of using it to one mote and one point of Willpower."
            .to_owned(),
    )
    .summary("Discount for Integrity-Protecting Prana".to_owned())
    .upgrades(CharmNameMutation::Solar(
        "Integrity-Protecting Prana".to_owned(),
    )) // Simplifying test, ignoring Breeze-Catching Descent
    .build();

    event_source
        .apply_mutation(CharacterMutation::AddArtifact(AddArtifact::Weapon(
            rainwalker,
        )))
        .unwrap();
    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            glamour_sloughing_parasol.clone()
        )))
        .is_err());
    event_source
        .apply_mutation(CharacterMutation::SetAbility(
            AbilityNameVanilla::Integrity,
            3,
        ))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Solar(
            integrity_protecting_prana,
        )))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Evocation(
            glamour_sloughing_parasol.clone(),
        )))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Evocation("Glamour-Sloughing Parasol"))
        .is_some());

    let character = event_source
        .apply_mutation(CharacterMutation::SetAbility(
            AbilityNameVanilla::Integrity,
            1,
        ))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Solar("Integrity-Protecting Prana"))
        .is_none());
    assert!(character
        .charms()
        .get(CharmName::Evocation("Glamour-Sloughing Parasol"))
        .is_none());
}

#[test]
fn test_spells() {
    let mut event_source = CharacterEventSource::default();
    // Non-sorcerers cannot add Spells
    let cirrus_skiff = Spell::builder("Cirrus Skiff".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 471))
        .sorcerous_motes(NonZeroU8::new(15).unwrap())
        .willpower(NonZeroU8::new(1).unwrap())
        .duration("Until Ended".to_owned())
        .summary("Summon cloud to ride on".to_owned())
        .description(
            "The sorcerer calls down a Cirrus Skiff to bear her skyward, \
    a small white puffy cloud just large enough for her and \
    one other passenger to ride upon."
                .to_owned(),
        )
        .control_spell_description(
            "A character who knows Cirrus Skiff as her control spell \
    may cast it with an Indefinite duration."
                .to_owned(),
        )
        .distortion(
            NonZeroU8::new(7).unwrap(),
            "Distorting a Cirrus Skiff \
    weighs it down, turning the cloud into a heavy, dense fog \
    for a scene."
                .to_owned(),
        )
        .build(SorceryCircle::Terrestrial);

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Spell(
            cirrus_skiff.clone()
        )))
        .is_err());

    // Mortal sorcerers can add Terrestrial Spells
    event_source
        .apply_mutation(CharacterMutation::SetAbility(
            AbilityNameVanilla::Occult,
            5,
        ))
        .unwrap();

    let add_sorcery = Sorcery::builder()
        .terrestrial()
        .archetype(SorceryArchetype::new(
            "Scarred by Nightmares".to_owned(),
            Some(BookReference::new(Book::CoreRulebook, 468)),
            "Perhaps you were a child lost in the warped depths of the \
        Wyld or a hero treading where the logic of the world \
        crumbles away[...]"
                .to_owned(),
        ))
        .shaping_ritual(ShapingRitual::new(
            "Scarred by Nightmares".to_owned(),
            "Gain motes from nightmares".to_owned(),
            Some(BookReference::new(Book::CoreRulebook, 468)),
            "When the sorcerer sleeps, her player may describe the \
        strange visions that haunt her."
                .to_owned(),
        ))
        .unwrap()
        .control_spell(
            Spell::builder("Corrupted Words".to_owned())
                .book_reference(BookReference::new(Book::CoreRulebook, 472))
                .sorcerous_motes(NonZeroU8::new(15).unwrap())
                .willpower(NonZeroU8::new(1).unwrap())
                .keyword(SpellKeyword::Psyche)
                .duration("Indefinite".to_owned())
                .description(
                    "The sorcerer speaks words that bind the tongue—she \
                    evokes a seething, bilious mass of green Essence between \
                    her hands and casts it down the throat of a victim at short \
                    range, where it dissolves into a ward that censors that \
                    character's mind."
                        .to_owned(),
                )
                .control_spell_description(
                    "A sorcerer who knows Corrupted Words as her control \
                    spell may cast it with no obvious display of magical \
                    intervention."
                        .to_owned(),
                )
                .distortion(
                    NonZeroU8::new(15).unwrap(),
                    "Distorting this curse makes \
                    it possible for the victim to speak around the forbidden \
                    subject matter for five minutes."
                        .to_owned(),
                )
                .summary("Prevent someone from talking about something".to_owned())
                .terrestrial(),
        );

    let mutation = CharacterMutation::AddSorcery(Box::new(add_sorcery));
    let character = event_source.apply_mutation(mutation).unwrap();
    assert!(character
        .charms()
        .get(CharmName::Spell("Corrupted Words"))
        .is_some());
    assert_eq!(
        character
            .sorcery()
            .unwrap()
            .spells()
            .get("Corrupted Words")
            .unwrap()
            .1,
        true
    );

    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Spell(cirrus_skiff)))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Spell("Cirrus Skiff"))
        .is_some());
    assert_eq!(
        character
            .sorcery()
            .unwrap()
            .spells()
            .get("Cirrus Skiff")
            .unwrap()
            .1,
        false
    );

    // Solar Terrestrial sorcerers can add Terrestrial Spells
    [
        AbilityNameVanilla::Archery,
        AbilityNameVanilla::Awareness,
        AbilityNameVanilla::Brawl,
        AbilityNameVanilla::Bureaucracy,
        AbilityNameVanilla::Dodge,
    ]
    .into_iter()
    .for_each(|vanilla| {
        event_source
            .apply_mutation(CharacterMutation::SetAbility(vanilla, 1))
            .unwrap();
    });

    let new_solar = Solar::builder()
        .zenith()
        .caste_ability(ZenithAbility::Athletics)
        .caste_ability(ZenithAbility::Integrity)
        .caste_ability(ZenithAbility::Lore)
        .caste_ability(ZenithAbility::Performance)
        .supernal_ability(ZenithAbility::Presence)
        .favored_ability(AbilityName::Archery)
        .favored_ability(AbilityName::Awareness)
        .favored_ability(AbilityName::Brawl)
        .favored_ability(AbilityName::Bureaucracy)
        .favored_ability(AbilityName::Dodge)
        .limit_trigger("Some limit trigger".to_owned())
        .build()
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::SetSolar(new_solar))
        .unwrap();

    let butterflies = Spell::builder("Death of Obsidian Butterflies".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 470))
        .keyword(SpellKeyword::DecisiveOnly)
        .keyword(SpellKeyword::Perilous)
        .sorcerous_motes(NonZeroU8::new(15).unwrap())
        .willpower(NonZeroU8::new(1).unwrap())
        .duration("Instant".to_owned())
        .description("Sculpting Essence into volant black glass, the sorcerer unleashes a cascade of obsidian butterflies[...]".to_owned())
        .control_spell_description("A sorcerer who knows Death of Obsidian Butterflies as her control spell gains (Essence) bonus dice to the spells attack roll[...]".to_owned())
        .summary("AOE attack that makes difficult terrain".to_owned())
        .build(SorceryCircle::Terrestrial);
    event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Spell(butterflies)))
        .unwrap();

    // ...but not Celestial Spells or Solar circle spells
    let demon = Spell::builder("Demon of the Second Circle".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 477))
        .ritual()
        .willpower(NonZeroU8::new(3).unwrap())
        .duration("Instant".to_owned())
        .summary("Summon a second-circle demon".to_owned())
        .description(
            "Celestial sorcerers may call upon demons of the Second \
    Circle, baleful spirits that serve the Yozis."
                .to_owned(),
        )
        .build(SorceryCircle::Celestial);

    let death_ray = Spell::builder("Death Ray".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 481))
        .sorcerous_motes(NonZeroU8::new(25).unwrap())
        .willpower(NonZeroU8::new(2).unwrap())
        .keyword(SpellKeyword::Aggravated)
        .keyword(SpellKeyword::DecisiveOnly)
        .keyword(SpellKeyword::Perilous)
        .duration("Instant or until ended".to_owned())
        .summary("Devastating laser beam".to_owned())
        .description(
            "The Solar Exalted devised this spell to terrify the enemies \
    of the gods."
                .to_owned(),
        )
        .control_spell_description(
            "A sorcerer who knows Death Ray as her control spell may \
    gain sorcerous motes whenever the Essence blast directly \
    incapacitates a character or deals damage to a battle group."
                .to_owned(),
        )
        .distortion(
            NonZeroU8::new(10).unwrap(),
            "Distorting the Death Ray
    gives the opposing sorcerer a measure of protection from \
    it, causing the destructive energy to bend and warp around \
    him before continuing on its original course."
                .to_owned(),
        )
        .build(SorceryCircle::Solar);

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Spell(demon.clone())))
        .is_err());
    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Spell(
            death_ray.clone()
        )))
        .is_err());

    // Solar Celestial sorcerer can add Terrestrial or Celestial spells
    let add_sorcery = Sorcery::builder()
        .celestial()
        .existing_archetype("Scarred by Nightmares".to_owned())
        .shaping_ritual(ShapingRitual::new(
            "Scarred by Nightmares".to_owned(),
            "Gain motes from a taboo or delusion".to_owned(),
            Some(BookReference::new(Book::CoreRulebook, 468)),
            "The sorcerer abides by an esoteric taboo or is victim to a \
    delusional belief inspired by the Wyld energies inside her \
    mind, a Defining Derangement which cannot be removed \
    or altered."
                .to_owned(),
        ))
        .unwrap()
        .control_spell(
            Spell::builder("Magma Kraken".to_owned())
                .book_reference(BookReference::new(Book::CoreRulebook, 478))
                .sorcerous_motes(NonZeroU8::new(30).unwrap())
                .willpower(NonZeroU8::new(1).unwrap())
                .duration("One scene".to_owned())
                .summary("Summons tentacles of magma from the ground".to_owned())
                .description(
                    "Calling to the Essence of fire and earth that roils deep beneath \
her, the sorcerer wills ten tentacles of lava to erupt from the \
ground, shaking the earth as they burst forward in torrents \
of molten stone."
                        .to_owned(),
                )
                .control_spell_description(
                    "A sorcerer who knows Magma Kraken as her control spell \
is forever chased by magmatic flame."
                        .to_owned(),
                )
                .distortion(
                    NonZeroU8::new(10).unwrap(),
                    " Distorting a Magma Kraken \
renders it incapable of perceiving and attacking the \
distorting sorcerer as well as all allies within close range \
of her."
                        .to_owned(),
                )
                .celestial(),
        );

    event_source
        .apply_mutation(CharacterMutation::SetEssenceRating(3))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::AddSorcery(Box::new(add_sorcery)))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Spell(demon)))
        .unwrap();

    // ...but not Solar circle spells
    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Spell(
            death_ray.clone()
        )))
        .is_err());

    // Solar Solar sorcerers can add any spell
    let add_sorcery = Sorcery::builder()
        .solar()
        .existing_archetype("Scarred by Nightmares".to_owned())
        .shaping_ritual(ShapingRitual::new(
            "Scarred by Nightmares".to_owned(),
            "Gain motes by feeding on others' emotions".to_owned(),
            Some(BookReference::new(Book::CoreRulebook, 468)),
            "The sorcerer may feed on emotional energies like the \
    raksha, shaping the passions of others through spellcraft."
                .to_owned(),
        ))
        .unwrap()
        .control_spell(
            Spell::builder("Benediction of Archgenesis".to_owned())
                .book_reference(BookReference::new(Book::CoreRulebook, 480))
                .ritual()
                .willpower(NonZeroU8::new(3).unwrap())
                .duration("Instant".to_owned())
                .summary("Make the land fertile".to_owned())
                .description(
                    "Chanting from one sunrise to the next, the sorcerer \
calls down a soft, warm rain that carries life-giving Essence."
                        .to_owned(),
                )
                .control_spell_description(
                    "A sorcerer who knows Benediction of Archgenesis as \
her control spell reduces the distance requirement to \
one hundred miles."
                        .to_owned(),
                )
                .distortion(
                    NonZeroU8::new(30).unwrap(),
                    "Distorting the Benediction \
of Archgenesis causes all plant life within (Essence) miles \
of the distorting sorcerer to wither and die, leaving a barren \
dead zone within the blessed land."
                        .to_owned(),
                )
                .solar(),
        );

    event_source
        .apply_mutation(CharacterMutation::SetEssenceRating(5))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::AddSorcery(Box::new(add_sorcery)))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Spell(death_ray)))
        .unwrap();
}

#[test]
fn test_martial_arts_charms() {
    let mut event_source = CharacterEventSource::default();
    // Mortals cannot add MA charms, even if they have the right style
    let (single_point_name, single_point) =
        MartialArtsStyle::builder("Single Point Shining Into the Void Style".to_owned())
            .book_reference(BookReference::new(Book::CoreRulebook, 434))
            .description(
                "Single Point Shining Into the Void is a sword style that\
    emphasizes blinding speed and deadly-perfect finishing \
    moves."
                    .to_owned(),
            )
            .weapon("Slashing Sword".to_owned())
            .weapon("Reaper Daiklave".to_owned())
            .max_armor_weight(ArmorWeightClass::Medium)
            .build();

    event_source
        .apply_mutation(CharacterMutation::SetAbility(
            AbilityNameVanilla::Brawl,
            1,
        ))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::AddMartialArtsStyle(
            single_point_name,
            single_point,
        ))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::SetMartialArtsDots(
            "Single Point Shining Into the Void Style".to_owned(),
            2,
        ))
        .unwrap();

    let gathering_light_concentration = MartialArtsCharmDetails::builder(
        "Gathering Light Concentration".to_owned(),
        "Single Point Shining Into the Void Style".to_owned(),
    )
    .book_reference(BookReference::new(Book::CoreRulebook, 434))
    .cost(CharmCostType::Motes, NonZeroU8::new(3).unwrap())
    .essence_required(NonZeroU8::new(1).unwrap())
    .ability_required(NonZeroU8::new(2).unwrap())
    .action_type(CharmActionType::Reflexive)
    .keyword(MartialArtsCharmKeyword::Uniform)
    .duration("Instant".to_owned())
    .summary("Reflect onslaught penalties after parry".to_owned())
    .description(
        "The clashing steel and ferocious blows of the swordsman's \
    enemies do not disrupt her focus—rather, she welcomes \
    them, gleaning the weaknesses of each foe's fighting style \
    from their offense."
            .to_owned(),
    )
    .mastery(
        "At Essence 3+, the Solar may spend an extra 3i \
    when activating Gathering Light Concentration to cancel \
    all onslaught penalties she's suffering from, and inflict \
    them on her attacker."
            .to_owned(),
    )
    .build();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::MartialArts(
            gathering_light_concentration.clone()
        )))
        .is_err());

    // Exalts must have the right MA style
    [
        AbilityNameVanilla::Medicine,
        AbilityNameVanilla::Melee,
        AbilityNameVanilla::Resistance,
        AbilityNameVanilla::Sail,
    ]
    .into_iter()
    .for_each(|vanilla| {
        event_source
            .apply_mutation(CharacterMutation::SetAbility(vanilla, 1))
            .unwrap();
    });

    let new_solar = Solar::builder()
        .night()
        .caste_ability(NightAbility::Athletics)
        .caste_ability(NightAbility::Awareness)
        .caste_ability(NightAbility::Dodge)
        .caste_ability(NightAbility::Larceny)
        .supernal_ability(NightAbility::Stealth)
        .favored_ability(AbilityName::Medicine)
        .favored_ability(AbilityName::Melee)
        .favored_ability(AbilityName::Brawl)
        .favored_ability(AbilityName::Resistance)
        .favored_ability(AbilityName::Sail)
        .limit_trigger("A limit trigger".to_owned())
        .build()
        .unwrap();

    event_source
        .apply_mutation(CharacterMutation::SetSolar(new_solar))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::MartialArts(
            gathering_light_concentration,
        )))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::MartialArts("Gathering Light Concentration"))
        .is_some());

    // Exalts must meet the MA ability requirements of charms
    let shining_starfall_execution = MartialArtsCharmDetails::builder(
        "Shining Starfall Execution".to_owned(),
        "Single Point Shining Into the Void Style".to_owned(),
    )
    .book_reference(BookReference::new(Book::CoreRulebook, 434))
    .cost(CharmCostType::Motes, NonZeroU8::new(6).unwrap())
    .essence_required(NonZeroU8::new(1).unwrap())
    .ability_required(NonZeroU8::new(3).unwrap())
    .action_type(CharmActionType::Supplemental)
    .keyword(MartialArtsCharmKeyword::DecisiveOnly)
    .duration("Instant".to_owned())
    .summary("Extra Decisive damage".to_owned())
    .description(
        "Committing fully to a lethal blow, the swordsman cleaves \
    through her enemies with killing speed."
            .to_owned(),
    )
    .mastery(
        "Shining Starfall Execution also doubles 10s on \
    the damage roll at Initiative 15+."
            .to_owned(),
    )
    .build();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::MartialArts(
            shining_starfall_execution.clone()
        )))
        .is_err());
    event_source
        .apply_mutation(CharacterMutation::SetMartialArtsDots(
            "Single Point Shining Into the Void Style".to_owned(),
            5,
        ))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::MartialArts(
            shining_starfall_execution,
        )))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::MartialArts("Shining Starfall Execution"))
        .is_some());

    let character = event_source
        .apply_mutation(CharacterMutation::SetMartialArtsDots(
            "Single Point Shining Into the Void Style".to_owned(),
            2,
        ))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::MartialArts("Shining Starfall Execution"))
        .is_none());
    let character = event_source.undo().unwrap();
    assert!(character
        .charms()
        .get(CharmName::MartialArts("Shining Starfall Execution"))
        .is_some());

    // Exalts must meet the Essence requirements of charms
    let form = MartialArtsCharmDetails::builder(
        "Single Point Shining Into the Void Form".to_owned(),
        "Single Point Shining Into the Void Style".to_owned(),
    )
    .book_reference(BookReference::new(Book::CoreRulebook, 434))
    .cost(CharmCostType::Motes, NonZeroU8::new(10).unwrap())
    .essence_required(NonZeroU8::new(2).unwrap())
    .ability_required(NonZeroU8::new(4).unwrap())
    .action_type(CharmActionType::Simple)
    .keyword(MartialArtsCharmKeyword::Form)
    .duration("One Scene".to_owned())
    .charm_prerequisite("Gathering Light Concentration".to_owned())
    .charm_prerequisite("Shining Starfall Execution".to_owned())
    .summary("Two actions per round".to_owned())
    .description(
        "Sheathing her blade for a brief moment, the swordsman \
    centers her mind and Essence. As she draws her sword \
    once again and enters this form, it is as a lightning bolt of \
    flashing steel, moving with unimaginable speed and \
    control."
            .to_owned(),
    )
    .mastery("The martial artist gains the following benefits:[...]".to_owned())
    .terrestrial(
        " A Dragon-Blood must pay a point of \
    Willpower each round she wishes to attack twice."
            .to_owned(),
    )
    .build();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::MartialArts(
            form.clone()
        )))
        .is_err());
    event_source
        .apply_mutation(CharacterMutation::SetEssenceRating(2))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::MartialArts(
            form.clone(),
        )))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::MartialArts(
            "Single Point Shining Into the Void Form"
        ))
        .is_some());

    let character = event_source
        .apply_mutation(CharacterMutation::SetEssenceRating(1))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::MartialArts(
            "Single Point Shining Into the Void Form"
        ))
        .is_none());

    // ...unless they are Dawn Solars with Martial Arts Supernal
    event_source
        .apply_mutation(CharacterMutation::SetAbility(
            AbilityNameVanilla::Linguistics,
            1,
        ))
        .unwrap();

    let new_solar = Solar::builder()
        .dawn()
        .caste_ability(DawnCasteAbility::Archery)
        .caste_ability(DawnCasteAbility::Awareness)
        .caste_ability(DawnCasteAbility::Dodge)
        .caste_ability(DawnCasteAbility::Thrown)
        .supernal_ability(DawnSupernalAbility::MartialArts)
        .favored_ability(AbilityName::Medicine)
        .favored_ability(AbilityName::Linguistics)
        .favored_ability(AbilityName::Melee)
        .favored_ability(AbilityName::Resistance)
        .favored_ability(AbilityName::Sail)
        .limit_trigger("A limit trigger".to_owned())
        .build()
        .unwrap();

    let character = event_source
        .apply_mutation(CharacterMutation::SetSolar(new_solar))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::MartialArts("Gathering Light Concentration"))
        .is_some());
    assert!(character
        .charms()
        .get(CharmName::MartialArts("Shining Starfall Execution"))
        .is_some());
    assert!(character
        .charms()
        .get(CharmName::MartialArts(
            "Single Point Shining Into the Void Form"
        ))
        .is_none());

    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::MartialArts(
            form.clone(),
        )))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::MartialArts(
            "Single Point Shining Into the Void Form"
        ))
        .is_some());

    // Exalts must satisfy the Charm tree prerequisites of their Styles
    let character = event_source
        .apply_mutation(CharacterMutation::RemoveCharm(
            CharmNameMutation::MartialArts("Shining Starfall Execution".to_owned()),
        ))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::MartialArts("Shining Starfall Execution"))
        .is_none());
    assert!(character
        .charms()
        .get(CharmName::MartialArts(
            "Single Point Shining Into the Void Form"
        ))
        .is_none());

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::MartialArts(form)))
        .is_err());
}

#[test]
fn test_eclipse_charms() {
    let mut event_source = CharacterEventSource::default();
    // Mortals cannot add Eclipse charms
    let towering_wheat_blessing = Charm::builder("Towering Wheat Blessing".to_owned())
        .spirit()
        .book_reference(BookReference::new(Book::CoreRulebook, 513))
        .cost(CharmCostType::Motes, NonZeroU8::new(10).unwrap())
        .cost(CharmCostType::Willpower, NonZeroU8::new(1).unwrap())
        .essence_required(NonZeroU8::new(1).unwrap())
        .action_type(CharmActionType::Simple)
        .duration("Instant".to_owned())
        .summary("Makes plants grow suddenly".to_owned())
        .description(
            "A field guardian may bid plants to \
    grow far beyond their usual size in an instant."
                .to_owned(),
        )
        .build_eclipse();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Eclipse(
            towering_wheat_blessing.clone()
        )))
        .is_err());

    // Non-Eclipse Solars cannot add Eclipse charms
    [
        AbilityNameVanilla::Medicine,
        AbilityNameVanilla::Linguistics,
        AbilityNameVanilla::Melee,
        AbilityNameVanilla::Resistance,
        AbilityNameVanilla::Sail,
    ]
    .into_iter()
    .for_each(|ability_name| {
        event_source
            .apply_mutation(CharacterMutation::SetAbility(ability_name, 1))
            .unwrap();
    });

    let new_solar = Solar::builder()
        .dawn()
        .caste_ability(DawnCasteAbility::Archery)
        .caste_ability(DawnCasteAbility::Awareness)
        .caste_ability(DawnCasteAbility::Dodge)
        .caste_ability(DawnCasteAbility::Thrown)
        .supernal_ability(DawnSupernalAbility::MartialArts)
        .favored_ability(AbilityName::Medicine)
        .favored_ability(AbilityName::Linguistics)
        .favored_ability(AbilityName::Melee)
        .favored_ability(AbilityName::Resistance)
        .favored_ability(AbilityName::Sail)
        .limit_trigger("A limit trigger".to_owned())
        .build()
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::SetSolar(new_solar))
        .unwrap();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Eclipse(
            towering_wheat_blessing.clone()
        )))
        .is_err());

    // Eclipse Solars can add Eclipse Charms
    let new_solar = Solar::builder()
        .eclipse()
        .caste_ability(EclipseAbility::Bureaucracy)
        .caste_ability(EclipseAbility::Larceny)
        .caste_ability(EclipseAbility::Socialize)
        .caste_ability(EclipseAbility::Occult)
        .supernal_ability(EclipseAbility::Presence)
        .favored_ability(AbilityName::Medicine)
        .favored_ability(AbilityName::Linguistics)
        .favored_ability(AbilityName::Melee)
        .favored_ability(AbilityName::Resistance)
        .favored_ability(AbilityName::Sail)
        .limit_trigger("A limit trigger".to_owned())
        .build()
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::SetSolar(new_solar))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Eclipse(
            towering_wheat_blessing,
        )))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Spirit("Towering Wheat Blessing"))
        .is_some());

    // Eclipse Solars must meet the Essence requirement of Eclipse Charms
    let night_black_carapace = SpiritCharm::builder("Night-Black Carapace".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 528))
        .cost(CharmCostType::Motes, NonZeroU8::new(5).unwrap())
        .cost(CharmCostType::Willpower, NonZeroU8::new(1).unwrap())
        .essence_required(NonZeroU8::new(4).unwrap())
        .action_type(CharmActionType::Simple)
        .keyword(SpiritCharmKeyword::DecisiveOnly)
        .duration("One scene".to_owned())
        .summary("Summons artifact armor that can shatter for AOE damage".to_owned())
        .description(
            " Darkness swirls in from \
    every corner to clothe Alveua in night-black armor with \
    the traits of light artifact armor."
                .to_owned(),
        )
        .build_eclipse();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Eclipse(
            night_black_carapace.clone()
        )))
        .is_err());

    event_source
        .apply_mutation(CharacterMutation::SetEssenceRating(4))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(AddCharm::Eclipse(
            night_black_carapace.clone(),
        )))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Spirit("Night-Black Carapace"))
        .is_some());

    let character = event_source
        .apply_mutation(CharacterMutation::SetEssenceRating(3))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmName::Spirit("Night-Black Carapace"))
        .is_none());
}
