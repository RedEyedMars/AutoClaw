use crate::game::battle::system::board::{Board, Tile};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MovementPath {
    Zero,
    One(Direction),
    Two(Direction, Direction),
    Three(Direction, Direction, Direction),
    Four(Direction, Direction, Direction, Direction),
}

impl MovementPath {
    pub fn append(&self, new_val: Direction) -> MovementPath {
        match *self {
            MovementPath::Zero => MovementPath::One(new_val),
            MovementPath::One(first_val) => MovementPath::Two(first_val, new_val),
            MovementPath::Two(first_val, second_val) => {
                MovementPath::Three(first_val, second_val, new_val)
            }
            MovementPath::Three(first_val, second_val, third_val) => {
                MovementPath::Four(first_val, second_val, third_val, new_val)
            }
            MovementPath::Four(first_val, second_val, third_val, fourth_val) => {
                MovementPath::Four(first_val, second_val, third_val, fourth_val)
            }
        }
    }
    pub fn last(&self) -> Option<Direction> {
        match *self {
            MovementPath::Zero => None,
            MovementPath::One(last) => Some(last),
            MovementPath::Two(_, last) => Some(last),
            MovementPath::Three(_, _, last) => Some(last),
            MovementPath::Four(_, _, _, last) => Some(last),
        }
    }
    pub fn first(&self) -> Option<Direction> {
        match *self {
            MovementPath::Zero => None,
            MovementPath::One(dir) => Some(dir),
            MovementPath::Two(dir, _) => Some(dir),
            MovementPath::Three(dir, _, _) => Some(dir),
            MovementPath::Four(dir, _, _, _) => Some(dir),
        }
    }
    pub fn pop(&self) -> MovementPath {
        match *self {
            MovementPath::Zero => MovementPath::Zero,
            MovementPath::One(_) => MovementPath::Zero,
            MovementPath::Two(_, a) => MovementPath::One(a),
            MovementPath::Three(_, a, b) => MovementPath::Two(a, b),
            MovementPath::Four(_, a, b, c) => MovementPath::Three(a, b, c),
        }
    }
}
