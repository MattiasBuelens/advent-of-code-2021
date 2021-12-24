use std::collections::HashMap;
use std::iter::empty;

use itertools::Itertools;
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
    InHallway,
    InRoom,
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
    fn amphipod_at(&self, pos: Vector2D) -> Option<&Amphipod> {
        self.amphipods
            .iter()
            .find(|amphipod| amphipod.position == pos)
    }

    fn amphipods_in_room<'a>(
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

    fn is_done(&self, burrow: &Burrow) -> bool {
        self.amphipods.iter().all(|amphipod| {
            burrow.tiles.get(&amphipod.position) == Some(&Tile::Room(amphipod.kind))
        })
    }
}

impl Burrow {
    fn get_path(&self, from: Vector2D, to: Vector2D) -> Vec<Vector2D> {
        let (path, _) = astar(
            &from,
            |pos| {
                pos.neighbours()
                    .filter(|pos| self.tiles.contains_key(&pos))
                    .map(|pos| (pos, 1))
            },
            |&pos| (pos - to).manhattan_distance(),
            |&pos| pos == to,
        )
        .expect("no path found");
        path
    }
}

impl Amphipod {
    fn get_moves<'a>(
        &'a self,
        burrow: &'a Burrow,
        state: &'a State,
    ) -> Box<dyn Iterator<Item = Vector2D> + 'a> {
        match self.state {
            AmphipodState::Initial => Box::new(
                burrow
                    .tiles
                    .iter()
                    .filter(|(pos, tile)| {
                        tile == &&Tile::Hallway && self.can_move(**pos, burrow, state)
                    })
                    .map(|(pos, _)| *pos),
            ),
            AmphipodState::InHallway => Box::new(
                burrow
                    .tiles
                    .iter()
                    .filter(|(pos, tile)| {
                        tile == &&Tile::Room(self.kind) && self.can_move(**pos, burrow, state)
                    })
                    .map(|(pos, _)| *pos),
            ),
            AmphipodState::InRoom => Box::new(empty()),
        }
    }

    fn can_move(&self, pos: Vector2D, burrow: &Burrow, state: &State) -> bool {
        let next_tile = *burrow
            .tiles
            .get(&pos)
            .expect("next position must be in bounds");
        let can_move = match (self.state, next_tile) {
            (AmphipodState::Initial, Tile::Hallway) => {
                // Amphipods will never stop on the space immediately outside any room.
                match burrow.tiles.get(&(pos + Vector2D::new(0, 1))) {
                    Some(Tile::Room(_)) => false,
                    _ => true,
                }
            }
            (AmphipodState::InHallway, Tile::Room(next_room)) if next_room == self.kind => {
                // Amphipods will never move from the hallway into a room unless that room is
                // their destination room and that room contains no amphipods which do not also
                // have that room as their own destination.
                state
                    .amphipods_in_room(self.kind, burrow)
                    .all(|amphipod| amphipod.kind == self.kind)
            }
            _ => false,
        };
        if !can_move {
            return false;
        }
        // Cannot move past or into occupied spaces.
        let path = burrow.get_path(self.position, pos);
        let is_clear_path = path
            .into_iter()
            .skip(1)
            .all(|pos| state.amphipod_at(pos).is_none());
        if !is_clear_path {
            return false;
        }
        true
    }

    fn move_to(&mut self, pos: Vector2D, burrow: &Burrow) -> u32 {
        let path_length = burrow.get_path(self.position, pos).len() as u32 - 1;
        self.position = pos;
        self.state = match self.state {
            AmphipodState::Initial => AmphipodState::InHallway,
            AmphipodState::InHallway => AmphipodState::InRoom,
            AmphipodState::InRoom => panic!("cannot move when already in destination room"),
        };
        path_length * self.kind.energy()
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
pub fn part1(input: &str) -> u32 {
    let (burrow, state) = &input_generator(input);
    let (_, cost) = astar(
        state,
        |state| {
            state
                .amphipods
                .iter()
                .enumerate()
                .flat_map(move |(amphipod_index, amphipod)| {
                    amphipod.get_moves(burrow, state).map(move |next_pos| {
                        let mut state = state.clone();
                        let amphipod = &mut state.amphipods[amphipod_index];
                        let cost = amphipod.move_to(next_pos, burrow);
                        // dbg!(&amphipod, amphipod_index, cost);
                        (state, cost)
                    })
                })
                .collect::<Vec<_>>()
        },
        |state| {
            state
                .amphipods
                .iter()
                .map(|amphipod| {
                    burrow
                        .tiles
                        .iter()
                        .filter(|(_, tile)| tile == &&Tile::Room(amphipod.kind))
                        .map(|(pos, _)| (amphipod.position - *pos).manhattan_distance() as u32)
                        .min()
                        .unwrap()
                })
                .sum::<u32>()
        },
        |state| state.is_done(burrow),
    )
    .expect("no solution found");
    cost
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> u32 {
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.insert(3, "  #D#C#B#A#");
    lines.insert(4, "  #D#B#A#C#");
    let input = lines.into_iter().intersperse("\n").collect::<String>();
    part1(&input)
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
        assert_eq!(part1(&TEST_INPUT), 12521);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 44169);
    }
}
