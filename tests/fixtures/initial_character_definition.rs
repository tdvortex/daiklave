use exalted_3e_gui::{Character, attributes::AttributeName, character::{Willpower, ExperiencePoints}, abilities::AbilityNameNoSubskill, intimacies::{Intimacy, IntimacyLevel, IntimacyType}, health::WoundPenalty, armor::{ArmorItem, ArmorTag}, weapons::{Weapon, RangeBand, WeaponTag, EquipHand}, player::Player};

pub fn create_initial_character(player: &Player) -> Character {
    let mut initial_character = Character::create()
            .with_player(player.clone())
            .with_name("Test Character Name".to_owned())
            .with_concept("A character for testing purposes".to_owned())
            .with_willpower(Willpower {
                current: 5,
                maximum: 6,
            })
            .with_experience(ExperiencePoints {
                current: 15,
                total: 15,
            });

        initial_character = vec![
            (AttributeName::Strength, 4),
            (AttributeName::Dexterity, 4),
            (AttributeName::Stamina, 3),
            (AttributeName::Charisma, 4),
            (AttributeName::Manipulation, 3),
            (AttributeName::Appearance, 2),
            (AttributeName::Intelligence, 3),
            (AttributeName::Wits, 3),
        ]
        .into_iter()
        .fold(initial_character, |ic, (attribute_name, value)| {
            ic.with_attribute(attribute_name, value).unwrap()
        });

        initial_character = vec![
            (AbilityNameNoSubskill::Awareness, 4),
            (AbilityNameNoSubskill::War, 3),
            (AbilityNameNoSubskill::Resistance, 3),
            (AbilityNameNoSubskill::Dodge, 3),
            (AbilityNameNoSubskill::Integrity, 2),
            (AbilityNameNoSubskill::Presence, 2),
            (AbilityNameNoSubskill::Socialize, 2),
            (AbilityNameNoSubskill::Athletics, 2),
            (AbilityNameNoSubskill::Linguistics, 1),
            (AbilityNameNoSubskill::Brawl, 1),
        ]
        .into_iter()
        .fold(initial_character, |ic, (ability_name_no_subskill, dots)| {
            ic.with_ability(ability_name_no_subskill, dots).unwrap()
        });

        initial_character
            .with_craft("Weapon Forging", 1)
            .with_martial_arts("Single Point Shining Into Void Style", 4)
            .with_specialty(AbilityNameNoSubskill::War, "While Outnumbered".to_owned())
            .unwrap()
            .with_specialty(AbilityNameNoSubskill::Socialize, "Tavern Gossip".to_owned())
            .unwrap()
            .with_craft_specialty("Weapon Forging", "Sharpening Blades".to_owned())
            .unwrap()
            .with_martial_arts_specialty(
                "Single Point Shining Into Void Style",
                "Join Battle".to_owned(),
            )
            .unwrap()
            .with_intimacy(Intimacy::new(
                IntimacyLevel::Defining,
                IntimacyType::Principle,
                "Never stand idle against injustice".to_owned(),
                None,
            ))
            .with_intimacy(Intimacy::new(
                IntimacyLevel::Major,
                IntimacyType::Tie,
                "Ragara Tirnis (Love)".to_owned(),
                None,
            ))
            .with_intimacy(Intimacy::new(
                IntimacyLevel::Major,
                IntimacyType::Tie,
                "Mask of Winters (Revenge)".to_owned(),
                None,
            ))
            .with_intimacy(Intimacy::new(
                IntimacyLevel::Minor,
                IntimacyType::Tie,
                "Street Vendors (Camaraderie)".to_owned(),
                None,
            ))
            .with_wound_penalties(vec![
                WoundPenalty::Incapacitated,
                WoundPenalty::MinusFour,
                WoundPenalty::MinusTwo,
                WoundPenalty::MinusTwo,
                WoundPenalty::MinusTwo,
                WoundPenalty::MinusTwo,
                WoundPenalty::MinusOne,
                WoundPenalty::MinusOne,
                WoundPenalty::MinusOne,
                WoundPenalty::Zero,
            ])
            .with_damage(2, 3, 1)
            .with_armor(
                ArmorItem::create_from_book("Core Rulebook".to_owned(), 600)
                    .with_name("Silken Armor".to_owned())
                    .as_light()
                    .as_artifact()
                    .with_tag(ArmorTag::Silent)
                    .with_tag(ArmorTag::Special)
                    .build()
                    .unwrap(),
                false,
            )
            .unwrap()
            .with_armor(
                ArmorItem::create_custom(None)
                    .with_name("Straw Hat".to_owned())
                    .as_light()
                    .build()
                    .unwrap(),
                true,
            )
            .unwrap()
            .with_weapon(
                Weapon::create_from_book("Core Rulebook".to_owned(), 581)
                    .with_name("Knife".to_owned())
                    .as_light()
                    .as_one_handed()
                    .as_melee()
                    .with_thrown_range(RangeBand::Short)
                    .dealing_lethal()
                    .build()
                    .unwrap(),
                None,
            )
            .unwrap()
            .with_weapon(
                Weapon::create_custom(None)
                    .with_name("Screamer (Red Jade Reaper Daiklave)".to_owned())
                    .as_artifact()
                    .as_medium()
                    .as_one_handed()
                    .as_melee()
                    .with_tag(WeaponTag::Balanced)
                    .with_martial_arts("Single Point Shining Into Void Style".to_owned())
                    .dealing_lethal()
                    .build()
                    .unwrap(),
                Some(EquipHand::Main),
            )
            .unwrap()
            .build()
            .unwrap()
}