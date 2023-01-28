pub enum AbilityNameQualified {
    Vanilla(AbilityNameVanilla),
    Craft(&'source str),
    MartialArts(&'source str),
}