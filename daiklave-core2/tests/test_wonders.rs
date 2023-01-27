use daiklave_core2::{
    artifact::{wonders::WonderId, Artifact, ArtifactName},
    book_reference::{Book, BookReference},
    unique_id::UniqueId,
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_wonders() {
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character().unwrap();

    // Default: no wonders
    assert!(character.wonders().iter().next().is_none());

    // Add a wonder
    let wonder = Artifact::Wonder(
        WonderId(UniqueId::Placeholder(1)),
        Artifact::wonder_builder("Belt of Shadow Walking")
            .book_reference(BookReference::new(Book::CoreRulebook, 602))
            .merit_dots(3)
            .powers("Night-black belts made from leathe from the wings of giant bats[...]")
            .attunement_cost(5)
            .build(),
    );
    let mutation = CharacterMutation::AddArtifact(wonder);
    let character = event_source.apply_mutation(mutation).unwrap();

    // Check the wonder's properties
    assert_eq!(
        character.wonders().iter().collect::<Vec<WonderId>>(),
        vec![WonderId(UniqueId::Placeholder(1))]
    );
    let wonder = character
        .wonders()
        .get(WonderId(UniqueId::Placeholder(1)))
        .unwrap();
    assert_eq!(wonder.id(), WonderId(UniqueId::Placeholder(1)));
    assert_eq!(wonder.name(), "Belt of Shadow Walking");
    assert!(wonder.book_reference().is_some());
    assert_eq!(
        wonder.powers(),
        "Night-black belts made from leathe from the wings of giant bats[...]"
    );
    assert!(wonder.lore().is_none());
    assert!(wonder.material().is_none());
    assert_eq!(wonder.hearthstone_slots(), 0);

    // Remove the wonder
    let mutation =
        CharacterMutation::RemoveArtifact(ArtifactName::Wonder(WonderId(UniqueId::Placeholder(1))));
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character().unwrap();

    assert!(character
        .wonders()
        .get(WonderId(UniqueId::Placeholder(1)))
        .is_none());
    assert!(character.wonders().iter().next().is_none());
}
