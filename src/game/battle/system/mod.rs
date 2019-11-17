use crate::game::battle::system::board::*;
use crate::game::entities::traits::TraitId;
use crate::game::{Events, GameParts};
use generational_arena::Index;

pub mod board;
pub mod movement;

pub struct BoardAnalyzer {}
impl BoardAnalyzer {
    pub fn pump(_board: &mut Board, _events: &mut Events) {}
}

pub trait CombatantRequirement {
    fn satisfies(&self, parts: &GameParts, candidate: &Index) -> bool;
}

pub struct AnyCombatant;
impl CombatantRequirement for AnyCombatant {
    fn satisfies(&self, _parts: &GameParts, _candidate: &Index) -> bool {
        true
    }
}

pub struct EachCombatant<'a> {
    requirements: Vec<&'a dyn CombatantRequirement>,
}
impl CombatantRequirement for EachCombatant<'_> {
    fn satisfies(&self, parts: &GameParts, candidate: &Index) -> bool {
        !self
            .requirements
            .iter()
            .any(|requirement| !requirement.satisfies(parts, candidate))
    }
}
pub struct AlternativeCombatant<'a> {
    requirements: Vec<&'a dyn CombatantRequirement>,
}
impl CombatantRequirement for AlternativeCombatant<'_> {
    fn satisfies(&self, parts: &GameParts, candidate: &Index) -> bool {
        self.requirements
            .iter()
            .any(|requirement| requirement.satisfies(parts, candidate))
    }
}
pub struct CombatantTraitRequirement<'a> {
    trt: &'a TraitId,
}
impl CombatantRequirement for CombatantTraitRequirement<'_> {
    fn satisfies(&self, parts: &GameParts, candidate: &Index) -> bool {
        parts.get_entity(candidate).has_trait(self.trt)
    }
}
