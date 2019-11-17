use crate::game::{Event, Events, Idable};
use crate::gui::animation::img::Img;
use std::collections::hash_map::{Entry, HashMap};
use std::collections::HashSet;

use generational_arena::{Arena, Index};

pub mod combat;
pub mod factions;
pub mod items;
pub mod skills;
pub mod stats;
pub mod traits;

#[derive(Clone, Copy)]
pub enum Change {
    AddTrait(traits::TraitId),
    State(State),
    SkillExp(skills::SkillId, u32),
}
#[derive(Debug, Clone, Copy)]
pub enum Cause {
    Agent(usize, u32),
    Natural(NaturalCause),
}
#[derive(Debug, Clone, Copy)]
pub enum NaturalCause {}
#[derive(Debug, Clone, Copy)]
pub enum CharacterEvent {
    Tick,
    Birth,
    GainedTrait(traits::TraitId),
    FailedToGainTrait(traits::TraitId),
    LevelUp(u32),
    LevelDown(u32),
    ModifyStat(stats::ModifyEvent),
    SkillUp(skills::SkillId, u32),
    Death(Cause),
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Pupa,
    Birth,
    Shopping,
    Traveling,
    Fighting { stance: combat::CombatStance },
    Dead(Cause),
}

pub struct Entity {
    id: Option<Index>,
    state: State,
    img: Img,
    stats: stats::StatSuite,
    faction_id: factions::FactionId,
    traits: HashSet<traits::TraitId>,
    skills: Vec<skills::Skill>,
    skill_exp: HashMap<skills::SkillId, u32>,
}
impl Entity {
    pub fn new(stats: stats::StatSuite, img: Img) -> Entity {
        Entity {
            id: None,
            stats: stats,
            img: img,
            state: State::Pupa,
            faction_id: factions::FactionId::Lawless,
            traits: HashSet::new(),
            skills: Vec::new(),
            skill_exp: HashMap::new(),
        }
    }
    pub fn set_id(&mut self, id: Index) {
        self.id = Some(id);
    }

    pub fn has_trait(&self, trt: &traits::TraitId) -> bool {
        self.traits.contains(trt)
    }
    pub fn gain_trait(&mut self, trt: traits::TraitId, events: &mut Events) {
        if trt.can_gain(self) {
            trt.on_gain(self, events);
            self.traits.insert(trt);
            events.character(&self.id(), CharacterEvent::GainedTrait(trt));
        } else {
            events.character(&self.id(), CharacterEvent::FailedToGainTrait(trt));
        }
    }
    pub fn set_state<'a>(&mut self, state: State, events: &mut Events) {
        self.state = state;
        match state {
            State::Birth => events.character(&self.id(), CharacterEvent::Birth),
            State::Dead(cause) => events.character(&self.id(), CharacterEvent::Death(cause)),
            _ => {}
        }
    }
    pub fn change_skill_exp(
        &mut self,
        skill_id: skills::SkillId,
        amount: u32,
        events: &mut Events,
    ) {
        let mut xp_added = false;
        if let Entry::Occupied(mut skill) = self.skill_exp.entry(skill_id) {
            skill.insert(skill.get() + amount);
            xp_added = true;
        }
        if xp_added {
            events.character(&self.id(), CharacterEvent::SkillUp(skill_id, amount));
        }
    }

    pub fn pump<'a>(&self, char_events: Vec<CharacterEvent>, new_events: &mut Events) {
        for char_event in char_events {
            for trt in self.traits.iter() {
                match trt.handle(self, char_event) {
                    Some(change) => new_events.push(2, Event::ChangeEntity(self.id(), change)),
                    None => (),
                }
            }
        }
    }
    pub fn change<'a>(&mut self, change: Change, events: &mut Events) {
        match change {
            Change::AddTrait(trt) => self.gain_trait(trt, events),
            Change::State(state) => self.set_state(state, events),
            Change::SkillExp(skill, amount) => self.change_skill_exp(skill, amount, events),
        }
    }
    pub fn change_stat(&mut self, id: &Index, change: stats::StatChange, events: &mut Events) {
        self.stats.change(id, change, events);
    }
    pub fn modify_stat(
        &mut self,
        id: stats::StatId,
        v: i32,
        cause: stats::StatChangeCause,
    ) -> stats::ModifyEvent {
        self.stats.modify(id, v, cause)
    }
    pub fn get_skills(&self) -> &Vec<skills::Skill> {
        &self.skills
    }
    pub fn get_stat(&self, stat: &stats::StatId) -> &stats::Stat {
        &self.stats.get(stat)
    }
}

//impl <'b> crate::game::Executable<'b> for Entity<'b> {

//}
impl Idable for Entity {
    fn id(&self) -> Index {
        match self.id {
            Some(id) => id,
            None => unimplemented!(),
        }
    }
}

impl skills::Skillable for Entity {
    fn add_skill(&mut self, skill: skills::Skill) {
        self.skills.push(skill);
    }
}
impl traits::Traitable for Entity {
    fn has_trait<'a>(&self, id: &'a traits::TraitId) -> bool {
        self.traits.contains(id)
    }
}

impl factions::Factionable for Entity {
    fn faction(&self) -> &factions::FactionId {
        &self.faction_id
    }
}

impl std::fmt::Debug for Entity {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "{:?}", "Hi")
    }
}
