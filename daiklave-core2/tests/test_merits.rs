use std::num::NonZeroU8;

use daiklave_core2::{
    abilities::{AbilityName, AbilityNameVanilla},
    armor::armor_item::{ArmorItem, ArmorWeightClass},
    artifact::{AddArtifact, ArtifactName, ArtifactNameMutation, MagicMaterial},
    attributes::AttributeName,
    book_reference::{Book, BookReference},
    exaltation::exalt::exalt_type::solar::{
        caste::{DawnCasteAbility, DawnSupernalAbility},
        Solar,
    },
    hearthstones::hearthstone::{GeomancyLevel, Hearthstone, HearthstoneCategory},
    languages::language::{LanguageMutation, MajorLanguage},
    martial_arts::style::MartialArtsStyle,
    merits::merit::{
        Merit, MeritInstanceName, MeritTemplateId, MeritType, NonStackableMerit,
        NonStackableMeritId, StackableMerit, StackableMeritId, StackableMeritTemplateId,
    },
    sorcery::{
        spell::{Spell, SpellKeyword},
        ShapingRitual, Sorcery, SorceryArchetype, SorceryArchetypeMerit, SorceryArchetypeMeritId,
    },
    unique_id::UniqueId,
    weapons::weapon::{OptionalWeaponTag, Weapon, WeaponWeightClass},
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_merits() {
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character().unwrap();
    // Characters have no merits by default
    assert!(character.merits().iter().next().is_none());

    // Add a bunch of merits (and things which read as merits)
    // Artifact weapon
    // Create and add a unique artifact weapon
    let mutation = CharacterMutation::AddArtifact(AddArtifact::Weapon(
        Weapon::artifact("Volcano Cutter".to_owned())
            .base_artifact(
                Weapon::base("Grand Daiklave".to_owned())
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
            .lore("Long lore description".to_owned())
            .powers(
                "A Solar who attunes to Volcano Cutter awakens Grand \
                Eruption at no experience cost. By paying an extra three motes \
                when attuning the grand daiklave, the Solar gains an \
                additional point of Initiative on any successful withering \
                attack made with Volcano Cutter which rolls no 1s."
                    .to_owned(),
            )
            .book_reference(BookReference::new(Book::CoreRulebook, 627))
            .build(),
    ));
    event_source.apply_mutation(mutation).unwrap();

    // Artifact armor
    let mutation = CharacterMutation::AddArtifact(AddArtifact::Armor(
        ArmorItem::artifact_builder("Brilliant Sentinel".to_owned())
            .base_artifact(
                ArmorItem::base_builder("Articulated Plate (Artifact)".to_owned())
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
    let mutation = CharacterMutation::AddArtifact(AddArtifact::Wonder(
        AddArtifact::wonder_builder("Belt of Shadow Walking")
            .book_reference(BookReference::new(Book::CoreRulebook, 602))
            .merit_dots(3)
            .powers("Night-black belts made from leathe from the wings of giant bats[...]")
            .attunement_cost(5)
            .build(),
    ));
    event_source.apply_mutation(mutation).unwrap();

    // Standalone hearthstone
    let mutation = CharacterMutation::AddHearthstone(
        Hearthstone::builder("Jewel of the Celestial Mandarin".to_string())
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
            .build(),
    );
    event_source.apply_mutation(mutation).unwrap();

    // Standalone demense
    let mutation =
        CharacterMutation::AddDemense("Nowhere special".to_owned(), GeomancyLevel::Standard);
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
    let mutation = CharacterMutation::AddManse((manse, demense, eye));
    event_source.apply_mutation(mutation).unwrap();

    // Martial arts style
    let mutation = CharacterMutation::SetAbility(AbilityNameVanilla::Brawl, 1);
    event_source.apply_mutation(mutation).unwrap();

    let (crane_style_name, crane_style) = MartialArtsStyle::with_name("Crane Style".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 443))
        .description(
            "Crane style is a defensive style, emulating the grace of the \
    crane in avoiding the blows of an enemy. Its students learn \
    not just to fight with physical blows, but to empathize \
    with her enemy, speaking or debating with him in an \
    attempt to bring the fight to an end without violence."
                .to_owned(),
        )
        .unarmed()
        .weapon("War Fan".to_owned())
        .weapon("Hook Sword".to_owned())
        .build();

    let mutation = CharacterMutation::AddMartialArtsStyle(crane_style_name, crane_style);
    event_source.apply_mutation(mutation).unwrap();

    // Exalted Healing
    let mutation = CharacterMutation::AddExaltedHealing;
    event_source.apply_mutation(mutation).unwrap();

    // Sorcery
    let mutation = CharacterMutation::SetAbility(AbilityNameVanilla::Occult, 3);
    event_source.apply_mutation(mutation).unwrap();

    let add_sorcery = Sorcery::builder().terrestrial().add_archetype(SorceryArchetype::new("The Talisman of Ten Thousand Eyes".to_owned(), Some(BookReference::new(Book::CoreRulebook, 470)), "A phylactery of great sorcerous puissance[...]".to_owned()))
    .shaping_ritual(ShapingRitual::new("The Talisman of Ten Thousand Eyes".to_owned(), "Gain motes by stunting Shape Sorcery".to_owned(), Some(BookReference::new(Book::CoreRulebook, 470)), "When the sorcerer takes the first shape sorcery action to \
    begin casting a spell and stunts it with a description of \
    how she casts the spell through the talisman or draws on \
    its power, she gains (stunt rating + 2) sorcerous motes \
    towards completing this spell. This benefit can only be \
    received once per scene. Stunts to enhance the sorcerer's \
    control spell do not count against the once per scene limit."
        .to_owned())).unwrap()
        .control_spell(Spell::with_name("Death of Obsidian Butterflies".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 470))
        .keyword(SpellKeyword::DecisiveOnly)
        .keyword(SpellKeyword::Perilous)
        .sorcerous_motes(NonZeroU8::new(15).unwrap())
        .willpower(NonZeroU8::new(1).unwrap())
        .duration("Instant".to_owned())
        .description("Sculpting Essence into volant black glass, the sorcerer unleashes a cascade of obsidian butterflies[...]".to_owned())
        .control_spell_description("A sorcerer who knows Death of Obsidian Butterflies as her control spell gains (Essence) bonus dice to the spells attack roll[...]".to_owned())
        .summary("AOE attack that makes difficult terrain".to_owned())
        .terrestrial());

    let mutation = CharacterMutation::AddSorcery(Box::new(add_sorcery));
    event_source.apply_mutation(mutation).unwrap();

    // A sorcery archetype merit
    let mutation = CharacterMutation::AddSorceryArchetypeMerit(
        "The Talisman of Ten Thousand Eyes".to_owned(),
        SorceryArchetypeMeritId(UniqueId::Placeholder(1)),
        SorceryArchetypeMerit::new(
            "Astral Meditation".to_owned(),
            Some(BookReference::new(Book::CoreRulebook, 470)),
            1,
            "The talisman serves as a \
        gateway through which the sorcerer may send her presence \
        to distant corners of Creation. Once per day, while wearing \
        the talisman, she may waive the Willpower costs of a spell \
        that allows her to sense things remotely or project her \
        presence from afar, such as Silent Words of Dreams and \
        Nightmares"
                .to_owned(),
        ),
    );
    event_source.apply_mutation(mutation).unwrap();

    // A major language
    let mutation =
        CharacterMutation::AddLanguage(LanguageMutation::MajorLanguage(MajorLanguage::HighRealm));
    event_source.apply_mutation(mutation).unwrap();

    // Multiple minor languages
    [
        "Local language",
        "Another local language",
        "A third local language",
        "Local language number 4",
        "Fifth and final local language",
    ]
    .into_iter()
    .map(|s| CharacterMutation::AddLanguage(LanguageMutation::LocalTongue(s.to_owned())))
    .for_each(|mutation| {
        event_source.apply_mutation(mutation).unwrap();
    });

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
    assert!(StackableMerit::new(retainers.clone(), 3, "A three-dot retainer".to_owned()).is_err());

    let mutation = CharacterMutation::AddStackableMerit(
        StackableMeritId(UniqueId::Placeholder(1)),
        StackableMerit::new(retainers, 2, "An expert bodyguard".to_owned()).unwrap(),
    );
    event_source.apply_mutation(mutation).unwrap();

    // A nonstackable merit without requirements
    let eidetic_memory = Merit::new_template("Eidetic Memory".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 161))
        .merit_type(MeritType::Innate)
        .fixed_dots(
            2,
            "The character enjoys near-perfect recall, adding one automatic success \
        to all attempts to remember details from previous scenes and events."
                .to_owned(),
        )
        .nonstackable(NonStackableMeritId(UniqueId::Placeholder(1)))
        .unwrap();

    let (merit_id, merit) = NonStackableMerit::new(eidetic_memory, 2).unwrap();
    let mutation = CharacterMutation::AddNonStackableMerit(merit_id, merit);
    event_source.apply_mutation(mutation).unwrap();

    // A nonstackable merit with requirements -- errors if not met
    let iron_stomach = Merit::new_template("Iron Stomach".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 162))
        .merit_type(MeritType::Purchased)
        .ability_prerequisite(AbilityName::Resistance, NonZeroU8::new(3).unwrap())
        .attribute_prerequisite(AttributeName::Stamina, NonZeroU8::new(3).unwrap())
        .fixed_dots(
            1,
            "The character can digest almost anything edible, from live
    insects and brackish water to raw meat or spoiled rations.
    She enjoys a -2 difficulty on Survival rolls to forage for
    herself, as well as Resistance rolls to recover from food
    poisoning."
                .to_owned(),
        )
        .nonstackable(NonStackableMeritId(UniqueId::Placeholder(2)))
        .unwrap();

    let (merit_id, merit) = NonStackableMerit::new(iron_stomach, 1).unwrap();
    let mutation = CharacterMutation::AddNonStackableMerit(merit_id, merit.clone());
    assert!(event_source.apply_mutation(mutation).is_err());

    // Succeeds if requirements are met
    let mutation = CharacterMutation::SetAttribute(AttributeName::Stamina, 3);
    event_source.apply_mutation(mutation).unwrap();
    let mutation = CharacterMutation::AddNonStackableMerit(merit_id, merit.clone());
    event_source.apply_mutation(mutation).unwrap();

    // Check that all these merits exist and have the right properties
    let character = event_source.as_character().unwrap();
    let merits = character.merits();
    let volcano_cutter = merits
        .get(MeritInstanceName::Artifact(ArtifactName::Weapon(
            "Volcano Cutter",
        )))
        .unwrap();
    assert_eq!(
        volcano_cutter.name(),
        MeritInstanceName::Artifact(ArtifactName::Weapon("Volcano Cutter"))
    );
    assert_eq!(volcano_cutter.template_name(), "Artifact");
    assert_eq!(
        volcano_cutter.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 159))
    );
    assert_eq!(volcano_cutter.detail().unwrap(), "Volcano Cutter");
    assert_eq!(volcano_cutter.dots(), 5);
    assert_eq!(volcano_cutter.merit_type(), MeritType::Story);
    assert_eq!(volcano_cutter.template_id(), MeritTemplateId::Artifact);
    assert!(volcano_cutter.description().1.is_some());

    let brilliant_sentinel = merits
        .get(MeritInstanceName::Artifact(ArtifactName::Armor(
            "Brilliant Sentinel",
        )))
        .unwrap();
    assert_eq!(
        brilliant_sentinel.name(),
        MeritInstanceName::Artifact(ArtifactName::Armor("Brilliant Sentinel"))
    );
    assert_eq!(brilliant_sentinel.template_name(), "Artifact");
    assert_eq!(
        brilliant_sentinel.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 159))
    );
    assert_eq!(brilliant_sentinel.detail().unwrap(), "Brilliant Sentinel");
    assert_eq!(brilliant_sentinel.dots(), 3);
    assert_eq!(brilliant_sentinel.merit_type(), MeritType::Story);
    assert_eq!(brilliant_sentinel.template_id(), MeritTemplateId::Artifact);
    assert!(brilliant_sentinel.description().1.is_some());

    let belt = merits
        .get(MeritInstanceName::Artifact(ArtifactName::Wonder(
            "Belt of Shadow Walking",
        )))
        .unwrap();
    assert_eq!(
        belt.name(),
        MeritInstanceName::Artifact(ArtifactName::Wonder("Belt of Shadow Walking"))
    );
    assert_eq!(belt.template_name(), "Artifact");
    assert_eq!(
        belt.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 159))
    );
    assert_eq!(belt.detail().unwrap(), "Belt of Shadow Walking");
    assert_eq!(belt.dots(), 3);
    assert_eq!(belt.merit_type(), MeritType::Story);
    assert_eq!(belt.template_id(), MeritTemplateId::Artifact);
    assert!(belt.description().1.is_some());

    let jewel = merits
        .get(MeritInstanceName::HearthstoneNoManse(
            "Jewel of the Celestial Mandarin",
        ))
        .unwrap();
    assert_eq!(
        jewel.name(),
        MeritInstanceName::HearthstoneNoManse("Jewel of the Celestial Mandarin")
    );
    assert_eq!(jewel.template_name(), "Hearthstone");
    assert_eq!(
        jewel.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 161))
    );
    assert_eq!(jewel.detail().unwrap(), "Jewel of the Celestial Mandarin");
    assert_eq!(jewel.dots(), 2);
    assert_eq!(jewel.merit_type(), MeritType::Story);
    assert_eq!(jewel.template_id(), MeritTemplateId::Hearthstone);
    assert!(jewel.description().1.is_some());

    let nowhere = merits
        .get(MeritInstanceName::Demense("Nowhere special"))
        .unwrap();
    assert_eq!(
        nowhere.name(),
        MeritInstanceName::Demense("Nowhere special")
    );
    assert_eq!(nowhere.template_name(), "Demense");
    assert_eq!(
        nowhere.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 160))
    );
    assert_eq!(nowhere.detail().unwrap(), "Nowhere special");
    assert_eq!(nowhere.dots(), 2);
    assert_eq!(nowhere.merit_type(), MeritType::Story);
    assert_eq!(nowhere.template_id(), MeritTemplateId::Demense);
    assert!(nowhere.description().1.is_some());

    let eye = merits
        .get(MeritInstanceName::HearthstoneWithManse("Hierophant's Eye"))
        .unwrap();
    assert_eq!(
        eye.name(),
        MeritInstanceName::HearthstoneWithManse("Hierophant's Eye")
    );
    assert_eq!(eye.template_name(), "Hearthstone");
    assert_eq!(
        eye.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 161))
    );
    assert_eq!(eye.detail().unwrap(), "Hierophant's Eye");
    assert_eq!(eye.dots(), 0);
    assert_eq!(eye.merit_type(), MeritType::Story);
    assert_eq!(eye.template_id(), MeritTemplateId::Hearthstone);
    assert!(eye.description().1.is_some());

    let manse = merits
        .get(MeritInstanceName::Manse("Hierophant's Eye"))
        .unwrap();
    assert_eq!(manse.name(), MeritInstanceName::Manse("Hierophant's Eye"));
    assert_eq!(manse.template_name(), "Manse");
    assert_eq!(
        manse.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 163))
    );
    assert_eq!(manse.detail().unwrap(), "A shiny mansion");
    assert_eq!(manse.dots(), 5);
    assert_eq!(manse.merit_type(), MeritType::Story);
    assert_eq!(manse.template_id(), MeritTemplateId::Manse);
    assert!(manse.description().1.is_some());

    let ma_crane = merits
        .get(MeritInstanceName::MartialArtist("Crane Style"))
        .unwrap();
    assert_eq!(
        ma_crane.name(),
        MeritInstanceName::MartialArtist("Crane Style")
    );
    assert_eq!(ma_crane.template_name(), "Martial Artist");
    assert_eq!(
        ma_crane.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 163))
    );
    assert_eq!(ma_crane.detail().unwrap(), "Crane Style");
    assert_eq!(ma_crane.dots(), 4);
    assert_eq!(ma_crane.merit_type(), MeritType::Purchased);
    assert_eq!(ma_crane.template_id(), MeritTemplateId::MartialArtist);
    assert!(ma_crane.description().1.is_none());

    let exalted_healing = merits.get(MeritInstanceName::ExaltedHealing).unwrap();
    assert_eq!(exalted_healing.name(), MeritInstanceName::ExaltedHealing);
    assert_eq!(exalted_healing.template_name(), "Exalted Healing");
    assert_eq!(
        exalted_healing.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 165))
    );
    assert!(exalted_healing.detail().is_none());
    assert_eq!(exalted_healing.dots(), 5);
    assert_eq!(exalted_healing.merit_type(), MeritType::Supernatural);
    assert_eq!(
        exalted_healing.template_id(),
        MeritTemplateId::ExaltedHealing
    );
    assert!(exalted_healing.description().1.is_none());

    let mortal_sorcerer = merits.get(MeritInstanceName::MortalSorcerer).unwrap();
    assert_eq!(mortal_sorcerer.name(), MeritInstanceName::MortalSorcerer);
    assert_eq!(
        mortal_sorcerer.template_name(),
        "Terrestrial Circle Sorcerer (Mortal)"
    );
    assert_eq!(
        mortal_sorcerer.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 470))
    );
    assert!(mortal_sorcerer.detail().is_none());
    assert_eq!(mortal_sorcerer.dots(), 5);
    assert_eq!(mortal_sorcerer.merit_type(), MeritType::Story);
    assert_eq!(
        mortal_sorcerer.template_id(),
        MeritTemplateId::MortalSorcerer
    );
    assert!(mortal_sorcerer.description().1.is_none());

    let astral_meditation = merits
        .get(MeritInstanceName::SorceryArchetype(
            SorceryArchetypeMeritId(UniqueId::Placeholder(1)),
        ))
        .unwrap();
    assert_eq!(
        astral_meditation.name(),
        MeritInstanceName::SorceryArchetype(SorceryArchetypeMeritId(UniqueId::Placeholder(1)))
    );
    assert_eq!(astral_meditation.template_name(), "Astral Meditation");
    assert_eq!(
        astral_meditation.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 470))
    );
    assert!(astral_meditation.detail().is_none());
    assert_eq!(astral_meditation.dots(), 1);
    assert_eq!(astral_meditation.merit_type(), MeritType::Story);
    assert_eq!(
        astral_meditation.template_id(),
        MeritTemplateId::SorceryArchetype(SorceryArchetypeMeritId(UniqueId::Placeholder(1)))
    );
    assert!(astral_meditation.description().1.is_none());

    let high_realm = merits
        .get(MeritInstanceName::MajorLanguage(MajorLanguage::HighRealm))
        .unwrap();
    assert_eq!(
        high_realm.name(),
        MeritInstanceName::MajorLanguage(MajorLanguage::HighRealm)
    );
    assert_eq!(high_realm.template_name(), "Language");
    assert_eq!(
        high_realm.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 162))
    );
    assert_eq!(high_realm.detail().unwrap(), "High Realm");
    assert_eq!(high_realm.dots(), 1);
    assert_eq!(high_realm.merit_type(), MeritType::Purchased);
    assert_eq!(high_realm.template_id(), MeritTemplateId::Language);
    assert!(high_realm.description().1.is_some());

    let other_languages = merits.get(MeritInstanceName::LocalTongues).unwrap();
    assert_eq!(other_languages.name(), MeritInstanceName::LocalTongues);
    assert_eq!(other_languages.template_name(), "Language");
    assert_eq!(
        other_languages.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 162))
    );
    assert_eq!(other_languages.detail().unwrap(), "Local Tongues");
    assert_eq!(other_languages.dots(), 2);
    assert_eq!(other_languages.merit_type(), MeritType::Purchased);
    assert_eq!(other_languages.template_id(), MeritTemplateId::Language);
    assert!(other_languages.description().1.is_some());

    let retainers = merits
        .get(MeritInstanceName::Stackable(StackableMeritId(
            UniqueId::Placeholder(1),
        )))
        .unwrap();
    assert_eq!(
        retainers.name(),
        MeritInstanceName::Stackable(StackableMeritId(UniqueId::Placeholder(1)))
    );
    assert_eq!(retainers.template_name(), "Retainers");
    assert_eq!(
        retainers.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 164))
    );
    assert_eq!(retainers.detail().unwrap(), "An expert bodyguard");
    assert_eq!(retainers.dots(), 2);
    assert_eq!(retainers.merit_type(), MeritType::Story);
    assert_eq!(
        retainers.template_id(),
        MeritTemplateId::Stackable(StackableMeritTemplateId(UniqueId::Placeholder(1)))
    );
    assert!(retainers.description().1.is_some());

    let eidetic_memory = merits
        .get(MeritInstanceName::NonStackable(NonStackableMeritId(
            UniqueId::Placeholder(1),
        )))
        .unwrap();
    assert_eq!(
        eidetic_memory.name(),
        MeritInstanceName::NonStackable(NonStackableMeritId(UniqueId::Placeholder(1)))
    );
    assert_eq!(eidetic_memory.template_name(), "Eidetic Memory");
    assert_eq!(
        eidetic_memory.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 161))
    );
    assert!(eidetic_memory.detail().is_none());
    assert_eq!(eidetic_memory.dots(), 2);
    assert_eq!(eidetic_memory.merit_type(), MeritType::Innate);
    assert_eq!(
        eidetic_memory.template_id(),
        MeritTemplateId::NonStackable(NonStackableMeritId(UniqueId::Placeholder(1)))
    );
    assert!(eidetic_memory.description().1.is_none());

    let iron_stomach = merits
        .get(MeritInstanceName::NonStackable(NonStackableMeritId(
            UniqueId::Placeholder(2),
        )))
        .unwrap();
    assert_eq!(
        iron_stomach.name(),
        MeritInstanceName::NonStackable(NonStackableMeritId(UniqueId::Placeholder(2)))
    );
    assert_eq!(iron_stomach.template_name(), "Iron Stomach");
    assert_eq!(
        iron_stomach.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 162))
    );
    assert!(iron_stomach.detail().is_none());
    assert_eq!(iron_stomach.dots(), 1);
    assert_eq!(iron_stomach.merit_type(), MeritType::Purchased);
    assert_eq!(
        iron_stomach.template_id(),
        MeritTemplateId::NonStackable(NonStackableMeritId(UniqueId::Placeholder(2)))
    );
    assert!(iron_stomach.description().1.is_none());

    // Change the character to be Exalted
    let new_solar = Solar::builder()
        .dawn()
        .caste_ability(DawnCasteAbility::Dodge)
        .caste_ability(DawnCasteAbility::Resistance)
        .caste_ability(DawnCasteAbility::Awareness)
        .caste_ability(DawnCasteAbility::War)
        .supernal_ability(DawnSupernalAbility::MartialArts)
        .favored_ability(AbilityName::Presence)
        .favored_ability(AbilityName::Socialize)
        .favored_ability(AbilityName::Linguistics)
        .favored_ability(AbilityName::Medicine)
        .favored_ability(AbilityName::Performance)
        .limit_trigger("Fleeing from a battle not yet lost".to_owned())
        .build()
        .unwrap();
    let mutation = CharacterMutation::SetSolar(new_solar);
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character().unwrap();
    let merits = character.merits();

    // Exalted Healing should be free
    let exalted_healing = merits.get(MeritInstanceName::ExaltedHealing).unwrap();
    assert_eq!(exalted_healing.name(), MeritInstanceName::ExaltedHealing);
    assert_eq!(exalted_healing.template_name(), "Exalted Healing");
    assert_eq!(
        exalted_healing.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 165))
    );
    assert!(exalted_healing.detail().is_none());
    assert_eq!(exalted_healing.dots(), 0);
    assert_eq!(exalted_healing.merit_type(), MeritType::Supernatural);
    assert_eq!(
        exalted_healing.template_id(),
        MeritTemplateId::ExaltedHealing
    );
    assert!(exalted_healing.description().1.is_none());

    // Sorcery should not be a merit, but should exist
    assert!(merits.get(MeritInstanceName::MortalSorcerer).is_none());
    assert!(character.sorcery().is_some());

    // Sorcery archetype merit should still exist
    assert!(merits
        .get(MeritInstanceName::SorceryArchetype(
            SorceryArchetypeMeritId(UniqueId::Placeholder(1))
        ))
        .is_some());

    // Dropping Occult below 3 removes sorcery and sorcery merits
    let mutation = CharacterMutation::SetAbility(AbilityNameVanilla::Occult, 2);
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character().unwrap();
    assert!(character.sorcery().is_none());
    let merits = character.merits();
    assert!(merits
        .get(MeritInstanceName::SorceryArchetype(
            SorceryArchetypeMeritId(UniqueId::Placeholder(1))
        ))
        .is_none());

    // Dropping Brawl to 0 removes Martial Arts and Martial Artist merit
    let mutation = CharacterMutation::SetAbility(AbilityNameVanilla::Brawl, 0);
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character().unwrap();
    let merits = character.merits();
    assert!(merits
        .get(MeritInstanceName::MartialArtist("Crane Style"))
        .is_none());

    // Dropping an ability or attribute removes dependent merits
    let mutation = CharacterMutation::SetAttribute(AttributeName::Stamina, 2);
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character().unwrap();
    let merits = character.merits();
    assert!(merits
        .get(MeritInstanceName::NonStackable(NonStackableMeritId(
            UniqueId::Placeholder(2)
        )))
        .is_none());

    // Remove the artifacts
    [
        ArtifactNameMutation::Weapon("Volcano Cutter".to_owned()),
        ArtifactNameMutation::Armor("Brilliant Sentinel".to_owned()),
        ArtifactNameMutation::Wonder("Belt of Shadow Walking".to_owned()),
    ]
    .into_iter()
    .map(|artifact_id| CharacterMutation::RemoveArtifact(artifact_id))
    .for_each(|mutation| {
        event_source.apply_mutation(mutation).unwrap();
    });

    // Remove the hearthstone and the manse
    ["Jewel of the Celestial Mandarin", "Hierophant's Eye"]
        .into_iter()
        .map(|hearthstone_name| CharacterMutation::RemoveHearthstone(hearthstone_name.to_owned()))
        .for_each(|mutation| {
            event_source.apply_mutation(mutation).unwrap();
        });

    // Remove the demense
    event_source
        .apply_mutation(CharacterMutation::RemoveDemense(
            "Nowhere special".to_owned(),
        ))
        .unwrap();

    // Remove languages
    [
        "Local language",
        "Another local language",
        "A third local language",
        "Local language number 4",
        "Fifth and final local language",
    ]
    .into_iter()
    .map(|s| LanguageMutation::LocalTongue(s.to_owned()))
    .chain(std::iter::once(LanguageMutation::MajorLanguage(
        MajorLanguage::HighRealm,
    )))
    .map(|language_mutation| CharacterMutation::RemoveLanguage(language_mutation))
    .for_each(|mutation| {
        event_source.apply_mutation(mutation).unwrap();
    });

    // Remove the stackable and nonstackable merits
    let mutation =
        CharacterMutation::RemoveStackableMerit(StackableMeritId(UniqueId::Placeholder(1)));
    event_source.apply_mutation(mutation).unwrap();

    let mutation =
        CharacterMutation::RemoveNonStackableMerit(NonStackableMeritId(UniqueId::Placeholder(1)));
    event_source.apply_mutation(mutation).unwrap();

    let character = event_source.as_character().unwrap();
    let merits = character.merits();
    assert!(merits
        .get(MeritInstanceName::NonStackable(NonStackableMeritId(
            UniqueId::Placeholder(1)
        )))
        .is_none());
    assert!(merits
        .get(MeritInstanceName::Stackable(StackableMeritId(
            UniqueId::Placeholder(1)
        )))
        .is_none());

    // Only merit left should be Exalted Healing
    assert!(merits.get(MeritInstanceName::ExaltedHealing).is_some());

    // Change character back to be mortal
    event_source
        .apply_mutation(CharacterMutation::SetMortal)
        .unwrap();
    let character = event_source.as_character().unwrap();
    let merits = character.merits();

    // No merits left
    assert!(merits.iter().next().is_none());
}
