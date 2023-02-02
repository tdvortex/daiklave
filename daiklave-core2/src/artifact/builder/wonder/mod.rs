use crate::{
    artifact::{
        wonders::{AddWonder, Wonder, WonderName, WonderNoAttunementMemo},
        MagicMaterial,
    },
    book_reference::BookReference,
};

/// A builder to construct a Wonder. Note that unlike artifacts and weapons,
/// magic materials are optional, but powers are not. Required fields (in
/// order) are name, merit dots, and powers. Hearthstone slots are also
/// required but are defaulted to 0.
pub struct WonderBuilder {
    pub(crate) name: WonderName,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) lore: Option<String>,
    pub(crate) magic_material: Option<MagicMaterial>,
    pub(crate) hearthstone_slots: u8,
    pub(crate) attunement_cost: Option<u8>,
}

impl WonderBuilder {
    /// The book reference for the Wonder.
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// The item's lore and history.
    pub fn lore(mut self, lore: String) -> Self {
        self.lore = Some(lore);
        self
    }

    /// The primary magic material used in construction of the wonder.
    pub fn magic_material(mut self, material: MagicMaterial) -> Self {
        self.magic_material = Some(material);
        self
    }

    /// The number of hearthstone slots in the artifact.
    pub fn hearthstone_slots(mut self, hearthstone_slots: u8) -> Self {
        self.hearthstone_slots = hearthstone_slots;
        self
    }

    /// The cost to attune to the artifact (if needed).
    pub fn attunement_cost(mut self, attunement_cost: u8) -> Self {
        self.attunement_cost = Some(attunement_cost);
        self
    }

    /// The number of merit dots required to purchase the artifact.
    pub fn merit_dots(self, dots: u8) -> WonderBuilderWithMeritDots {
        WonderBuilderWithMeritDots {
            name: self.name,
            book_reference: self.book_reference,
            lore: self.lore,
            magic_material: self.magic_material,
            merit_dots: dots,
            hearthstone_slots: self.hearthstone_slots,
            attunement_cost: self.attunement_cost,
        }
    }
}

/// A wonder builder after merit dots have been specified.
pub struct WonderBuilderWithMeritDots {
    name: WonderName,
    book_reference: Option<BookReference>,
    lore: Option<String>,
    magic_material: Option<MagicMaterial>,
    merit_dots: u8,
    hearthstone_slots: u8,
    attunement_cost: Option<u8>,
}

impl WonderBuilderWithMeritDots {
    /// The book reference for the Wonder (if any).
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// The item's lore and history (if any).
    pub fn lore(mut self, lore: String) -> Self {
        self.lore = Some(lore);
        self
    }

    /// The primary magic material used in construction of the wonder.
    pub fn magic_material(mut self, material: MagicMaterial) -> Self {
        self.magic_material = Some(material);
        self
    }

    /// The number of hearthstone slots in the artifact.
    pub fn hearthstone_slots(mut self, hearthstone_slots: u8) -> Self {
        self.hearthstone_slots = hearthstone_slots;
        self
    }

    /// The cost to attune to the artifact (if needed).
    pub fn attunement_cost(mut self, attunement_cost: u8) -> Self {
        self.attunement_cost = Some(attunement_cost);
        self
    }

    /// Sets the powers of the artifact.
    pub fn powers(self, powers: &str) -> WonderBuilderWithPowers {
        WonderBuilderWithPowers {
            name: self.name,
            book_reference: self.book_reference,
            lore: self.lore,
            magic_material: self.magic_material,
            merit_dots: self.merit_dots,
            hearthstone_slots: self.hearthstone_slots,
            powers: powers.to_owned(),
            attunement_cost: self.attunement_cost,
        }
    }
}

/// A wonder builder with powers.
pub struct WonderBuilderWithPowers {
    name: WonderName,
    book_reference: Option<BookReference>,
    lore: Option<String>,
    magic_material: Option<MagicMaterial>,
    merit_dots: u8,
    powers: String,
    hearthstone_slots: u8,
    attunement_cost: Option<u8>,
}

impl WonderBuilderWithPowers {
    /// The book reference for the Wonder (if any).
    pub fn book_reference(mut self, book_reference: BookReference) -> Self {
        self.book_reference = Some(book_reference);
        self
    }

    /// The item's lore and history (if any).
    pub fn lore(mut self, lore: String) -> Self {
        self.lore = Some(lore);
        self
    }

    /// The primary magic material used in construction of the wonder.
    pub fn magic_material(mut self, material: MagicMaterial) -> Self {
        self.magic_material = Some(material);
        self
    }

    /// The number of hearthstone slots in the artifact.
    pub fn hearthstone_slots(mut self, hearthstone_slots: u8) -> Self {
        self.hearthstone_slots = hearthstone_slots;
        self
    }

    /// The cost to attune to the artifact (if needed).
    pub fn attunement_cost(mut self, attunement_cost: u8) -> Self {
        self.attunement_cost = Some(attunement_cost);
        self
    }

    /// Completes the builder.
    pub fn build(self) -> AddWonder {
        AddWonder {
            name: self.name,
            wonder: Wonder(WonderNoAttunementMemo {
                book_reference: self.book_reference,
                lore: self.lore,
                powers: self.powers,
                hearthstone_slots: (0..self.hearthstone_slots).fold(Vec::new(), |mut acc, _| {
                    acc.push(None);
                    acc
                }),
                merit_dots: self.merit_dots,
                magic_material: self.magic_material,
                attunement_cost: self.attunement_cost,
            }),
        }
    }
}
