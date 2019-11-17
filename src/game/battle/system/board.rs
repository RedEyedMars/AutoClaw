use std::collections::HashMap;

use crate::game::battle::system::movement::{Direction, MovementPath};
use crate::game::battle::system::CombatantRequirement;
use crate::game::entities::factions::{FactionId, Factionable};
use crate::game::entities::skills::Skill;
use crate::game::entities::Entity;
use crate::game::{Event, Events, GameParts, Idable};
use generational_arena::Index;

#[derive(Clone, Copy)]
pub enum BoardChange {
    MoveEntity(Index, (usize, usize), (usize, usize)),
}

//Current theory:
//Each tile is a triangle, housing 1 unit, that can traverse through the tiles
#[derive(PartialEq, Eq, Clone)]
pub struct Tile {
    face_up: bool,
    x: usize,
    y: usize,
    enhabitant: Option<Index>,
}
impl Tile {
    pub fn new(face: bool, x: usize, y: usize) -> Tile {
        Tile {
            face_up: face,
            x: x,
            y: y,
            enhabitant: None,
        }
    }
    pub fn get(&self) -> Option<Index> {
        self.enhabitant
    }
    pub fn set(&mut self, combatant: Option<Index>) {
        self.enhabitant = combatant;
    }
    pub fn set_if_none(&mut self, combatant: Option<Index>) -> bool {
        match self.enhabitant {
            None => {
                self.enhabitant = combatant;
                true
            }
            Some(_) => false,
        }
    }
    pub fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    pub fn get_from_dir<'a>(&'a self, board: &'a Board, dir: Direction) -> Option<&'a Tile> {
        match dir {
            Direction::Down => self.down(&board.tiles),
            Direction::Left => self.left(&board.tiles),
            Direction::Up => self.up(&board.tiles),
            Direction::Right => self.right(&board.tiles),
        }
    }
    pub fn get_mut_from_dir<'a>(
        &'a self,
        board: &'a mut Board,
        dir: Direction,
    ) -> Option<&'a mut Tile> {
        match dir {
            Direction::Down => self.down_mut(&mut board.tiles),
            Direction::Left => self.left_mut(&mut board.tiles),
            Direction::Up => self.up_mut(&mut board.tiles),
            Direction::Right => self.right_mut(&mut board.tiles),
        }
    }
    pub fn left<'a>(&self, tiles: &'a Vec<Vec<Tile>>) -> Option<&'a Tile> {
        if self.x > 0 {
            Some(&tiles[self.y][self.x - 1])
        } else {
            None
        }
    }
    pub fn right<'a>(&self, tiles: &'a Vec<Vec<Tile>>) -> Option<&'a Tile> {
        if self.x < tiles[0].len() - 1 {
            Some(&tiles[self.y][self.x + 1])
        } else {
            None
        }
    }
    pub fn up<'a>(&self, tiles: &'a Vec<Vec<Tile>>) -> Option<&'a Tile> {
        if self.y < tiles.len() && (self.x + self.y % 2) % 2 == 0 {
            Some(&tiles[self.y + 1][self.x])
        } else {
            None
        }
    }
    pub fn down<'a>(&self, tiles: &'a Vec<Vec<Tile>>) -> Option<&'a Tile> {
        if self.y > 0 && (self.x + 1 + self.y % 2) % 2 == 0 {
            Some(&tiles[self.y - 1][self.x])
        } else {
            None
        }
    }
    pub fn left_mut<'a>(&self, tiles: &'a mut Vec<Vec<Tile>>) -> Option<&'a mut Tile> {
        if self.x > 0 {
            Some(&mut tiles[self.y][self.x - 1])
        } else {
            None
        }
    }
    pub fn right_mut<'a>(&self, tiles: &'a mut Vec<Vec<Tile>>) -> Option<&'a mut Tile> {
        if self.x < tiles[0].len() - 1 {
            Some(&mut tiles[self.y][self.x + 1])
        } else {
            None
        }
    }
    pub fn up_mut<'a>(&self, tiles: &'a mut Vec<Vec<Tile>>) -> Option<&'a mut Tile> {
        if self.y < tiles.len() && (self.x + self.y % 2) % 2 == 0 {
            Some(&mut tiles[self.y + 1][self.x])
        } else {
            None
        }
    }
    pub fn down_mut<'a>(&self, tiles: &'a mut Vec<Vec<Tile>>) -> Option<&'a mut Tile> {
        if self.y > 0 && (self.x + 1 + self.y % 2) % 2 == 0 {
            Some(&mut tiles[self.y - 1][self.x])
        } else {
            None
        }
    }
}
pub struct ConstructionPart((usize, usize), Option<Index>);
pub struct Board {
    tiles: Vec<Vec<Tile>>,
    flat: Vec<(usize, usize)>,

    width: usize,
    height: usize,

    combatants: HashMap<Index, (usize, usize)>,
    allies: HashMap<FactionId, Vec<Index>>,
    foes: HashMap<FactionId, Vec<Index>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let mut tiles = Vec::new();
        let mut face = false;
        let mut flat = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                let tile = Tile::new(face, x, y);
                flat.push((x, y));
                row.push(tile);
                face = !face;
            }
            tiles.push(row);
        }

        let mut allies = HashMap::new();
        allies.insert(FactionId::Feline, Vec::new());
        allies.insert(FactionId::Rat, Vec::new());
        allies.insert(FactionId::Duck, Vec::new());
        allies.insert(FactionId::Arachine, Vec::new());

        let mut foes = HashMap::new();
        foes.insert(FactionId::Lawless, Vec::new());
        foes.insert(FactionId::Feline, Vec::new());
        foes.insert(FactionId::Rat, Vec::new());
        foes.insert(FactionId::Duck, Vec::new());
        foes.insert(FactionId::Arachine, Vec::new());
        Board {
            tiles: tiles,
            flat: flat,

            height: height,
            width: width,

            combatants: HashMap::new(),
            allies: allies,
            foes: foes,
        }
    }
    pub fn add_parts(&mut self, parts: &GameParts, construction: Vec<ConstructionPart>) {
        for part in construction.iter() {
            match part.1 {
                Some(entity) => {
                    self.combatants
                        .insert(entity, self.get_unsafe(part.0).pos());
                    self.get_mut_unsafe(part.0).set(Some(entity));
                    let faction = parts.get_entity(&entity).faction();
                    if let FactionId::Lawless = *faction {
                    } else {
                        self.allies.get_mut(faction).unwrap().push(entity);
                    }
                    match faction {
                        FactionId::Feline => {
                            self.foes.get_mut(&FactionId::Lawless).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Rat).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Duck).unwrap().push(entity);
                            self.foes
                                .get_mut(&FactionId::Arachine)
                                .unwrap()
                                .push(entity);
                        }
                        FactionId::Rat => {
                            self.foes.get_mut(&FactionId::Lawless).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Feline).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Duck).unwrap().push(entity);
                            self.foes
                                .get_mut(&FactionId::Arachine)
                                .unwrap()
                                .push(entity);
                        }
                        FactionId::Duck => {
                            self.foes.get_mut(&FactionId::Lawless).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Rat).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Feline).unwrap().push(entity);
                            self.foes
                                .get_mut(&FactionId::Arachine)
                                .unwrap()
                                .push(entity);
                        }
                        FactionId::Arachine => {
                            self.foes.get_mut(&FactionId::Lawless).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Rat).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Duck).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Feline).unwrap().push(entity);
                        }
                        FactionId::Lawless => {
                            self.foes.get_mut(&FactionId::Lawless).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Feline).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Rat).unwrap().push(entity);
                            self.foes.get_mut(&FactionId::Duck).unwrap().push(entity);
                            self.foes
                                .get_mut(&FactionId::Arachine)
                                .unwrap()
                                .push(entity);
                        }
                    }
                }
                None => self.get_mut_unsafe(part.0).set(None),
            }
        }
    }
    pub fn get_unsafe(&self, (x, y): (usize, usize)) -> &Tile {
        &self.tiles[y][x]
    }
    pub fn get_mut_unsafe(&mut self, (x, y): (usize, usize)) -> &mut Tile {
        &mut self.tiles[y][x]
    }
    pub fn get(&self, (x, y): (usize, usize)) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Tile> {
        if x < self.width && y < self.height {
            Some(&mut self.tiles[y][x])
        } else {
            None
        }
    }
    pub fn all_coords(&self) -> std::slice::Iter<(usize, usize)> {
        self.flat.iter()
    }
    pub fn all(&self) -> Vec<&Tile> {
        self.tiles.iter().flatten().collect()
    }
    pub fn all_mut(&mut self) -> Vec<&mut Tile> {
        self.tiles.iter_mut().flatten().collect()
    }
    pub fn all_that_have_combatant(
        &self,
        parts: &GameParts,
        requirement: &dyn CombatantRequirement,
    ) -> Vec<&Tile> {
        self.tiles
            .iter()
            .flatten()
            .filter(|tile| match tile.get() {
                Some(combatant) => requirement.satisfies(parts, &combatant),
                None => false,
            })
            .collect()
    }
    pub fn all_mut_that_have_combatant(
        &mut self,
        parts: &GameParts,
        requirement: &dyn CombatantRequirement,
    ) -> Vec<&mut Tile> {
        self.tiles
            .iter_mut()
            .flatten()
            .filter(|tile| match tile.get() {
                Some(combatant) => requirement.satisfies(parts, &combatant),
                None => false,
            })
            .collect()
    }
    pub fn change<'a>(&mut self, change: BoardChange, events: &mut Events) {
        match change {
            BoardChange::MoveEntity(id, start, end) => {
                self.move_entity_from_tiles(id, start.0, start.1, end.0, end.1)
            }
        }
    }
    pub fn get_target<'a>(
        &self,
        entity: &'a Entity,
        parts: &'a GameParts,
    ) -> Option<(&'a Entity, Skill)> {
        let id = entity.id();
        if let Some(xy) = self.combatants.get(&id) {
            for skill in entity.get_skills() {
                if skill.can_target_self() {
                    return Some((entity, *skill));
                } else {
                    for (i, ab) in self.combatants.iter() {
                        if *i != id {
                            if skill.can_target(distance(xy, ab), *i, parts) {
                                return Some((parts.get_entity(i), *skill));
                            }
                        }
                    }
                }
            }
        }
        return None;
    }
    pub fn get_closest_ally(&self, id: Index, parts: &GameParts) -> Option<Index> {
        let faction = parts.get_entity(&id).faction();
        if let FactionId::Lawless = faction {
        } else {
            return None;
        }
        let mut dist = 100000f64;
        let mut result: Option<Index> = None;
        let xy = self.combatants.get(&id).unwrap();
        for ally in self.allies.get(faction).unwrap() {
            if *ally == id {
                continue;
            }
            let d = distance(xy, self.combatants.get(ally).unwrap());
            if d < dist {
                dist = d;
                result = Some(*ally);
            }
        }
        return result;
    }
    pub fn get_closest_enemy(&self, id: Index, parts: &GameParts) -> Option<Index> {
        let faction = parts.get_entity(&id).faction();
        let mut dist = 100000f64;
        let mut result: Option<Index> = None;
        let xy = self.combatants.get(&id).unwrap();
        for ally in self.foes.get(faction).unwrap() {
            let d = distance(xy, self.combatants.get(ally).unwrap());
            if d < dist {
                dist = d;
                result = Some(*ally);
            }
        }
        return result;
    }
    pub fn move_entity_from_tiles(
        &mut self,
        id: Index,
        x_start: usize,
        y_start: usize,
        x_end: usize,
        y_end: usize,
    ) {
        if self
            .tiles
            .get_mut(x_end)
            .unwrap()
            .get_mut(y_end)
            .unwrap()
            .set_if_none(Some(id))
        {
            self.tiles
                .get_mut(x_start)
                .unwrap()
                .get_mut(y_start)
                .unwrap()
                .set(None);
        }
    }
    pub fn move_entity(
        &self,
        id: Index,
        path: &MovementPath,
        events: &mut Events,
    ) -> Option<MovementPath> {
        let (x, y) = self.combatants.get(&id).unwrap();
        let tile = self.tiles.get(*x).unwrap().get(*y).unwrap();
        if let Some(end_tile) = match path.first() {
            Some(dir) => {
                if let Some(to) = tile.get_from_dir(self, dir) {
                    if let None = to.get() {
                        Some(to)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            None => None,
        } {
            events.push(
                2,
                Event::ChangeBoard(BoardChange::MoveEntity(
                    id,
                    (*x, *y),
                    (end_tile.x, end_tile.y),
                )),
            );
            Some(path.pop())
        } else {
            None
        }
    }
    pub fn get_path_to_target(&self, start: Index, end: Index, _parts: &GameParts) -> MovementPath {
        let (x_start, y_start) = self.combatants.get(&start).unwrap();
        let tile_start = self.tiles.get(*x_start).unwrap().get(*y_start).unwrap();
        let (x_end, y_end) = self.combatants.get(&end).unwrap();
        let tile_end = self.tiles.get(*x_end).unwrap().get(*y_end).unwrap();

        if let Some(path) = self.get_path(tile_start, tile_end, MovementPath::Zero) {
            path
        } else {
            MovementPath::Zero
        }
    }
    fn get_path_left(
        &self,
        start: &Tile,
        end: &Tile,
        current: MovementPath,
    ) -> Option<MovementPath> {
        if let Some(next_tile) = start.left(&self.tiles) {
            if let Some(_) = next_tile.enhabitant {
                if let Some(last) = current.last() {
                    if last != Direction::Right {
                        self.get_path(next_tile, end, current.append(Direction::Left))
                    } else {
                        None
                    }
                } else {
                    self.get_path(next_tile, end, current.append(Direction::Left))
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn get_path_down(
        &self,
        start: &Tile,
        end: &Tile,
        current: MovementPath,
    ) -> Option<MovementPath> {
        if let Some(next_tile) = start.down(&self.tiles) {
            if let Some(_) = next_tile.enhabitant {
                if let Some(last) = current.last() {
                    if last != Direction::Up {
                        self.get_path(next_tile, end, current.append(Direction::Down))
                    } else {
                        None
                    }
                } else {
                    self.get_path(next_tile, end, current.append(Direction::Down))
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn get_path_right(
        &self,
        start: &Tile,
        end: &Tile,
        current: MovementPath,
    ) -> Option<MovementPath> {
        if let Some(next_tile) = start.right(&self.tiles) {
            if let Some(_) = next_tile.enhabitant {
                if let Some(last) = current.last() {
                    if last != Direction::Left {
                        self.get_path(next_tile, end, current.append(Direction::Right))
                    } else {
                        None
                    }
                } else {
                    self.get_path(next_tile, end, current.append(Direction::Right))
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn get_path_up(&self, start: &Tile, end: &Tile, current: MovementPath) -> Option<MovementPath> {
        if let Some(next_tile) = start.up(&self.tiles) {
            if let Some(_) = next_tile.enhabitant {
                if let Some(last) = current.last() {
                    if last != Direction::Down {
                        self.get_path(next_tile, end, current.append(Direction::Up))
                    } else {
                        None
                    }
                } else {
                    self.get_path(next_tile, end, current.append(Direction::Up))
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn get_path(&self, start: &Tile, end: &Tile, current: MovementPath) -> Option<MovementPath> {
        if start == end {
            Some(current)
        } else if let MovementPath::Four(a, b, c, d) = current {
            Some(MovementPath::Four(a, b, d, c))
        } else {
            match start.face_up {
                true => {
                    if end.y < start.y {
                        if let Some(path) = self.get_path_down(start, end, current) {
                            Some(path)
                        } else {
                            if end.x > start.x {
                                if let Some(path) = self.get_path_right(start, end, current) {
                                    Some(path)
                                } else {
                                    if end.x <= start.x {
                                        if let Some(path) = self.get_path_left(start, end, current)
                                        {
                                            Some(path)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                            } else {
                                if end.x <= start.x {
                                    if let Some(path) = self.get_path_left(start, end, current) {
                                        Some(path)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        if end.x > start.x {
                            if let Some(path) = self.get_path_right(start, end, current) {
                                Some(path)
                            } else {
                                if end.x <= start.x {
                                    if let Some(path) = self.get_path_left(start, end, current) {
                                        Some(path)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                        } else {
                            if end.x <= start.x {
                                if let Some(path) = self.get_path_left(start, end, current) {
                                    Some(path)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                    }
                }
                false => {
                    if end.y > start.y {
                        if let Some(path) = self.get_path_up(start, end, current) {
                            Some(path)
                        } else {
                            if end.x < start.x {
                                if let Some(path) = self.get_path_left(start, end, current) {
                                    Some(path)
                                } else {
                                    if end.x >= start.x {
                                        if let Some(path) = self.get_path_right(start, end, current)
                                        {
                                            Some(path)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                            } else {
                                if end.x >= start.x {
                                    if let Some(path) = self.get_path_right(start, end, current) {
                                        Some(path)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        if end.x < start.x {
                            if let Some(path) = self.get_path_left(start, end, current) {
                                Some(path)
                            } else {
                                if end.x >= start.x {
                                    if let Some(path) = self.get_path_right(start, end, current) {
                                        Some(path)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                        } else {
                            if end.x >= start.x {
                                if let Some(path) = self.get_path_right(start, end, current) {
                                    Some(path)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn distance((x1, y1): &(usize, usize), (x2, y2): &(usize, usize)) -> f64 {
    (((*x2 - *x1).pow(2) + (*y2 - *y1).pow(2)) as f64).sqrt()
}
