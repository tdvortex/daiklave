use crate::{
    book_reference::{Book, BookReference},
    exaltation::exalt::AnimaEffect,
};

pub(crate) const ZENITH_ONE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 176,
    }),
    description: "A Zenith Caste may spend 1 mote to cremate a corpse, and \
    learn the Intimacies of the deceased. If these Intimacies are postive, the \
    Zenith may pay 1 mote to pass them onto a target of those Intimacies, or \
    bind them to an object. If these Intimacies are negative, the Zenith may \
    touch a responsible party, spend 1 mote, and roll (any social Attribute + \
    Presence + 3 non-Charm successes) vs Resolve to inflict pain on the victim.",
};

pub(crate) const ZENITH_TWO: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 176,
    }),
    description: "A Zenith Caste may spend 7 motes and roll (Charisma + \
        Presence) plus (Essence) automatic non-Charm successes to persuade a \
        dematerialized spirit to Materialize. The target counts as having a \
        Defining Intimacy for this persuasion, but does not have to spend \
        motes to materialize.",
};

pub(crate) const ZENITH_THREE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 176,
    }),
    description: "A Zenith Caste may spend 10 motes and 1 Willpower after \
    successfully landing a decisive attack against a creature of darkness to \
    prevent returning to base Initiative. At bonfire anima, the costs is 5 \
    motes, no Willpower. Limit once per day, resetting at midday.",
};
