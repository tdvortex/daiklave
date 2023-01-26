use daiklave_core2::{
    abilities::AbilityName,
    armor::armor_item::{artifact::ArtifactArmorId, ArmorItem, ArmorWeightClass, BaseArmorId},
    artifact::{wonders::WonderId, Artifact, ArtifactId, MagicMaterial},
    book_reference::{Book, BookReference},
    exaltation::exalt::{
        essence::{MotePoolName, UncommitMotes},
        exalt_type::solar::{
            caste::{DawnCasteAbility, DawnSupernalAbility},
            Solar,
        },
    },
    unique_id::UniqueId,
    weapons::weapon::{
        ArtifactWeaponId, BaseWeaponId, OptionalWeaponTag, Weapon, WeaponWeightClass,
    },
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_attunement() {
    let mut event_source = CharacterEventSource::default();
    // Add some stuff to attune to
    let glider = Artifact::wonder_builder("Essence Glider")
        .attunement_cost(2)
        .book_reference(BookReference::new(Book::CoreRulebook, 603))
        .merit_dots(3)
        .powers(
            "This fragile-looking construct of magical material, \
            feathersteel, and Essence has a wingspan of 20 feet[...]",
        )
        .build();

    let yasal = Artifact::wonder_builder("Yasal Crystal")
        .book_reference(BookReference::new(Book::CoreRulebook, 601))
        .merit_dots(2)
        .powers(
            "This extraordinarily valuable yellow gemstone can trap \
            minor spirits and newly made ghosts.",
        )
        .build();

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

    let dauntless = ArmorItem::artifact("Dauntless")
        .book_reference(BookReference::new(Book::CoreRulebook, 624))
        .lore(
            "When Zan the Invincible, Sword of Heaven, stood alone \
            against an army of thirty men, his Circlemates worried.",
        )
        .powers(
            "By paying four extra motes at attunement, Dauntless grants \
            the Solar great strength and ferocity, adding one non-Charm \
            automatic success to any attempt to lift or break an object \
            and treating his Strength as if it were three higher for all \
            such attempts.",
        )
        .base_artifact(
            BaseArmorId(UniqueId::Placeholder(1)),
            ArmorItem::base("Lamellar (Artifact)")
                .book_reference(BookReference::new(Book::CoreRulebook, 600))
                .weight_class(ArmorWeightClass::Medium)
                .build_artifact(),
        )
        .material(MagicMaterial::Orichalcum)
        .merit_dots(5)
        .hearthstone_slots(2)
        .build();

    event_source
        .apply_mutation(CharacterMutation::AddArtifact(Artifact::Wonder(
            WonderId(UniqueId::Placeholder(1)),
            glider,
        )))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::AddArtifact(Artifact::Wonder(
            WonderId(UniqueId::Placeholder(2)),
            yasal,
        )))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::AddArtifact(Artifact::Weapon(
            ArtifactWeaponId(UniqueId::Placeholder(1)),
            spring_razor,
        )))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::AddArtifact(Artifact::Armor(
            ArtifactArmorId(UniqueId::Placeholder(1)),
            dauntless,
        )))
        .unwrap();

    // Mortals can't attune to anything
    assert!(event_source
        .apply_mutation(CharacterMutation::AttuneArtifact(
            ArtifactId::Wonder(WonderId(UniqueId::Placeholder(1))),
            MotePoolName::Peripheral
        ))
        .is_err());

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

    // Can't attune to a missing artifact
    assert!(event_source
        .apply_mutation(CharacterMutation::AttuneArtifact(
            ArtifactId::Wonder(WonderId(UniqueId::Placeholder(666))),
            MotePoolName::Peripheral
        ))
        .is_err());

    // Exalts can attune to anything with an attunement cost
    event_source
        .apply_mutation(CharacterMutation::AttuneArtifact(
            ArtifactId::Wonder(WonderId(UniqueId::Placeholder(1))),
            MotePoolName::Peripheral,
        ))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::AttuneArtifact(
            ArtifactId::Weapon(ArtifactWeaponId(UniqueId::Placeholder(1))),
            MotePoolName::Peripheral,
        ))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AttuneArtifact(
            ArtifactId::Armor(ArtifactArmorId(UniqueId::Placeholder(1))),
            MotePoolName::Peripheral,
        ))
        .unwrap();
    assert_eq!(
        character
            .essence()
            .unwrap()
            .motes()
            .committed()
            .map(|(_, commitment)| (commitment.peripheral(), commitment.personal()))
            .fold(
                (0, 0),
                |(peripheral_sum, personal_sum), (peripheral, personal)| (
                    peripheral_sum + peripheral,
                    personal_sum + personal
                )
            ),
        (12, 0)
    );
    assert_eq!(
        character
            .essence()
            .unwrap()
            .motes()
            .peripheral()
            .available(),
        21
    );
    assert_eq!(character.essence().unwrap().motes().peripheral().spent(), 0);

    // Exalts cannot attune to no-attunement Wonders
    assert!(event_source
        .apply_mutation(CharacterMutation::AttuneArtifact(
            ArtifactId::Wonder(WonderId(UniqueId::Placeholder(2))),
            MotePoolName::Peripheral,
        ))
        .is_err());

    // Exalts can unattune from everything they've attuned to
    event_source
        .apply_mutation(CharacterMutation::UncommitMotes(
            UncommitMotes::UnattuneArtifact(ArtifactId::Wonder(WonderId(UniqueId::Placeholder(1)))),
        ))
        .unwrap();
    event_source
        .apply_mutation(CharacterMutation::UncommitMotes(
            UncommitMotes::UnattuneArtifact(ArtifactId::Armor(ArtifactArmorId(
                UniqueId::Placeholder(1),
            ))),
        ))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::UncommitMotes(
            UncommitMotes::UnattuneArtifact(ArtifactId::Weapon(ArtifactWeaponId(
                UniqueId::Placeholder(1),
            ))),
        ))
        .unwrap();

    assert_eq!(
        character
            .essence()
            .unwrap()
            .motes()
            .committed()
            .map(|(_, commitment)| (commitment.peripheral(), commitment.personal()))
            .fold(
                (0, 0),
                |(peripheral_sum, personal_sum), (peripheral, personal)| (
                    peripheral_sum + peripheral,
                    personal_sum + personal
                )
            ),
        (0, 0)
    );
    assert_eq!(
        character
            .essence()
            .unwrap()
            .motes()
            .peripheral()
            .available(),
        21
    );
    assert_eq!(
        character.essence().unwrap().motes().peripheral().spent(),
        12
    );

    // Can't unattune from an artifact that is already unattuned
    assert!(event_source
        .apply_mutation(CharacterMutation::UncommitMotes(
            UncommitMotes::UnattuneArtifact(ArtifactId::Wonder(WonderId(UniqueId::Placeholder(1))))
        ))
        .is_err());
}
