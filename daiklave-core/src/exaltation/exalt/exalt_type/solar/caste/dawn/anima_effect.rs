use crate::{
    book_reference::{Book, BookReference},
    exaltation::exalt::AnimaEffect,
};

pub(crate) const DAWN_ONE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 175,
    }),
    description: "A Dawn Caste adds (Essence/2, round up) non-Charm dice to all \
    intimidation checks, and may attempt to intimidate even beings which do \
    not feel fear.",
};

pub(crate) const DAWN_TWO: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 175,
    }),
    description: "A Dawn Caste may spend 10 motes to reset all combat and movement \
    Charms with limited uses. Limit once per day, resetting at dawn.",
};

pub(crate) const DAWN_THREE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 175,
    }),
    description: "At bonfire anima, a Dawn Caste adds (Essence/2, round up) non-Charm \
    dice to base Initiative after resetting after a successful decisive attack.",
};
