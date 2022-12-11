use exalted_3e_gui::{
    armor::{ArmorItem, ArmorTag},
    health::WoundPenalty,
    intimacies::{Intimacy, IntimacyLevel, IntimacyType},
    player::Player,
    weapons::{EquipHand, RangeBand, Weapon, WeaponTag},
    Character,
};

use super::{create_initial_base_character, attributes::create_attributes, abilities::create_abilities};





pub fn create_initial_character(player: &Player) -> Character {
    let initial_character = create_initial_base_character(player);
    let initial_character = create_attributes(initial_character);
    let initial_character = create_abilities(initial_character);

    initial_character
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
