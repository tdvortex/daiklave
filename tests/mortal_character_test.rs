use std::collections::HashSet;

use exalted_3e_gui::{
    character::traits::range_bands::RangeBand,
    character::traits::abilities::AbilityName,
    character::traits::armor::{ArmorItem, ArmorTag},
    character::traits::attributes::AttributeName,
    character::traits::merits::{Merit, MeritType},
    character::traits::weapons::{EquipHand, Weapon, WeaponTag},
    MortalCharacter,
};

fn default_mortal_character() -> MortalCharacter {
    MortalCharacter::default()
}

fn custom_mortal_character() -> MortalCharacter {
    let mut mortal = MortalCharacter::default();

    [
        (AttributeName::Appearance, 4),
        (AttributeName::Charisma, 2),
        (AttributeName::Dexterity, 3),
        (AttributeName::Intelligence, 2),
        (AttributeName::Manipulation, 5),
        (AttributeName::Perception, 3),
        (AttributeName::Stamina, 2),
        (AttributeName::Strength, 2),
        (AttributeName::Wits, 4),
    ]
    .into_iter()
    .for_each(|(attribute_name, rating)| {
        mortal
            .attributes
            .get_mut(&attribute_name)
            .set_value(rating)
            .unwrap()
    });

    mortal.abilities.add_martial_arts("Crane Style".to_owned());

    [
        (AbilityName::Athletics, 3),
        (AbilityName::Brawl, 1),
        (AbilityName::Bureaucracy, 2),
        (AbilityName::Integrity, 3),
        (AbilityName::Investigation, 2),
        (AbilityName::Larcency, 2),
        (AbilityName::Linguistics, 3),
        (AbilityName::Lore, 2),
        (AbilityName::MartialArts("Crane Style".to_owned()), 4),
        (AbilityName::Occult, 2),
        (AbilityName::Presence, 5),
        (AbilityName::Resistance, 2),
        (AbilityName::Socialize, 5),
        (AbilityName::Thrown, 1),
    ]
    .into_iter()
    .for_each(|(ability_name, rating)| {
        mortal
            .abilities
            .get_mut(&ability_name)
            .unwrap()
            .set_dots(rating);
    });

    [
        (AbilityName::Bureaucracy, "Realm Bureaucratic Processes"),
        (AbilityName::Bureaucracy, "Management"),
        (AbilityName::Socialize, "Formal Situations"),
        (AbilityName::Presence, "Asserting Authority"),
        (AbilityName::Presence, "Convincing Authorities"),
    ]
    .into_iter()
    .for_each(|(ability_name, specialty)| {
        mortal
            .abilities
            .get_mut(&ability_name)
            .unwrap()
            .add_specialty(specialty.to_owned())
            .unwrap();
    });

    [
        (
            "Martial Artist",
            4,
            MeritType::Purchased,
            "The character has undergone near-perfect training[...]",
            Some("Crane Style"),
        ),
        (
            "Eidetic Memory",
            2,
            MeritType::Innate,
            "The character enjoys near-perfect recall[...]",
            None,
        ),
        (
            "Language",
            1,
            MeritType::Purchased,
            "Each purchase grants the character fluency in one language[...]",
            Some("Low Realm"),
        ),
        (
            "Language",
            1,
            MeritType::Purchased,
            "Each purchase grants the character fluency in one language[...]",
            Some("Dragontongue"),
        ),
        (
            "Natural Immunity",
            4,
            MeritType::Innate,
            "Whether naturally hardy or blessed by a spirit[...]",
            None,
        ),
        (
            "Resources",
            1,
            MeritType::Story,
            "This Merit describes a character's finances[...]",
            Some("Remaining Savings"),
        ),
    ]
    .into_iter()
    .for_each(|(name, dots, merit_type, description, maybe_detail)| {
        mortal.merits.insert(Merit::new(
            name.to_owned(),
            dots,
            merit_type,
            description.to_owned(),
            maybe_detail.map(|detail| detail.to_owned()),
        ));
    });

    let key = mortal.weapons.add_weapon(
        Weapon::new(
            "Hook Swords".to_owned(),
            [
                WeaponTag::Medium,
                WeaponTag::Disarming,
                WeaponTag::Lethal,
                WeaponTag::MartialArts("Crane Style".to_owned()),
                WeaponTag::OneHanded,
            ]
            .into(),
        )
        .unwrap(),
    );

    mortal.weapons.equip(key, EquipHand::Both).unwrap();

    let key = mortal.armor.add_armor_item(
        ArmorItem::new(
            "Silken Armor".to_owned(),
            [
                ArmorTag::Light,
                ArmorTag::Artifact,
                ArmorTag::Silent,
                ArmorTag::Special("Doesn't count as armor for Martial Arts".to_owned()),
            ]
            .into(),
        )
        .unwrap(),
    );

    mortal.armor.equip_armor_item(key).unwrap();

    mortal
}

#[test]
fn test_build_default() {
    let _mortal = default_mortal_character();
}

#[test]
fn test_build_custom() {
    let _mortal = custom_mortal_character();
}

#[test]
fn test_default_abilities() {
    let mortal = default_mortal_character();

    let actual: Vec<(AbilityName, u8, Option<Vec<String>>)> = mortal
        .abilities
        .iter()
        .map(|ability| {
            (
                ability.name().clone(),
                ability.dots().clone(),
                ability
                    .specialties()
                    .map(|s| s.iter().map(|s| s.clone()).collect()),
            )
        })
        .collect();

    let expected: Vec<(AbilityName, u8, Option<Vec<String>>)> = vec![
        (AbilityName::Archery, 0, None),
        (AbilityName::Athletics, 0, None),
        (AbilityName::Awareness, 0, None),
        (AbilityName::Brawl, 0, None),
        (AbilityName::Bureaucracy, 0, None),
        (AbilityName::Dodge, 0, None),
        (AbilityName::Integrity, 0, None),
        (AbilityName::Investigation, 0, None),
        (AbilityName::Larcency, 0, None),
        (AbilityName::Linguistics, 0, None),
        (AbilityName::Lore, 0, None),
        (AbilityName::Medicine, 0, None),
        (AbilityName::Melee, 0, None),
        (AbilityName::Occult, 0, None),
        (AbilityName::Performance, 0, None),
        (AbilityName::Presence, 0, None),
        (AbilityName::Resistance, 0, None),
        (AbilityName::Ride, 0, None),
        (AbilityName::Sail, 0, None),
        (AbilityName::Socialize, 0, None),
        (AbilityName::Stealth, 0, None),
        (AbilityName::Survival, 0, None),
        (AbilityName::Thrown, 0, None),
        (AbilityName::War, 0, None),
    ];

    for (act, exp) in actual.into_iter().zip(expected.into_iter()) {
        assert_eq!(act, exp)
    }
}

#[test]
fn test_custom_abilities() {
    let mortal = custom_mortal_character();

    let actual: Vec<(AbilityName, u8, Option<HashSet<String>>)> = mortal
        .abilities
        .iter()
        .map(|ability| {
            (
                ability.name().clone(),
                ability.dots().clone(),
                ability
                    .specialties()
                    .map(|s| s.iter().map(|s| s.clone()).collect()),
            )
        })
        .collect();

    let expected: Vec<(AbilityName, u8, Option<HashSet<String>>)> = vec![
        (AbilityName::Archery, 0, None),
        (AbilityName::Athletics, 3, None),
        (AbilityName::Awareness, 0, None),
        (AbilityName::Brawl, 1, None),
        (
            AbilityName::Bureaucracy,
            2,
            Some(
                [
                    "Realm Bureaucratic Processes".to_owned(),
                    "Management".to_owned(),
                ]
                .into_iter()
                .collect(),
            ),
        ),
        (AbilityName::Dodge, 0, None),
        (AbilityName::Integrity, 3, None),
        (AbilityName::Investigation, 2, None),
        (AbilityName::Larcency, 2, None),
        (AbilityName::Linguistics, 3, None),
        (AbilityName::Lore, 2, None),
        (AbilityName::Medicine, 0, None),
        (AbilityName::Melee, 0, None),
        (AbilityName::Occult, 2, None),
        (AbilityName::Performance, 0, None),
        (
            AbilityName::Presence,
            5,
            Some(
                [
                    "Asserting Authority".to_owned(),
                    "Convincing Authorities".to_owned(),
                ]
                .into_iter()
                .collect(),
            ),
        ),
        (AbilityName::Resistance, 2, None),
        (AbilityName::Ride, 0, None),
        (AbilityName::Sail, 0, None),
        (
            AbilityName::Socialize,
            5,
            Some(["Formal Situations".to_owned()].into_iter().collect()),
        ),
        (AbilityName::Stealth, 0, None),
        (AbilityName::Survival, 0, None),
        (AbilityName::Thrown, 1, None),
        (AbilityName::War, 0, None),
        (AbilityName::MartialArts("Crane Style".to_owned()), 4, None),
    ];

    for (act, exp) in actual.into_iter().zip(expected.into_iter()) {
        assert_eq!(act, exp)
    }
}

#[test]
fn test_default_attributes() {
    let mortal = default_mortal_character();

    let actual: Vec<(AttributeName, u8)> = mortal
        .attributes
        .iter()
        .map(|attribute| (attribute.name(), attribute.dots()))
        .collect();

    let expected: Vec<(AttributeName, u8)> = vec![
        (AttributeName::Strength, 1),
        (AttributeName::Dexterity, 1),
        (AttributeName::Stamina, 1),
        (AttributeName::Charisma, 1),
        (AttributeName::Manipulation, 1),
        (AttributeName::Appearance, 1),
        (AttributeName::Perception, 1),
        (AttributeName::Intelligence, 1),
        (AttributeName::Wits, 1),
    ];

    for (act, exp) in actual.into_iter().zip(expected.into_iter()) {
        assert_eq!(act, exp)
    }
}

#[test]
fn test_custom_attributes() {
    let mortal = custom_mortal_character();

    let actual: Vec<(AttributeName, u8)> = mortal
        .attributes
        .iter()
        .map(|attribute| (attribute.name(), attribute.dots()))
        .collect();

    let expected: Vec<(AttributeName, u8)> = vec![
        (AttributeName::Strength, 2),
        (AttributeName::Dexterity, 3),
        (AttributeName::Stamina, 2),
        (AttributeName::Charisma, 2),
        (AttributeName::Manipulation, 5),
        (AttributeName::Appearance, 4),
        (AttributeName::Perception, 3),
        (AttributeName::Intelligence, 2),
        (AttributeName::Wits, 4),
    ];

    for (act, exp) in actual.into_iter().zip(expected.into_iter()) {
        assert_eq!(act, exp)
    }
}

#[test]
fn test_default_merits() {
    let mortal = default_mortal_character();

    let actual: HashSet<(String, u8, MeritType, String)> = mortal
        .merits
        .iter()
        .map(|merit| {
            (
                format!("{}", merit),
                merit.dots,
                merit.merit_type.clone(),
                merit.description.clone(),
            )
        })
        .collect();

    let expected: HashSet<(String, u8, MeritType, String)> = HashSet::new();

    assert_eq!(actual, expected);
}

#[test]
fn test_custom_merits() {
    let mortal = custom_mortal_character();

    let actual: HashSet<(String, u8, MeritType, String)> = mortal
        .merits
        .iter()
        .map(|merit| {
            (
                format!("{}", merit),
                merit.dots,
                merit.merit_type.clone(),
                merit.description.clone(),
            )
        })
        .collect();

    let expected: HashSet<(String, u8, MeritType, String)> = [
        (
            "Martial Artist (Crane Style) (••••)".to_owned(),
            4,
            MeritType::Purchased,
            "The character has undergone near-perfect training[...]".to_owned(),
        ),
        (
            "Natural Immunity (••••)".to_owned(),
            4,
            MeritType::Innate,
            "Whether naturally hardy or blessed by a spirit[...]".to_owned(),
        ),
        (
            "Resources (Remaining Savings) (•)".to_owned(),
            1,
            MeritType::Story,
            "This Merit describes a character's finances[...]".to_owned(),
        ),
        (
            "Language (Dragontongue) (•)".to_owned(),
            1,
            MeritType::Purchased,
            "Each purchase grants the character fluency in one language[...]".to_owned(),
        ),
        (
            "Language (Low Realm) (•)".to_owned(),
            1,
            MeritType::Purchased,
            "Each purchase grants the character fluency in one language[...]".to_owned(),
        ),
        (
            "Eidetic Memory (••)".to_owned(),
            2,
            MeritType::Innate,
            "The character enjoys near-perfect recall[...]".to_owned(),
        ),
    ]
    .into_iter()
    .collect();

    assert!(actual.len() == expected.len());
    for act in actual.iter() {
        assert_eq!(Some(act), expected.get(&act));
    }
}

#[test]
fn test_default_character_weapons() {
    let mortal = default_mortal_character();

    assert!(mortal
        .weapons
        .equipped_iter()
        .collect::<Vec<(usize, &Weapon)>>()
        .is_empty());
    assert!(mortal
        .weapons
        .iter()
        .collect::<Vec<(usize, &Weapon)>>()
        .is_empty());
}

#[test]
fn test_custom_character_weapons() {
    let mut mortal = custom_mortal_character();

    // Test owned iterato
    let expected: [(
        usize,
        String,
        Option<i8>,
        Option<i8>,
        i8,
        Option<i8>,
        u8,
        i8,
    ); 1] = [(0, "Hook Swords".to_owned(), Some(2), None, 9, Some(1), 0, 1)];

    let actual = mortal.weapons.iter().collect::<Vec<(usize, &Weapon)>>();

    assert_eq!(actual.len(), expected.len());

    for ((actual_key, actual_weapon), right) in mortal.weapons.iter().zip(expected) {
        assert_eq!(actual_key, right.0);
        assert_eq!(actual_weapon.name(), right.1);
        assert_eq!(actual_weapon.accuracy(RangeBand::Close), right.2);
        assert_eq!(actual_weapon.accuracy(RangeBand::Extreme), right.3);
        assert_eq!(actual_weapon.damage(), right.4);
        assert_eq!(actual_weapon.defense(), right.5);
        assert_eq!(actual_weapon.attunement(), right.6);
        assert_eq!(actual_weapon.overwhelming(), right.7);
    }

    // Test equipped iterator
    let expected: [(
        usize,
        String,
        Option<i8>,
        Option<i8>,
        i8,
        Option<i8>,
        u8,
        i8,
    ); 2] = [
        (0, "Hook Swords".to_owned(), Some(2), None, 9, Some(1), 0, 1),
        (0, "Hook Swords".to_owned(), Some(2), None, 9, Some(1), 0, 1),
    ];

    let actual = mortal
        .weapons
        .equipped_iter()
        .collect::<Vec<(usize, &Weapon)>>();

    assert_eq!(actual.len(), expected.len());

    for ((actual_key, actual_weapon), right) in mortal.weapons.iter().zip(expected) {
        assert_eq!(actual_key, right.0);
        assert_eq!(actual_weapon.name(), right.1);
        assert_eq!(actual_weapon.accuracy(RangeBand::Close), right.2);
        assert_eq!(actual_weapon.accuracy(RangeBand::Extreme), right.3);
        assert_eq!(actual_weapon.damage(), right.4);
        assert_eq!(actual_weapon.defense(), right.5);
        assert_eq!(actual_weapon.attunement(), right.6);
        assert_eq!(actual_weapon.overwhelming(), right.7);
    }

    // Test unequip
    mortal.weapons.unequip(EquipHand::Main);

    let expected: [(
        usize,
        String,
        Option<i8>,
        Option<i8>,
        i8,
        Option<i8>,
        u8,
        i8,
    ); 1] = [(0, "Hook Swords".to_owned(), Some(2), None, 9, Some(1), 0, 1)];

    let actual = mortal
        .weapons
        .equipped_iter()
        .collect::<Vec<(usize, &Weapon)>>();

    assert_eq!(actual.len(), expected.len());

    for ((actual_key, actual_weapon), right) in mortal.weapons.iter().zip(expected) {
        assert_eq!(actual_key, right.0);
        assert_eq!(actual_weapon.name(), right.1);
        assert_eq!(actual_weapon.accuracy(RangeBand::Close), right.2);
        assert_eq!(actual_weapon.accuracy(RangeBand::Extreme), right.3);
        assert_eq!(actual_weapon.damage(), right.4);
        assert_eq!(actual_weapon.defense(), right.5);
        assert_eq!(actual_weapon.attunement(), right.6);
        assert_eq!(actual_weapon.overwhelming(), right.7);
    }
}

#[test]
fn test_default_character_armor() {
    let mortal = default_mortal_character();

    assert!(mortal.armor.iter().count() == 0);
    assert!(mortal.armor.equipped().is_none());
}

#[test]
fn test_custom_character_armor() {
    let mut mortal = custom_mortal_character();

    assert!(mortal.armor.iter().count() == 1);
    assert!(mortal.armor.equipped().is_some());
    let actual = mortal.armor.equipped().unwrap();
    assert_eq!(actual.name(), "Silken Armor".to_owned());
    assert_eq!(actual.soak(), 5);
    assert_eq!(actual.hardness(), 4);
    assert_eq!(actual.mobility_penality(), 0);
    assert_eq!(actual.attunement(), 4);

    mortal.armor.unequip_armor_item();
    assert!(mortal.armor.iter().count() == 1);
    assert!(mortal.armor.equipped().is_none());
}
