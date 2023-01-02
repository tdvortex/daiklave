/// The different phases of a guided character builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidedStage {
    /// The first stage, choosing a character name and (optional) concept.
    NameAndConcept,
    /// The second stage, choosing the Exaltation for the character (or Mortal).
    Exaltation,
    /// The attribute selection stage. Comes after ChooseExaltation for
    /// Mortals and Solars.
    Attributes,
    /// The stage where Solars pick five Caste abilities from the 7 available
    /// for their Caste.
    SolarCasteAbilities,
    /// The stage where Solars pick their Supernal ability from the 5 Caste
    /// abilities they previously selected, except that Dawn castes may
    /// instead pick Martial Arts if Brawl is a selected caste ability.
    SolarSupernalAbility,
    /// The stage where Solars pick their Favored abilities.
    SolarFavoredAbilities,
    /// A stage for selecting which Martial Arts styles (if any) the character
    /// practices. This purchases the MartialArtist merit and forces Brawl 1
    /// but does not purchase any MartialArts dots, specialties, or charms.
    MartialArtsStyles,
    /// A stage for selecting whether to be a sorcerer or not, and if so, what
    /// Terrestrial shaping ritual they use, and what their Control Spell is.
    /// This purchases either the Mortal Sorcerer merit if mortal, or the
    /// Terrestrial Circle Sorcery Charm if Exalted, and forces Occult 3, but
    /// does not purchase any non-Control Spells or associated Shaping Ritual
    /// merits.
    Sorcery,
    /// The stage for choosing Ability dot values, including Martial Arts and
    /// Craft.
    Abilities,
    /// The stage for choosing Specialties for your abilities.
    Specialties,
}
