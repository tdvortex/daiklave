/// The different phases of a guided character builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuidedStage {
    /// The first stage, choosing a character name and (optional) concept.
    ChooseNameAndConcept,
    /// The second stage, choosing the Exaltation for the character (or Mortal).
    ChooseExaltation,
    /// The attribute selection stage. Comes after ChooseExaltation for
    /// Mortals and Solars.
    ChooseAttributes,
    /// The stage where Solars pick five Caste abilities from the 7 available
    /// for their Caste.
    ChooseSolarCasteAbilities,
    /// The stage where Solars pick their Supernal ability from the 5 Caste
    /// abilities they previously selected, except that Dawn castes may
    /// instead pick Martial Arts if Brawl is a selected caste ability.
    ChooseSolarSupernalAbility,
    /// The stage where Solars pick their Favored abilities.
    ChooseSolarFavoredAbilities,
    /// A stage for selecting which Martial Arts styles (if any) the character
    /// practices. This purchases the MartialArtist merit and forces Brawl 1
    /// but does not purchase any MartialArts dots, specialties, or charms.
    ChooseMartialArtsStyles,
    /// A stage for selecting whether to be a sorcerer or not, and if so, what
    /// Terrestrial shaping ritual they use, and what their Control Spell is.
    /// This purchases either the Mortal Sorcerer merit if mortal, or the
    /// Terrestrial Circle Sorcery Charm if Exalted, and forces Occult 3, but
    /// does not purchase any non-Control Spells or associated Shaping Ritual
    /// merits.
    ChooseSorcery,
    /// The stage for choosing Ability dot values, including Martial Arts and
    /// Craft.
    ChooseAbilities,
    /// The stage for choosing Specialties for your abilities.
    ChooseSpecialties,
}
