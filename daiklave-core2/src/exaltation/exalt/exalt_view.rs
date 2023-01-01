use std::collections::HashMap;

use crate::{
    abilities::{AbilityView, SetAbilityError},
    exaltation::sorcery::SorceryViewSwitch,
    martial_arts::{
        AddMartialArtsStyleError, MartialArtsCharmId, MartialArtsStyle, MartialArtsStyleId,
        RemoveMartialArtsStyleError, SetMartialArtsDotsError,
    },
    sorcery::{
        ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SorceryView, SpellId,
        TerrestrialSpell,
    },
    CharacterMutationError,
};

use super::{
    essence::{
        CommitMotesError, EssenceView, MoteCommitmentId, MoteCommitmentView, MotePoolName,
        SetEssenceRatingError, SpendMotesError, UncommitMotesError,
    },
    exalt_type::{solar::SolarView, ExaltTypeView},
    martial_arts::ExaltMartialArtistView,
    sorcery::ExaltSorceryViewSwitch,
    ExaltMemo,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltView<'source> {
    essence: EssenceView<'source>,
    martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtistView<'source>>,
    exalt_type: ExaltTypeView<'source>,
}

impl<'source> ExaltView<'source> {
    pub fn new(
        essence: EssenceView<'source>,
        martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtistView<'source>>,
        exalt_type: ExaltTypeView<'source>,
    ) -> Self {
        Self {
            essence,
            martial_arts_styles,
            exalt_type,
        }
    }

    pub fn as_memo(&self) -> ExaltMemo {
        ExaltMemo::new(
            self.essence.as_memo(),
            self.martial_arts_styles
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            self.exalt_type.as_memo(),
        )
    }

    pub fn exalt_type(&self) -> &ExaltTypeView<'source> {
        &self.exalt_type
    }

    pub fn essence(&self) -> &EssenceView {
        &self.essence
    }

    pub fn essence_mut(&mut self) -> &mut EssenceView<'source> {
        &mut self.essence
    }

    pub fn martial_arts_styles(
        &self,
    ) -> &HashMap<MartialArtsStyleId, ExaltMartialArtistView<'source>> {
        &self.martial_arts_styles
    }

    pub fn martial_arts_styles_mut(
        &mut self,
    ) -> &mut HashMap<MartialArtsStyleId, ExaltMartialArtistView<'source>> {
        &mut self.martial_arts_styles
    }

    pub fn check_spend_motes(
        &self,
        _first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        let total_available = self.essence().motes().peripheral().available()
            + self.essence().motes().personal().available();

        if total_available < amount {
            Err(CharacterMutationError::SpendMotesError(
                SpendMotesError::InsufficientMotes(total_available, amount),
            ))
        } else {
            Ok(())
        }
    }

    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_spend_motes(first, amount)?;

        let (peripheral_spent, personal_spent) = if let MotePoolName::Peripheral = first {
            let peripheral_spent = self.essence().motes().peripheral().available().min(amount);
            let personal_spent = amount - peripheral_spent;
            (peripheral_spent, personal_spent)
        } else {
            let personal_spent = self.essence().motes().personal().available().min(amount);
            let peripheral_spent = amount - personal_spent;
            (peripheral_spent, personal_spent)
        };

        self.essence_mut()
            .motes_mut()
            .peripheral_mut()
            .spend(peripheral_spent)?;
        self.essence_mut()
            .motes_mut()
            .personal_mut()
            .spend(personal_spent)?;
        Ok(self)
    }

    pub fn check_commit_motes(
        &self,
        _id: &MoteCommitmentId,
        _name: &str,
        _first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        let total_available = self.essence().motes().peripheral().available()
            + self.essence().motes().personal().available();

        if total_available < amount {
            Err(CharacterMutationError::CommitMotesError(
                CommitMotesError::InsufficientMotes(total_available, amount),
            ))
        } else {
            Ok(())
        }
    }

    pub fn commit_motes(
        &mut self,
        id: &MoteCommitmentId,
        name: &'source str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_commit_motes(id, name, first, amount)?;
        let (peripheral_committed, personal_committed) = if let MotePoolName::Peripheral = first {
            let peripheral_committed = self.essence().motes().peripheral().available().min(amount);
            let personal_committed = amount - peripheral_committed;
            (peripheral_committed, personal_committed)
        } else {
            let personal_committed = self.essence().motes().personal().available().min(amount);
            let peripheral_committed = amount - personal_committed;
            (peripheral_committed, personal_committed)
        };

        self.essence_mut()
            .motes_mut()
            .peripheral_mut()
            .commit(peripheral_committed)?;
        self.essence_mut()
            .motes_mut()
            .personal_mut()
            .commit(personal_committed)?;
        let commitment = MoteCommitmentView {
            name,
            peripheral: peripheral_committed,
            personal: personal_committed,
        };
        self.essence_mut()
            .motes_mut()
            .commitments_mut()
            .insert(*id, commitment);
        Ok(self)
    }

    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        let peripheral_recovered = self.essence().motes().peripheral().spent().min(amount);
        let personal_recovered = self
            .essence()
            .motes()
            .personal()
            .spent()
            .min(amount - peripheral_recovered);

        self.essence_mut()
            .motes_mut()
            .peripheral_mut()
            .recover(peripheral_recovered)?;
        self.essence_mut()
            .motes_mut()
            .personal_mut()
            .recover(personal_recovered)?;
        Ok(self)
    }

    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        if !self.essence().motes().commitments().contains_key(id) {
            Err(CharacterMutationError::UncommitMotesError(
                UncommitMotesError::NotFound(*id),
            ))
        } else {
            Ok(())
        }
    }

    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
    ) -> Result<&mut Self, CharacterMutationError> {
        let commitment = self
            .essence_mut()
            .motes_mut()
            .commitments_mut()
            .remove(id)
            .ok_or({
                CharacterMutationError::UncommitMotesError(UncommitMotesError::NotFound(*id))
            })?;
        self.essence_mut()
            .motes_mut()
            .peripheral_mut()
            .uncommit(commitment.peripheral)
            .unwrap();
        self.essence_mut()
            .motes_mut()
            .personal_mut()
            .uncommit(commitment.personal)
            .unwrap();
        Ok(self)
    }

    pub fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        if self.essence().rating() == rating {
            return Ok(self);
        }

        if !(1..=5).contains(&rating) {
            return Err(CharacterMutationError::SetEssenceRatingError(
                SetEssenceRatingError::InvalidRating(rating),
            ));
        }

        let (new_peripheral, new_personal) = match self.exalt_type {
            ExaltTypeView::Solar(_) => (rating * 7 + 26, rating * 3 + 10),
        };

        let committed_ids = self
            .essence()
            .motes()
            .committed()
            .map(|x| x.0)
            .collect::<Vec<MoteCommitmentId>>();
        for id in committed_ids {
            self.uncommit_motes(&id).unwrap();
        }

        let spent_peripheral = self.essence().motes().peripheral().spent();
        self.essence_mut()
            .motes_mut()
            .peripheral_mut()
            .recover(spent_peripheral)
            .unwrap();
        let available_peripheral = self.essence().motes().peripheral().available();
        if available_peripheral < new_peripheral {
            self.essence_mut()
                .motes_mut()
                .peripheral_mut()
                .uncommit(new_peripheral - available_peripheral)
                .unwrap()
                .recover(new_peripheral - available_peripheral)
                .unwrap();
        } else {
            self.essence_mut()
                .motes_mut()
                .peripheral_mut()
                .commit(available_peripheral - new_peripheral)
                .unwrap();
        }

        let spent_personal = self.essence().motes().personal().spent();
        self.essence_mut()
            .motes_mut()
            .personal_mut()
            .recover(spent_personal)
            .unwrap();
        let available_personal = self.essence().motes().personal().available();
        if available_personal < new_personal {
            self.essence_mut()
                .motes_mut()
                .personal_mut()
                .uncommit(new_personal - available_personal)
                .unwrap()
                .recover(new_personal - available_personal)
                .unwrap();
        } else {
            self.essence_mut()
                .motes_mut()
                .peripheral_mut()
                .commit(available_personal - new_personal)
                .unwrap();
        }

        self.essence_mut().rating = rating;

        Ok(self)
    }

    pub fn is_solar(&self) -> bool {
        self.exalt_type.is_solar()
    }

    pub fn solar_traits(&self) -> Option<&SolarView> {
        self.exalt_type.solar_traits()
    }

    pub(crate) fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        _style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::AddMartialArtsStyleError(
                AddMartialArtsStyleError::DuplicateStyle,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &'source MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.martial_arts_styles.insert(
            id,
            ExaltMartialArtistView::new(style, AbilityView::Zero, HashMap::new()),
        );
        Ok(self)
    }

    pub(crate) fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        if self.martial_arts_styles.contains_key(&id) {
            Err(CharacterMutationError::RemoveMartialArtsStyleError(
                RemoveMartialArtsStyleError::NotFound,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_martial_arts_style(id)?;
        self.martial_arts_styles.remove(&id);
        Ok(self)
    }

    pub(crate) fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(
                SetAbilityError::InvalidRating(dots),
            ))
        } else if self.martial_arts_styles.contains_key(&id) {
            Ok(())
        } else {
            Err(CharacterMutationError::SetMartialArtsDotsError(
                SetMartialArtsDotsError::NotFound,
            ))
        }
    }

    pub(crate) fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(
                SetAbilityError::InvalidRating(dots),
            ))
        } else if let Some(style) = self.martial_arts_styles.get_mut(&id) {
            if dots < style.ability().dots() {
                // May have to remove charms
                let mut prereq_charms_map =
                    HashMap::<MartialArtsCharmId, Vec<MartialArtsCharmId>>::new();
                let mut removal_stack = Vec::<MartialArtsCharmId>::new();

                for (charm_id, charm) in style.charms() {
                    for prereq_charm_id in charm.charms_required() {
                        prereq_charms_map
                            .entry(prereq_charm_id)
                            .or_default()
                            .push(charm_id);
                    }

                    if charm.ability_required() > dots {
                        removal_stack.push(charm_id);
                    }
                }

                while let Some(id_to_remove) = removal_stack.pop() {
                    style.charms_mut().remove(&id_to_remove);
                    if let Some(dependents) = prereq_charms_map.remove(&id_to_remove) {
                        for dependent_id in dependents.iter() {
                            removal_stack.push(*dependent_id);
                        }
                    }
                }
            }
            style.ability_mut().set_dots(dots)?;
            Ok(self)
        } else {
            Err(CharacterMutationError::SetMartialArtsDotsError(
                SetMartialArtsDotsError::NotFound,
            ))
        }
    }

    pub fn add_terrestrial_sorcery(
        &mut self,
        archetype_id: SorceryArchetypeId,
        archetype: &'source SorceryArchetype,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        control_spell_id: SpellId,
        control_spell: &'source TerrestrialSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.exalt_type {
            ExaltTypeView::Solar(solar) => {
                solar.add_terrestrial_sorcery(
                    archetype_id,
                    archetype,
                    shaping_ritual_id,
                    shaping_ritual,
                    control_spell_id,
                    control_spell,
                )?;
            }
        }
        Ok(self)
    }
}

impl<'view, 'source> ExaltView<'source> {
    pub(crate) fn sorcery(&'view self) -> Option<SorceryView<'view, 'source>> {
        match self.exalt_type() {
            ExaltTypeView::Solar(solar) => solar.sorcery().map(|sorcerer| {
                SorceryView(SorceryViewSwitch::Exalt(ExaltSorceryViewSwitch::Solar(
                    sorcerer,
                )))
            }),
        }
    }
}
