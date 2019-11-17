use crate::game::entities::stats::{StatChange, StatChangeCause};
use crate::game::entities::Change;
use crate::game::{Event, Events, GameParts, Idable};
use generational_arena::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillId {
    BasicAttack,

    //Rogue
    Backstab,

    //Mage
    FireBall,
}

pub enum SkillRejectionReason<'a> {
    StatTooLow(&'a dyn SkillRequirement, i32),
}

pub trait Skillable: Idable + std::fmt::Debug {
    fn add_skill(&mut self, skill: Skill);
}

pub trait SkillRequirement {
    fn satisfies(&self, learner: &Index, _parts: &GameParts) -> Option<SkillRejectionReason>;
}

pub struct RequireAny;
impl SkillRequirement for RequireAny {
    fn satisfies(&self, _learner: &Index, _parts: &GameParts) -> Option<SkillRejectionReason> {
        None
    }
}
pub struct EachRequirement<'a> {
    requirements: Vec<&'a dyn SkillRequirement>,
}
impl SkillRequirement for EachRequirement<'_> {
    fn satisfies(&self, learner: &Index, parts: &GameParts) -> Option<SkillRejectionReason> {
        (&self.requirements)
            .into_iter()
            .map(|r| r.satisfies(learner, parts))
            .fold(None, |acc, r| match acc {
                None => r,
                Some(reason) => Some(reason),
            })
    }
}
pub struct AlternativeRequirement<'a> {
    requirements: Vec<&'a dyn SkillRequirement>,
}
impl SkillRequirement for AlternativeRequirement<'_> {
    fn satisfies(&self, learner: &Index, parts: &GameParts) -> Option<SkillRejectionReason> {
        (&self.requirements)
            .into_iter()
            .map(|r| r.satisfies(learner, parts))
            .fold(None, |acc, r| match r {
                None => None,
                Some(reason) => match acc {
                    None => Some(reason),
                    Some(other_reason) => Some(other_reason),
                },
            })
    }
}
use crate::game::entities::stats::StatId;
pub struct RequireStat {
    stat: StatId,
    required: i32,
}
impl SkillRequirement for RequireStat {
    fn satisfies(&self, learner: &Index, parts: &GameParts) -> Option<SkillRejectionReason> {
        let dif = *parts.get_entity(learner).get_stat(&self.stat).val() - self.required;
        if dif < 0 {
            Some(SkillRejectionReason::StatTooLow(self, dif))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Skill {
    BasicAttack,
    Backstab { dmg: i32 },
    FireBall { dmg: i32 },
}

pub fn add_skill<'a>(id: &SkillId, learner: &Index, parts: &mut GameParts) {
    if can_learn(id, parts.get_entity(learner)) {
        parts.add_skill(learner, create_skill(parts, id, learner));
    }
}
pub fn can_learn<'a, 'b>(id: &SkillId, _learner: &'a dyn Skillable) -> bool {
    match id {
        SkillId::BasicAttack => true,
        SkillId::Backstab => true,
        SkillId::FireBall => true,
    }
}
pub fn create_skill(parts: &GameParts, id: &SkillId, entity: &Index) -> Skill {
    match *id {
        SkillId::BasicAttack => Skill::BasicAttack,
        SkillId::Backstab => Skill::Backstab {
            dmg: parts.get_entity(entity).get_stat(&StatId::Strength).val()
                + parts.get_entity(entity).get_stat(&StatId::Dexerity).val() / 3,
        },
        SkillId::FireBall => Skill::FireBall {
            dmg: StatId::Willpower.get_val(parts, entity)
                + parts.get_entity(entity).get_stat(&StatId::Skill).val() / 3,
        },
    }
}
impl Skill {
    pub fn act(&self, parts: &GameParts, events: &mut Events, caster: Index, target: Index) {
        match self {
            Skill::BasicAttack => {
                StatId::Health.event_change_for(
                    events,
                    target,
                    StatId::Strength.get_val(parts, &caster),
                    StatChangeCause::EntityDamage(caster),
                );
                events.push(
                    5,
                    Event::ChangeEntity(caster, Change::SkillExp(SkillId::BasicAttack, 1)),
                );
            }
            Skill::Backstab { dmg: _ } => (),
            Skill::FireBall { dmg: _ } => (),
        }
    }

    pub fn can_target_self(&self) -> bool {
        match self {
            Skill::BasicAttack => false,
            Skill::Backstab { dmg: _ } => false,
            Skill::FireBall { dmg: _ } => false,
        }
    }

    pub fn can_target(&self, distance: f64, _id: Index, _parts: &GameParts) -> bool {
        match self {
            Skill::BasicAttack => distance <= 1f64,
            Skill::Backstab { dmg: _ } => distance <= 1f64,
            Skill::FireBall { dmg: _ } => distance <= 3f64,
        }
    }
}
