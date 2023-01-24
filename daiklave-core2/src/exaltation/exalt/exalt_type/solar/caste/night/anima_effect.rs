use crate::{
    book_reference::{Book, BookReference},
    exaltation::exalt::AnimaEffect,
};

pub(crate) const NIGHT_ONE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 176,
    }),
    description: "A Night Caste may pay 2 extra motes when activating an effect to \
    prevent their anima from flaring.",
};

pub(crate) const NIGHT_TWO: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 176,
    }),
    description: "A Night Caste may pay 3 motes to ignore (higher of Essence or 3) \
    penalties to one Stealth attempt. This never flares anima.",
};

pub(crate) const NIGHT_THREE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook,
        page_number: 176,
    }),
    description: "At bonfire anima, a Night Caste's identity becomes obscured by \
    their anima, and cannot be recognized by any effect.",
};
