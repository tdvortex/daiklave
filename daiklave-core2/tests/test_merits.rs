use std::collections::HashSet;

use daiklave_core2::{CharacterEventSource, CharacterMutation, artifact::{Artifact, MagicMaterial, wonders::WonderId}, unique_id::UniqueId, weapons::weapon::{ArtifactWeaponId, Weapon, BaseWeaponId, WeaponWeightClass, OptionalWeaponTag}, book_reference::{BookReference, Book}, armor::armor_item::{artifact::ArtifactArmorId, ArmorItem, BaseArmorId, ArmorWeightClass}, hearthstones::{hearthstone::{GeomancyLevel, Hearthstone, HearthstoneCategory}, HearthstoneId}, martial_arts::{MartialArtsStyle, MartialArtsStyleId}, abilities::{AbilityNameVanilla}, sorcery::{SorceryArchetypeId, SorceryArchetype, ShapingRitualId, ShapingRitual, SpellId, Spell, TerrestrialSpell, SorceryArchetypeMerit, SorceryArchetypeMeritId}, charms::{CharmCost, CharmCostType, CharmKeyword}, languages::language::{LanguageMutation, MajorLanguage}, merits::merit::{Merit, StackableMeritId, MeritType, StackableMeritTemplateId}};

#[test]
fn test_merits() {
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character_view().unwrap();
    // Characters have no merits by default
    assert!(character.merits().iter().next().is_none());
    // Add a bunch of merit (and things which read as merits)
    // Artifact weapon
    // Create and add a unique artifact weapon
    let mutation = CharacterMutation::AddArtifact(Artifact::Weapon(
        ArtifactWeaponId(UniqueId::Placeholder(1)),
        Weapon::artifact("Volcano Cutter")
            .base_artifact(
                BaseWeaponId(UniqueId::Placeholder(8)),
                Weapon::base("Grand Daiklave")
                    .book_reference(BookReference::new(Book::CoreRulebook, 597))
                    .weight_class(WeaponWeightClass::Heavy)
                    .two_handed()
                    .lethal()
                    .melee()
                    .tag(OptionalWeaponTag::Balanced)
                    .tag(OptionalWeaponTag::Reaching)
                    .build_artifact(),
            )
            .material(MagicMaterial::RedJade)
            .merit_dots(5)
            .hearthstone_slots(2)
            .lore("Long lore description")
            .powers(
                "A Solar who attunes to Volcano Cutter awakens Grand \
                Eruption at no experience cost. By paying an extra three motes \
                when attuning the grand daiklave, the Solar gains an \
                additional point of Initiative on any successful withering \
                attack made with Volcano Cutter which rolls no 1s.",
            )
            .book_reference(BookReference::new(Book::CoreRulebook, 627))
            .build(),
    ));
    event_source.apply_mutation(mutation).unwrap();


    // Artifact armor
    let mutation = CharacterMutation::AddArtifact(Artifact::Armor(
        ArtifactArmorId(UniqueId::Placeholder(1)),
        ArmorItem::artifact("Brilliant Sentinel")
            .base_artifact(
                BaseArmorId(UniqueId::Placeholder(2)),
                ArmorItem::base("Articulated Plate (Artifact)")
                    .book_reference(BookReference::new(Book::CoreRulebook, 600))
                    .weight_class(ArmorWeightClass::Heavy)
                    .build_artifact(),
            )
            .material(MagicMaterial::Orichalcum)
            .merit_dots(3)
            .hearthstone_slots(2)
            .lore("There was once an enlightened city[...]")
            .powers(
                "When attuned and worn, the armor imposes a -1 penalty \
            to the Join Battle rolls of all enemies who have harmed \
            or wish to harm the object of one of the wearer's Major \
            or Defining Intimacies. This penalty becomes -1 success \
            to all unExalted creatures of darkness.",
            )
            .book_reference(BookReference::new(Book::CoreRulebook, 616))
            .build(),
    ));
    event_source.apply_mutation(mutation).unwrap();

    // Artifact wonder
    let mutation = CharacterMutation::AddArtifact(Artifact::Wonder(
        WonderId(UniqueId::Placeholder(1)),
        Artifact::wonder_builder("Belt of Shadow Walking")
            .book_reference(BookReference::new(Book::CoreRulebook, 602))
            .merit_dots(3)
            .powers("Night-black belts made from leathe from the wings of giant bats[...]")
            .attunement_cost(5)
            .build(),
    ));
    event_source.apply_mutation(mutation).unwrap();

    // Standalone hearthstone
    let mutation =
        CharacterMutation::AddHearthstone(HearthstoneId(UniqueId::Placeholder(1)), Hearthstone::builder("Jewel of the Celestial Mandarin".to_string())
        .book_reference(BookReference::new(Book::CoreRulebook, 611))
        .category(HearthstoneCategory::Sidereal)
        .geomancy_level(GeomancyLevel::Standard)
        .powers(
            "This transparent, faceted square stone glows with a violet\
        light. Any individual bearing it in an attuned hearthstone\
        socket may glimpse the abodes of spirits—the doors of the\
        sanctums where gods dwell become visible to her eyes.\
        Although this hearthstone doesn't grant the power to enter\
        such sanctums, she can speak and her voice will be heard\
        within, booming with celestial authority—any command\
        to come forth is treated as though it aligns with a Minor\
        Intimacy."
                .to_owned(),
        )
        .manseborn()
        .build());
    event_source.apply_mutation(mutation).unwrap();

    // Standalone demense
    let mutation = CharacterMutation::AddDemense(UniqueId::Placeholder(1), "Nowhere special".to_owned(), GeomancyLevel::Standard);
    event_source.apply_mutation(mutation).unwrap();


    // Manse, hearthstone, and demense
    let eye = Hearthstone::builder("Hierophant's Eye".to_string())
        .book_reference(BookReference::new(Book::CoreRulebook, 610))
        .category(HearthstoneCategory::Solar)
        .geomancy_level(GeomancyLevel::Greater)
        .powers(
            "This black octagonal stone blazes with orange-gold light
            along each of its perfect edges. The Hierophant's Eye
            grants one automatic non-Charm success to every shape
            sorcery action while socketed into an attuned artifact."
                .to_owned(),
        )
        .linked()
        .dependent()
        .build();

    let manse = "A shiny mansion".to_owned();
    let demense = "A cool place".to_owned();
    let mutation =
        CharacterMutation::AddManse(manse, demense, HearthstoneId(UniqueId::Placeholder(2)), eye);
    event_source.apply_mutation(mutation).unwrap();

    // Martial arts style
    let mutation = CharacterMutation::SetAbilityDots(AbilityNameVanilla::Brawl, 1);
    event_source.apply_mutation(mutation).unwrap();

    let crane_style = MartialArtsStyle::new(
        Some(BookReference::new(Book::CoreRulebook, 443)),
        "Crane Style".to_owned(),
        "Crane style is a defensive style, emulating the grace of the \
        crane in avoiding the blows of an enemy. Its students learn \
        not just to fight with physical blows, but to empathize \
        with her enemy, speaking or debating with him in an \
        attempt to bring the fight to an end without violence. \
        However, those who mistake the Crane master's restraint \
        for weakness find themselves quickly meeting the ground. \
        When she must, a student of this style can unleash \
        devastating counterattacks, flowing with the force of an \
        enemy's blow so she can strike back in turn. \n\
        Crane Weapons: Crane style practitioners typically dual \
        wield a war fan and hook sword, using the fan for defense \
        while disarming enemies with the sword. Unarmed attacks \
        usually consist of graceful kicks, but a Crane stylist lacking \
        his usual weapons might use one hand to deliver rapid \
        chops while holding back the other for powerful lunges \
        and sweeping blows. \n \
        Armor: Crane style is incompatible with armor. \n \
        Complementary Abilities: Many Crane stylists use \
        Presence, Performance, or Socialize in combat to sway \
        their opponents into peaceful resolution or compromise, \
        and later Charms of this style empower such efforts."
            .to_owned(),
        HashSet::from([
            BaseWeaponId(UniqueId::Placeholder(1)),
            BaseWeaponId(UniqueId::Placeholder(2)),
            BaseWeaponId(UniqueId::Placeholder(3)),
        ]),
        None,
    );

    let mutation = CharacterMutation::AddMartialArtsStyle(MartialArtsStyleId(UniqueId::Placeholder(1)), crane_style);
    event_source.apply_mutation(mutation).unwrap();

    // Exalted Healing
    let mutation = CharacterMutation::AddExaltedHealing;
    event_source.apply_mutation(mutation).unwrap();
    
    // Sorcery
    let mutation = CharacterMutation::SetAbilityDots(AbilityNameVanilla::Occult, 3);
    event_source.apply_mutation(mutation).unwrap();

    let mutation = CharacterMutation::AddTerrestrialSorcery(
        Box::new((
        SorceryArchetypeId(UniqueId::Placeholder(1)),
        SorceryArchetype::new(
            "The Talisman of Ten Thousand Eyes".to_owned(), 
            Some(BookReference::new(Book::CoreRulebook, 470)), 
            "A phylactery of great sorcerous puissance[...]".to_owned()
        ),
        ShapingRitualId(UniqueId::Placeholder(1)),
        ShapingRitual::new(
            SorceryArchetypeId(UniqueId::Placeholder(1)), 
            Some(BookReference::new(Book::CoreRulebook, 470)), 
            "When the sorcerer takes the first shape sorcery action to \
            begin casting a spell and stunts it with a description of \
            how she casts the spell through the talisman or draws on \
            its power, she gains (stunt rating + 2) sorcerous motes \
            towards completing this spell. This benefit can only be \
            received once per scene. Stunts to enhance the sorcerer's \
            control spell do not count against the once per scene limit.".to_owned()
        ),
        SpellId(UniqueId::Placeholder(1)),
        TerrestrialSpell::from_spell(Spell::new(
            "Death of Obsidian Butterflies".to_owned(), 
            Some(BookReference::new(Book::CoreRulebook, 470)), 
            vec![CharmCost::new(CharmCostType::SorcerousMotes, 15), CharmCost::new(CharmCostType::Willpower, 1)], 
            vec![CharmKeyword::DecisiveOnly, CharmKeyword::Perilous], 
            "Instant".to_string(), 
            "Sculpting Essence into volant black glass, the sorcerer unleashes a cascade of obsidian butterflies.".to_owned()
        ))
    )));
    event_source.apply_mutation(mutation).unwrap();

    // A sorcery archetype merit
    let mutation = CharacterMutation::AddSorceryArchetypeMerit(SorceryArchetypeId(UniqueId::Placeholder(1)), SorceryArchetypeMeritId(UniqueId::Placeholder(1)), SorceryArchetypeMerit {
        name: "Astral Meditation".to_owned(),
        book_reference: Some(BookReference::new(Book::CoreRulebook, 470)),
        dots: 1,
        description: " The talisman serves as a \
        gateway through which the sorcerer may send her presence \
        to distant corners of Creation. Once per day, while wearing \
        the talisman, she may waive the Willpower costs of a spell \
        that allows her to sense things remotely or project her \
        presence from afar, such as Silent Words of Dreams and \
        Nightmares".to_owned(),
    });
    event_source.apply_mutation(mutation).unwrap();

    // A major language
    let mutation = CharacterMutation::AddLanguage(LanguageMutation::MajorLanguage(MajorLanguage::HighRealm));
    event_source.apply_mutation(mutation).unwrap();

    // Multiple minor languages
    [
        "Local language",
        "Another local language",
        "A third local language",
        "Local language number 4",
        "Fifth and final local language"
    ].into_iter().map(|s| CharacterMutation::AddLanguage(LanguageMutation::LocalTongue(s.to_owned()))).fold(Ok(&mut event_source), |acc, mutation| acc.and_then(|es| es.apply_mutation(mutation))).unwrap();

    // A stackable merit
    let retainers = Merit::new_template("Retainers".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 164))
        .merit_type(MeritType::Story)
        .variable_dots("Each purchase grants the character a single Storyteller-controlled servant, an expert in their field with noteworthy and useful abilities.".to_owned())
        .dot_option(2, "Two dots grants a mortal or minor supernatural ally, \
            who may possess useful contacts or experience, political clout, \
            martial prowess, or other resources.".to_owned())
        .dot_option(4, "Four dots grants a supernatural ally at least as powerful as a young Terrestrial Exalt".to_owned())
        .stackable(StackableMeritTemplateId(UniqueId::Placeholder(1)))
        .unwrap();

    // Check you can't add a merit with the wrong number of dots
    assert!(Merit::new_stackable(3, "A three-dot retainer".to_owned(), retainers.clone()).is_err());
    
    let mutation = CharacterMutation::AddStackableMerit(StackableMeritId(UniqueId::Placeholder(1)), Merit::new_stackable(2, "An expert bodyguard".to_owned(), retainers).unwrap());
    event_source.apply_mutation(mutation).unwrap();

    // A nonstackable merit without requirements
    // A nonstackable merit with requirements -- errors if not met
    // Succeeds if requirements are met

    // Check that all these merits exist and have the right properties
    // Change the character to be Exalted
    // Exalted Healing should be gone
    // Sorcery should not be a merit
    // Sorcery archetype merit should still exist
    // Remove all of the merits
    // No merits left
}