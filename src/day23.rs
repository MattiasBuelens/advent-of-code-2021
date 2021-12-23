use std::collections::HashMap;

use pathfinding::prelude::*;

use crate::util::Vector2D;

#[derive(Debug, Clone)]
pub struct Burrow {
    tiles: HashMap<Vector2D, Tile>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct State {
    amphipods: Vec<Amphipod>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Hallway,
    Room(AmphipodKind),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Amphipod {
    kind: AmphipodKind,
    position: Vector2D,
    state: AmphipodState,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum AmphipodKind {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum AmphipodState {
    Initial,
    MovingToHallway,
    MovingInHallway,
    StoppedInHallway,
    MovingToRoom,
    StoppedInRoom,
}

pub type Input = (Burrow, State);

impl Amphipod {
    pub fn new(position: Vector2D, kind: AmphipodKind) -> Self {
        Self {
            kind,
            position,
            state: AmphipodState::Initial,
        }
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Input {
    let (tiles, amphipods) = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let pos = Vector2D::new(x as i32, y as i32);
                let (tile, amphipod) = match c {
                    '#' | ' ' => return None,
                    '.' => (Tile::Hallway, None),
                    c @ ('A' | 'B' | 'C' | 'D') => {
                        let amphipod_kind = match c {
                            'A' => AmphipodKind::Amber,
                            'B' => AmphipodKind::Bronze,
                            'C' => AmphipodKind::Copper,
                            'D' => AmphipodKind::Desert,
                            _ => panic!("unknown amphipod kind: {}", c),
                        };
                        let amphipod = Amphipod::new(pos, amphipod_kind);
                        let room_kind = match x {
                            3 => AmphipodKind::Amber,
                            5 => AmphipodKind::Bronze,
                            7 => AmphipodKind::Copper,
                            9 => AmphipodKind::Desert,
                            _ => panic!("unexpected room at x = {}", x),
                        };
                        (Tile::Room(room_kind), Some(amphipod))
                    }
                    c => panic!("unexpected character: {}", c),
                };
                Some(((pos, tile), amphipod))
            })
        })
        .unzip::<(Vector2D, Tile), Option<Amphipod>, HashMap<_, _>, Vec<_>>();
    let amphipods = amphipods.into_iter().filter_map(|x| x).collect();
    let burrow = Burrow { tiles };
    let state = State { amphipods };
    (burrow, state)
}

impl State {
    pub fn amphipod_at(&self, pos: Vector2D) -> Option<&Amphipod> {
        self.amphipods
            .iter()
            .find(|amphipod| amphipod.position == pos)
    }

    pub fn amphipods_in_room<'a>(
        &'a self,
        room_kind: AmphipodKind,
        burrow: &'a Burrow,
    ) -> impl Iterator<Item = &Amphipod> + 'a {
        burrow
            .tiles
            .iter()
            .filter(move |(_, &tile)| tile == Tile::Room(room_kind))
            .filter_map(|(pos, _)| self.amphipod_at(*pos))
    }

    pub fn moving_amphipod(&self) -> Option<&Amphipod> {
        self.amphipods.iter().find(|amphipod| {
            amphipod.state == AmphipodState::MovingToHallway
                || amphipod.state == AmphipodState::MovingInHallway
                || amphipod.state == AmphipodState::MovingToRoom
        })
    }

    pub fn is_done(&self, burrow: &Burrow) -> bool {
        self.amphipods.iter().all(|amphipod| {
            burrow.tiles.get(&amphipod.position) == Some(&Tile::Room(amphipod.kind))
        })
    }
}

impl Amphipod {
    pub fn can_move(&self, pos: Vector2D, burrow: &Burrow, state: &State) -> bool {
        // Cannot move while a different amphipod is moving.
        match state.moving_amphipod() {
            Some(x) if x != self => return false,
            _ => {}
        };
        // Cannot move into a wall.
        let next_tile = match burrow.tiles.get(&pos) {
            Some(tile) => tile,
            None => return false,
        };
        // Cannot move into occupied space.
        if state.amphipod_at(pos).is_some() {
            return false;
        }
        match (self.state, next_tile) {
            (AmphipodState::Initial, _) => true,
            (AmphipodState::MovingToHallway, _) => true,
            (AmphipodState::MovingInHallway, Tile::Room(_)) => false,
            (AmphipodState::MovingInHallway, Tile::Hallway) => true,
            (AmphipodState::StoppedInHallway, _) => true,
            (AmphipodState::MovingToRoom, Tile::Room(next_room)) if next_room == &self.kind => {
                // Amphipods will never move from the hallway into a room unless that room is
                // their destination room and that room contains no amphipods which do not also
                // have that room as their own destination.
                state
                    .amphipods_in_room(self.kind, burrow)
                    .all(|amphipod| amphipod.kind == self.kind)
            }
            (AmphipodState::MovingToRoom, Tile::Room(_)) => false,
            (AmphipodState::MovingToRoom, Tile::Hallway) => true,
            (AmphipodState::StoppedInRoom, _) => false,
        }
    }

    pub fn do_move(&mut self, pos: Vector2D, burrow: &Burrow) {
        self.position = pos;
        self.state = match self.state {
            AmphipodState::Initial => AmphipodState::MovingToHallway,
            AmphipodState::MovingToHallway if burrow.tiles.get(&pos) == Some(&Tile::Hallway) => {
                AmphipodState::MovingInHallway
            }
            AmphipodState::StoppedInHallway => AmphipodState::MovingToRoom,
            state => state,
        }
    }

    pub fn can_stop(&self, burrow: &Burrow) -> bool {
        let tile = burrow.tiles.get(&self.position).unwrap();
        match self.state {
            AmphipodState::MovingInHallway if tile == &Tile::Hallway => {
                // Amphipods will never stop on the space immediately outside any room.
                match burrow.tiles.get(&(self.position + Vector2D::new(0, 1))) {
                    Some(Tile::Room(_)) => false,
                    _ => true,
                }
            }
            AmphipodState::MovingToRoom if tile == &Tile::Room(self.kind) => true,
            _ => false,
        }
    }

    pub fn do_stop(&mut self) {
        self.state = match self.state {
            AmphipodState::MovingInHallway => AmphipodState::StoppedInHallway,
            AmphipodState::MovingToRoom => AmphipodState::StoppedInRoom,
            state => state,
        }
    }
}

impl AmphipodKind {
    pub fn energy(&self) -> u32 {
        match *self {
            AmphipodKind::Amber => 1,
            AmphipodKind::Bronze => 10,
            AmphipodKind::Copper => 100,
            AmphipodKind::Desert => 1000,
        }
    }
}

#[aoc(day23, part1)]
pub fn part1((burrow, state): &Input) -> u32 {
    let (_, cost) = dijkstra(
        state,
        |state| {
            state
                .amphipods
                .iter()
                .enumerate()
                .flat_map(move |(amphipod_index, amphipod)| {
                    let moves = amphipod.position.neighbours().flat_map(move |next_pos| {
                        if amphipod.can_move(next_pos, burrow, state) {
                            println!("move {:?} to {}", &amphipod, &next_pos);
                            let mut state = state.clone();
                            let amphipod = &mut state.amphipods[amphipod_index];
                            let cost = amphipod.kind.energy();
                            amphipod.do_move(next_pos, burrow);
                            Some((state, cost))
                        } else {
                            None
                        }
                    });
                    let stop = if amphipod.can_stop(burrow) {
                        println!("stop {:?}", &amphipod);
                        let mut state = state.clone();
                        let amphipod = &mut state.amphipods[amphipod_index];
                        amphipod.do_stop();
                        Some((state, 0u32))
                    } else {
                        None
                    };
                    moves.chain(stop)
                })
                .collect::<Vec<_>>()
        },
        |state| state.is_done(burrow),
    )
    .expect("no solution found");
    cost
}

#[aoc(day23, part2)]
pub fn part2((burrow, state): &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 12521);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
