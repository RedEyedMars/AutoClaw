pub mod battle;
pub mod entities;

extern crate generational_arena;
use generational_arena::{Arena, Index};

use std::collections::HashMap;

use crate::game::battle::system::board::{Board, BoardChange};
use crate::game::entities::skills::Skillable;
use crate::game::entities::stats::{Stat, StatChange};
use crate::game::entities::stats::{StatChangeCause, StatId, StatSuite};
use crate::game::entities::{Change, CharacterEvent, Entity, State};
use crate::gui::animation::img::Img;
use crate::gui::resources::Resources;

pub trait Idable {
    fn id(&self) -> Index;
}
#[derive(Clone, Copy)]
pub enum Event {
    ChangeEntity(Index, Change),
    ChangeStat(Index, StatChange),
    ChangeBoard(BoardChange),
}
#[derive(Clone)]
pub struct Events {
    events: (Vec<Event>, Vec<Event>, Vec<Event>, Vec<Event>, Vec<Event>),
    character_events: HashMap<Index, Vec<CharacterEvent>>,
}
impl Events {
    pub fn new() -> Events {
        Events {
            events: (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            character_events: HashMap::new(),
        }
    }
    pub fn push(&mut self, pri: u8, event: Event) {
        match pri {
            1 => self.events.0.push(event),
            2 => self.events.1.push(event),
            3 => self.events.2.push(event),
            4 => self.events.3.push(event),
            5 => self.events.4.push(event),
            _ => unimplemented!(),
        }
    }
    pub fn character(&mut self, id: &Index, event: CharacterEvent) {
        if let Some(events) = self.character_events.get_mut(id) {
            events.push(event);
        } else {
            let mut vec = Vec::new();
            vec.push(event);
            self.character_events.insert(*id, vec);
        }
    }

    pub fn step(&mut self, parts: &mut GameParts) -> Events {
        let mut new_events = Events::new();
        let mut handled = 1;
        let mut previously_handled = 0;

        let mut index = (0, 0, 0, 0, 0);
        while handled > previously_handled && handled < 50 {
            previously_handled = handled;
            if index.0 == 0 && self.events.0.len() != 0 {
                handled += self.events.0.len();
                index.0 = self.events.0.len();
                for event in self.events.0.iter() {
                    parts.push(*event, &mut new_events);
                }
            } else {
                if index.1 < self.events.1.len() {
                    if index.1 + 50 < self.events.1.len() {
                        for i in index.1..index.1 + 50 {
                            parts.push(self.events.1[i], &mut new_events);
                        }
                        handled += 50;
                    } else {
                        for i in index.1..self.events.1.len() {
                            parts.push(self.events.1[i], &mut new_events);
                        }
                        handled += self.events.1.len() - index.1;
                    }
                }
            }
        }
        new_events
    }
}

pub struct GameParts {
    board: Option<Board>,
    entities: Arena<Entity>,
}
impl GameParts {
    pub fn tick<'a>(&mut self, mut events: Events) -> Events {
        let mut new_events = events.step(self);
        for (_, entity) in self.entities.iter() {
            entity.pump(
                match events.character_events.get_mut(&entity.id()) {
                    Some(cevs) => {
                        cevs.push(CharacterEvent::Tick);
                        cevs.to_vec()
                    }
                    None => vec![CharacterEvent::Tick],
                },
                &mut new_events,
            );
        }
        new_events
    }
    pub fn push(&mut self, step: Event, new_events: &mut Events) {
        match step {
            Event::ChangeEntity(id, change) => self.entities[id].change(change, new_events),
            Event::ChangeStat(id, change) => self.entities[id].change_stat(&id, change, new_events),
            Event::ChangeBoard(change) => {
                if let Some(b) = &mut self.board {
                    b.change(change, new_events)
                }
            }
        }
    }
    pub fn get_entity(&self, id: &Index) -> &Entity {
        &self.entities[*id]
    }
    pub fn get_board(&self) -> &Option<Board> {
        &self.board
    }
    pub fn modify_stat<'a>(
        &mut self,
        events: &mut Events,
        stater: &Index,
        stat: StatId,
        val: i32,
        cause: StatChangeCause,
    ) {
        events.character(
            stater,
            CharacterEvent::ModifyStat(self.entities[*stater].modify_stat(stat, val, cause)),
        );
    }
    pub fn add_skill(&mut self, skiller: &Index, skill: crate::game::entities::skills::Skill) {
        self.entities[*skiller].add_skill(skill);
    }
}
pub struct GameLoop {
    //executors: Vec<Box<dyn Executable<'b>>>,
    parts: GameParts,

    tick_debt: u128,
}
impl GameLoop {
    pub fn new() -> GameLoop {
        GameLoop {
            //executors: Vec::new(),
            parts: GameParts {
                board: None,
                entities: Arena::new(),
            },

            tick_debt: 0u128,
        }
    }

    pub fn execute<'a>(game: &mut GameLoop, ticks: &u128, events: Events) -> Events {
        game.tick_debt += *ticks;
        if game.tick_debt > 10 {
            GameLoop::execute_loop(game, 5, events)
        } else if game.tick_debt > 0 {
            GameLoop::execute_loop(game, game.tick_debt + 1, events)
        } else {
            events
        }
    }
    fn execute_loop<'a>(game: &mut GameLoop, ticks: u128, mut events: Events) -> Events {
        if ticks == 0 {
            return events;
        }

        for _ in 0..ticks {
            events = GameParts::tick(&mut game.parts, events);
            if game.tick_debt > 0 {
                game.tick_debt -= 1;
            }
        }
        events
    }

    pub fn add_entity(
        &mut self,
        events: &mut Events,
        res: &Resources,
        gl: &gl::Gl,
    ) -> Result<(), failure::Error> {
        let entity = Entity::new(StatSuite::new(), Img::new(res, gl)?);
        let id = self.parts.entities.insert(entity);
        self.parts.entities[id].set_id(id);
        events.push(1, Event::ChangeEntity(id, Change::State(State::Birth)));
        Ok(())
    }
}
