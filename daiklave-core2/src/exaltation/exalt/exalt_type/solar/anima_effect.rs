use crate::{exaltation::exalt::AnimaEffect, book_reference::{BookReference, Book}};

pub(crate) const SOLAR_ONE: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook, 
        page_number: 175
    }),
    description: "Solars may spend 1 mote to know the exact position of the \
    sun and time of day, even while underground or outside Creation.",
};


pub(crate) const SOLAR_TWO: AnimaEffect<'static> = AnimaEffect {
    book_reference: Some(BookReference {
        book: Book::CoreRulebook, 
        page_number: 175
    }),
    description: "Solars may spend 1 mote to display their Caste Mark even at \
    Dim anima.",
};