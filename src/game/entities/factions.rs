#[derive(Hash, PartialEq, Eq)]
pub enum FactionId {
    Lawless,

    Feline,

    Rat,
    Duck,
    Arachine,
}

pub trait Factionable {
    fn faction(&self) -> &FactionId;
    fn belongs(&self, other: &FactionId) -> bool {
        *(self.faction()) == *other
    }
    fn allied(&self, other: &dyn Factionable) -> bool {
        *(self.faction()) == *(other.faction())
    }
}
