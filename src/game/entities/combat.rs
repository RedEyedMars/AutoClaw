use crate::game::battle::system::movement::MovementPath;
use crate::game::entities::skills::Skill;
use crate::game::entities::traits::TraitId;
use crate::game::entities::{Change, Entity, Idable, State};
use crate::game::{Event, Events, GameParts};
use generational_arena::Index;

#[derive(Debug, Clone, Copy)]
pub enum CombatStance {
    FindingTarget,
    MovingCloser(Index, MovementPath),
    UsingSkill(Skill, Index),
}

impl CombatStance {
    pub fn pump<'a>(&self, entity: &Entity, parts: &GameParts, events: &mut Events) {
        if let Some(board) = parts.get_board() {
            match self {
                CombatStance::FindingTarget => {
                    if let Some(target) = board.get_target(entity, parts) {
                        events.push(
                            2,
                            Event::ChangeEntity(
                                entity.id(),
                                Change::State(State::Fighting {
                                    stance: CombatStance::UsingSkill(target.1, target.0.id()),
                                }),
                            ),
                        );
                    } else {
                        if entity.has_trait(&TraitId::Priest) {
                            if let Some(target) = board.get_closest_ally(entity.id(), parts) {
                                events.push(
                                    3,
                                    Event::ChangeEntity(
                                        entity.id(),
                                        Change::State(State::Fighting {
                                            stance: CombatStance::MovingCloser(
                                                target,
                                                board.get_path_to_target(
                                                    entity.id(),
                                                    target,
                                                    parts,
                                                ),
                                            ),
                                        }),
                                    ),
                                );
                            }
                        } else {
                            if let Some(target) = board.get_closest_enemy(entity.id(), parts) {
                                events.push(
                                    3,
                                    Event::ChangeEntity(
                                        entity.id(),
                                        Change::State(State::Fighting {
                                            stance: CombatStance::MovingCloser(
                                                target,
                                                board.get_path_to_target(
                                                    entity.id(),
                                                    target,
                                                    parts,
                                                ),
                                            ),
                                        }),
                                    ),
                                );
                            }
                        }
                    }
                }
                CombatStance::UsingSkill(skill, target) => {
                    skill.act(parts, events, entity.id(), *target);
                    events.push(
                        2,
                        Event::ChangeEntity(
                            entity.id(),
                            Change::State(State::Fighting {
                                stance: CombatStance::FindingTarget,
                            }),
                        ),
                    );
                }
                CombatStance::MovingCloser(target, path) => {
                    if let Some(new_path) = board.move_entity(entity.id(), path, events) {
                        events.push(
                            2,
                            Event::ChangeEntity(
                                entity.id(),
                                Change::State(State::Fighting {
                                    stance: CombatStance::MovingCloser(*target, new_path),
                                }),
                            ),
                        );
                    } else {
                        if let Some(target) = board.get_target(entity, parts) {
                            events.push(
                                2,
                                Event::ChangeEntity(
                                    entity.id(),
                                    Change::State(State::Fighting {
                                        stance: CombatStance::UsingSkill(target.1, target.0.id()),
                                    }),
                                ),
                            );
                        } else {
                            events.push(
                                2,
                                Event::ChangeEntity(
                                    entity.id(),
                                    Change::State(State::Fighting {
                                        stance: CombatStance::FindingTarget,
                                    }),
                                ),
                            );
                        }
                    }
                }
            }
        }
    }
}
