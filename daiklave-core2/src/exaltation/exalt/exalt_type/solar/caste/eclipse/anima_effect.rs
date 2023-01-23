use crate::{
    book_reference::{Book, BookReference},
    exaltation::exalt::AnimaEffect,
};

pub(crate) const ECLIPSE_ONE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook, 
        page_number: 177
    }),
    description: "An Eclipse Caste may learn Charms from spirits with the Eclipse \
    Keyword at a cost of 8xp per Charm.",
};

pub(crate) const ECLIPSE_TWO: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook, 
        page_number: 177
    }),
    description: "An Eclipse Caste and their companions may not be attacked by \
    spirits, ghosts, demons, or the Fair Folk so long as they are peacefully \
    pursuing legitimate business with them.",
};

pub(crate) const ECLIPSE_THREE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook, 
        page_number: 176
    }),
    description: "An Eclipse Caste may spend 10 motes to witness and sanctify an \
    oath. If the oath is broken, the oathbreaker suffers a curse chosen by the \
    Storyteller.",
};
