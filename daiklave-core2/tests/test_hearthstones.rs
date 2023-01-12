use daiklave_core2::{CharacterEventSource, hearthstones::{hearthstone::{Hearthstone, HearthstoneCategory, GeomancyLevel, HearthstoneKeyword}, HearthstoneId}, book_reference::{BookReference, Book}, CharacterMutation, unique_id::UniqueId, weapons::weapon::{Weapon, BaseWeaponId, WeaponWeightClass, WeaponTag, OptionalWeaponTag, ArtifactWeaponId, WeaponId}, artifact::{MagicMaterial, Artifact, wonders::WonderId, ArtifactId}, armor::{armor_item::{ArmorItem, BaseArmorId, artifact::ArtifactArmorId, ArmorWeightClass, ArmorTag}}};

#[test]
fn test_hearthstones() {
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character_view().unwrap();
    
    // Characters have no hearthstones by default
    assert!(character.hearthstones().iter().next().is_none());

    // Add two hearthstones, one with a manse
    let jewel = Hearthstone::new("Jewel of the Celestial Mandarin".to_string())
        .book_reference(BookReference::new(Book::CoreRulebook, 611))
        .category(HearthstoneCategory::Sidereal)
        .geomancy_level(GeomancyLevel::Standard)
        .powers("This transparent, faceted square stone glows with a violet\
        light. Any individual bearing it in an attuned hearthstone\
        socket may glimpse the abodes of spirits—the doors of the\
        sanctums where gods dwell become visible to her eyes.\
        Although this hearthstone doesn't grant the power to enter\
        such sanctums, she can speak and her voice will be heard\
        within, booming with celestial authority—any command\
        to come forth is treated as though it aligns with a Minor\
        Intimacy.".to_owned())
        .manseborn()
        .build();

    let jewel_clone = jewel.clone();

    let mutation = CharacterMutation::AddHearthstone(HearthstoneId(UniqueId::Placeholder(1)), jewel);
    character.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character_view().unwrap();
    
    let eye = Hearthstone::new("Hierophant's Eye".to_string())
        .book_reference(BookReference::new(Book::CoreRulebook, 610))
        .category(HearthstoneCategory::Solar)
        .geomancy_level(GeomancyLevel::Greater)
        .powers("This black octagonal stone blazes with orange-gold light
            along each of its perfect edges. The Hierophant's Eye
            grants one automatic non-Charm success to every shape
            sorcery action while socketed into an attuned artifact.".to_owned())
        .linked()
        .dependent()
        .build();

    let manse = "A shiny mansion".to_owned();
    let demense = "A cool place".to_owned();
    let mutation = CharacterMutation::AddManse(manse, demense, HearthstoneId(UniqueId::Placeholder(2)), eye);
    character.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character_view().unwrap();

    // Check the properties
    let eye_get = character.hearthstones().get(HearthstoneId(UniqueId::Placeholder(2))).unwrap();
    assert_eq!(eye_get.name(), "Hierophant's Eye");
    assert_eq!(eye_get.id(), HearthstoneId(UniqueId::Placeholder(2)));
    assert_eq!(eye_get.book_reference(), Some(BookReference::new(Book::CoreRulebook, 610)));
    assert_eq!(eye_get.category(), HearthstoneCategory::Solar);
    assert_eq!(eye_get.geomancy_level(), GeomancyLevel::Greater);
    assert_eq!(eye_get.keywords().collect::<Vec<HearthstoneKeyword>>(), vec![HearthstoneKeyword::Dependent, HearthstoneKeyword::Linked]);
    assert_eq!(eye_get.manse_and_demense().unwrap(), ("A shiny mansion", "A cool place"));

    // Check you can't add a duplicate hearthstone
    assert!(character.check_mutation(&CharacterMutation::AddHearthstone(HearthstoneId(UniqueId::Placeholder(1)), jewel)).is_err());

    // Add artifacts to slot them into
    let adorei = Weapon::artifact("Beloved Adorei")
        .base_artifact(
            BaseWeaponId(UniqueId::Placeholder(1)), 
            Weapon::base("Daiklave")
            .book_reference(BookReference::new(Book::CoreRulebook, 614))
            .weight_class(WeaponWeightClass::Medium)
            .one_handed()
            .lethal()
            .melee()
            .tag(OptionalWeaponTag::Balanced)
            .build_artifact()
        ).lore("Auravash the Twilight craftsman saw a need for the\
            daiklave Adorei when he looked upon the Dawn of his\
            Circle, Brother of Brothers, Night of His Heart, whom he\
            considered to be an idiot and perhaps the single greatest\
            reaver the Celestial Host had ever produced.")
        .material(MagicMaterial::Orichalcum)
        .merit_dots(3)
        .powers("If the Solar who wields Adorei exults in battles, the blade\
            offers +1 to Accuracy upon attunement. The weapon will\
            form an instant Major Tie of affection to a Solar who takes\
            her out of the darkness of the tomb to carry her into battle[...]")
        .hearthstone_slots(3)
        .build();

    let mutation = CharacterMutation::AddArtifact(Artifact::Weapon(ArtifactWeaponId(UniqueId::Placeholder(1)), adorei));
    character.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character_view().unwrap();

    let freedoms_cadence = ArmorItem::artifact("Freedom's Cadence")
        .book_reference(BookReference::new(Book::CoreRulebook, 621))
        .base_artifact(BaseArmorId(UniqueId::Placeholder(1)), 
        ArmorItem::base("Chain Shirt (Artifact)").book_reference(BookReference::new(Book::CoreRulebook, 599)).weight_class(ArmorWeightClass::Light).tag(ArmorTag::Concealable).build_artifact()
    ).lore("So it was said, at the end of sixteen cycles, Gu-Shaiyen, \
    Celestial Daimyo of the Drums of War, poured the last of \
    his spirit into a masterwork of starmetal, creating the \
    hauberk called Freedom's Cadence.")
    .powers("The lamellar pauldrons, vambraces and fist-guards that \
    come with the armor allow for dramatic blocks and parries \
    against hard-striking heavy weaponry, but do not count \
    as actual armor. A Sidereal, Solar, or Getimian Exalted \
    may pay three extra motes when attuning this armor to \
    loosen the substance of its Essence, causing it to no longer \
    count as armor. This part of the attunement is cancelled \
    if the wearer suffers crash.")
    .material(MagicMaterial::Starmetal)
    .merit_dots(4)
    .hearthstone_slots(1)
    .build();

    let mutation = CharacterMutation::AddArtifact(Artifact::Armor(ArtifactArmorId(UniqueId::Placeholder(1)), freedoms_cadence));
    character.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character_view().unwrap();

    let hearthstone_amulet = Artifact::wonder("Hearthstone Amulet").attunement_cost(1).book_reference(BookReference::new(Book::CoreRulebook, 601)).hearthstone_slots(1).merit_dots(2).powers("A wide variety of decorative artifacts exist whose sole \
        purpose is to provide a socket into which an attuned \
        hearthstone may be placed. The most common form of \
        these artifacts are amulets wrought of the five magical \
        materials, though hearthstone bracers are nearly as \
        popular, and some Chosen—particularly Sidereals—prefer \
        circlets or tiaras which fix the hearthstone in the center \
        of their forehead, directly over their Caste Mark. All such \
        items cost a single mote to attune. \
        If a character wears magical armor of the same material \
        as a worn hearthstone amulet (tiara, bracer, etc.), then the \
        amulet's socket is considered to be part of that armor for \
        the purpose of dependent hearthstones (see p. 604).")
        .magic_material(MagicMaterial::Starmetal)
        .build();

    let mutation = CharacterMutation::AddArtifact(Artifact::Wonder(WonderId(UniqueId::Placeholder(1)), hearthstone_amulet));
    character.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character_view().unwrap();

    // Check slotting into all three artifacts
    let mutation = CharacterMutation::SlotHearthstone(ArtifactId::Wonder(WonderId(UniqueId::Placeholder(1))), HearthstoneId(UniqueId::Placeholder(2)));
    character.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character_view().unwrap();

    let mutation = CharacterMutation::SlotHearthstone(ArtifactId::Armor(ArtifactArmorId(UniqueId::Placeholder(1))), HearthstoneId(UniqueId::Placeholder(1)));
    character.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character_view().unwrap();

    // If a hearthstone is already slotted, reslotting it will remove it from
    // the original position
    let mutation = CharacterMutation::SlotHearthstone(ArtifactId::Weapon(ArtifactWeaponId(UniqueId::Placeholder(1))), HearthstoneId(UniqueId::Placeholder(2)));
    character.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character_view().unwrap();
    assert_eq!(character.weapons().get(WeaponId::Artifact(ArtifactWeaponId(UniqueId::Placeholder(1))), None).unwrap().slotted_hearthstones().next().unwrap().id(), HearthstoneId(UniqueId::Placeholder(2)));
    assert!(character.wonders().get(WonderId(UniqueId::Placeholder(1))).unwrap().slotted_hearthstones().next().is_none());

    // Check you can unslot a hearthstone
    let mutation = CharacterMutation::UnslotHearthstone(HearthstoneId(UniqueId::Placeholder(1)));
    character.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character_view().unwrap();

    // Check you can remove an unslotted hearthstone
    let mutation = CharacterMutation::RemoveHearthstone(HearthstoneId(UniqueId::Placeholder(1)));
    character.check_mutation(&mutation).unwrap();
    event_source.apply_mutation(mutation).unwrap();
    let character = event_source.as_character_view().unwrap();
    assert!(character.hearthstones().get(HearthstoneId(UniqueId::Placeholder(1))).is_none());

    // Check you can't remove a slotted hearthstone
    let mutation = CharacterMutation::RemoveHearthstone(HearthstoneId(UniqueId::Placeholder(2)));
    assert!(character.check_mutation(&mutation).is_err());
}