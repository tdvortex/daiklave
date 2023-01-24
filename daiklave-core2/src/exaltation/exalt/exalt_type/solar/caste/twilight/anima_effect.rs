use crate::{
    book_reference::{Book, BookReference},
    exaltation::exalt::AnimaEffect,
};

pub(crate) const TWILIGHT_ONE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 176,
    }),
    description: "A Twilight Caste may reflexively spend 5 motes to gain 5 \
    Hardness for one turn. This is usable even in Initiative Crash. This is \
    free and automatic at bonfire anima. This does not stack with other \
    effects that raise Hardness.",
};

pub(crate) const TWILIGHT_TWO: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 176,
    }),
    description: "A Twilight Caste may spend 10 motes to be consumed by their \
    anima. This takes 1 round and is disrupted if they move or are knocked \
    down. Once complete, the Twilight disappears, and reappears the next \
    sunset at a location of the Storyteller's choosing within 10 miles.",
};

pub(crate) const TWILIGHT_THREE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 176,
    }),
    description: "A Twilight Case may spend 10 motes and touch an elemental \
    or demon of Essence 3 or less. Roll (Intelligence + Occult) vs the \
    target's Resolve. On a success, the target becomes the Twilight's \
    familiar. They can reflexively summon the creature for 3 motes, or \
    temporarily banish them for free. Limit (Essence) familiars at a time.",
};
