use crate::game::entities::{Change, CharacterEvent, Entity};
use crate::game::Events;

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TraitId {
    Crusader,
    Mage,
    Priest,
    Ranger,
    Bard,
}

pub trait Traitable {
    fn has_trait<'b>(&self, id: &'b TraitId) -> bool;
}

impl TraitId {
    pub fn can_gain(self, entity: &Entity) -> bool {
        match self {
            TraitId::Crusader => !entity.has_trait(&TraitId::Crusader),
            TraitId::Mage => !entity.has_trait(&TraitId::Mage),
            TraitId::Priest => !entity.has_trait(&TraitId::Priest),
            TraitId::Ranger => !entity.has_trait(&TraitId::Ranger),
            TraitId::Bard => !entity.has_trait(&TraitId::Bard),
        }
    }

    pub fn on_gain<'a>(self, _entity: &Entity, _events: &mut Events) {
        match self {
            TraitId::Crusader => (),
            TraitId::Mage => (),
            TraitId::Priest => (),
            TraitId::Ranger => (),
            TraitId::Bard => (),
        }
    }
    pub fn handle<'a>(&self, _entity: &Entity, _event: CharacterEvent) -> Option<Change> {
        None
    }
}

/*
pub struct Knight;

impl ClassTrait for Knight {

        fn get_name<'a>(&'a self)->&'a &str{
            "Knight".into_string()
        }

        fn handle<'a>(&self, handlee:&mut Entity,event: &CharacterEvent) -> CharacterChange{
            CharacterChange::None
        }

        fn can_gain(&self, candidate:&Entity) -> bool{
            true
        }
        fn gain(&self, gainee:&mut Entity){

        }
        fn remove(&self, removee:&mut Entity){

        }
}

pub fn create_trait<T: ClassTrait>(name:str)-> T where T:std::marker::Sized {
    match name.into_string() {
        "Knight" => Knight{},
    }
}
*/
