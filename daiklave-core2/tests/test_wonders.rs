use daiklave_core2::{
    artifact::{AddArtifact, ArtifactNameMutation},
    book_reference::{Book, BookReference},
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_wonders() {
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character().unwrap();

    // Default: no wonders
    assert!(character.wonders().iter().next().is_none());

    // Add a wonder
    let wonder = AddArtifact::Wonder(
        AddArtifact::wonder_builder("Belt of Shadow Walking")
            .book_reference(BookReference::new(Book::CoreRulebook, 602))
            .merit_dots(3)
            .powers("Night-black belts made from leather from the wings of giant bats[...]")
            .attunement_cost(5)
            .build(),
    );
    let mutation = CharacterMutation::AddArtifact(wonder);
    let character = event_source.apply_mutation(mutation).unwrap();

    // Check the wonder's properties
    assert_eq!(
        character.wonders().iter().collect::<Vec<&str>>(),
        vec!["Belt of Shadow Walking"]
    );
    let wonder = character.wonders().get("Belt of Shadow Walking").unwrap();
    assert_eq!(wonder.name(), "Belt of Shadow Walking");
    assert!(wonder.book_reference().is_some());
    assert_eq!(
        wonder.powers(),
        "Night-black belts made from leather from the wings of giant bats[...]"
    );
    assert!(wonder.lore().is_none());
    assert!(wonder.material().is_none());
    assert_eq!(wonder.hearthstone_slots(), 0);

    // Remove the wonder
    let mutation = CharacterMutation::RemoveArtifact(ArtifactNameMutation::Wonder(
        "Belt of Shadow Walking".to_owned(),
    ));
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character().unwrap();

    assert!(character.wonders().get("Belt of Shadow Walking").is_none());
    assert!(character.wonders().iter().next().is_none());
}
