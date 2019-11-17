use crate::game::entities::items::ItemId;
use crate::game::entities::{CharacterEvent, Entity};
use crate::game::{Event, Events, GameParts};
use generational_arena::Index;

#[derive(Clone, Copy, Debug)]
pub enum StatChangeCause {
    ItemEffect(ItemId, usize),
    PassiveRestoration,
    EntityDamage(Index),
    LevelUp,
}

#[derive(Clone, Copy, Debug)]
pub enum StatChange {
    Add(StatId, i32, StatChangeCause),
    Sub(StatId, i32, StatChangeCause),
}

#[derive(Clone, Copy, Debug)]
pub enum ModifyEvent {
    Add(StatId, i32, StatChangeCause),
    Sub(StatId, i32, StatChangeCause),
    ZeroReached(StatId, i32, StatChangeCause),
    MaxReached(StatId, i32, StatChangeCause),
}

#[derive(Debug, Clone, Copy)]
pub enum StatId {
    Health,
    Stamina,
    Strength,
    Dexerity,
    Fortitude,
    Mana,
    Willpower,
    Skill,
    Experience,
}

impl StatId {
    pub fn event_change_for(
        self,
        events: &mut Events,
        target: Index,
        amount: i32,
        cause: StatChangeCause,
    ) {
        if amount < 0 {
            events.push(
                3,
                Event::ChangeStat(target, StatChange::Sub(self, amount, cause)),
            );
        } else {
            events.push(
                3,
                Event::ChangeStat(target, StatChange::Add(self, amount, cause)),
            );
        }
    }
    pub fn get_val(self, parts: &GameParts, target: &Index) -> i32 {
        *parts.get_entity(target).get_stat(&self).val()
    }
}

pub struct StatSuite {
    hp: Stat,
    stam: Stat,
    str: Stat,
    dex: Stat,
    fort: Stat,

    mp: Stat,
    will: Stat,

    skill_rating: Stat,
    exp: Stat,
}

impl StatSuite {
    pub fn new() -> StatSuite {
        StatSuite {
            hp: Stat::new(10),
            stam: Stat::new(100),
            str: Stat::new(8),
            dex: Stat::new(8),
            fort: Stat::new(3),

            mp: Stat::new(0),
            will: Stat::new(3),

            skill_rating: Stat::new(1),
            exp: Stat::new_disharmonic(0, 15),
        }
    }

    pub fn change(&mut self, id: &Index, change: StatChange, events: &mut Events) {
        events.character(
            id,
            CharacterEvent::ModifyStat(match change {
                StatChange::Add(stat, value, cause) => self.modify(stat, value, cause),
                StatChange::Sub(stat, value, cause) => self.modify(stat, value, cause),
            }),
        );
    }
    pub fn modify(&mut self, id: StatId, v: i32, cause: StatChangeCause) -> ModifyEvent {
        self.get_mut(&id).modify(id, v, cause)
    }
    pub fn get(&self, id: &StatId) -> &Stat {
        match *id {
            StatId::Health => &self.hp,
            StatId::Stamina => &self.stam,
            StatId::Strength => &self.str,
            StatId::Fortitude => &self.fort,
            StatId::Mana => &self.mp,
            StatId::Dexerity => &self.dex,
            StatId::Willpower => &self.will,
            StatId::Skill => &self.skill_rating,
            StatId::Experience => &self.exp,
        }
    }
    pub fn get_mut(&mut self, id: &StatId) -> &mut Stat {
        match *id {
            StatId::Health => &mut self.hp,
            StatId::Stamina => &mut self.stam,
            StatId::Strength => &mut self.str,
            StatId::Fortitude => &mut self.fort,
            StatId::Mana => &mut self.mp,
            StatId::Dexerity => &mut self.dex,
            StatId::Willpower => &mut self.will,
            StatId::Skill => &mut self.skill_rating,
            StatId::Experience => &mut self.exp,
        }
    }
}

pub struct Stat {
    value: i32,
    max_value: i32,
}

impl Stat {
    pub fn new(v: i32) -> Stat {
        return Stat {
            value: v,
            max_value: v,
        };
    }
    pub fn new_disharmonic(v: i32, m: i32) -> Stat {
        Stat {
            value: v,
            max_value: m,
        }
    }
    pub fn val(&self) -> &i32 {
        &self.value
    }
    pub fn max(&self) -> &i32 {
        &self.max_value
    }
    pub fn modify(&mut self, id: StatId, v: i32, cause: StatChangeCause) -> ModifyEvent {
        self.value += v;
        if v > 0 {
            if self.value >= self.max_value {
                let dif = self.max_value - (self.value + v);
                self.value = self.max_value;
                ModifyEvent::MaxReached(id, dif, cause)
            } else {
                ModifyEvent::Add(id, v, cause)
            }
        } else {
            if self.value <= 0 {
                let dif = -v - self.value;
                self.value = 0;
                ModifyEvent::ZeroReached(id, dif, cause)
            } else {
                ModifyEvent::Sub(id, -1 * v, cause)
            }
        }
    }
}
