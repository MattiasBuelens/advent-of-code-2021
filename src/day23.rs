use std::collections::HashMap;

use crate::util::Vector2D;

#[derive(Debug, Clone)]
pub struct Burrow {
    tiles: HashMap<Vector2D, Tile>,
    amphipods: Vec<Amphipod>,
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Hallway,
    Room(AmpipodKind),
}

#[derive(Debug, Clone)]
struct Amphipod {
    kind: AmpipodKind,
    position: Vector2D,
    state: AmphipodState,
}

#[derive(Debug, Copy, Clone)]
enum AmpipodKind {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Debug, Copy, Clone)]
enum AmphipodState {
    Initial,
    MovingToHallway,
    Stopped,
    MovingToRoom,
}

impl Amphipod {
    pub fn new(position: Vector2D, kind: AmpipodKind) -> Self {
        Self {
            kind,
            position,
            state: AmphipodState::Initial,
        }
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Burrow {
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
                            'A' => AmpipodKind::Amber,
                            'B' => AmpipodKind::Bronze,
                            'C' => AmpipodKind::Copper,
                            'D' => AmpipodKind::Desert,
                            _ => panic!("unknown amphipod kind: {}", c),
                        };
                        let amphipod = Amphipod::new(pos, amphipod_kind);
                        let room_kind = match x {
                            3 => AmpipodKind::Amber,
                            5 => AmpipodKind::Bronze,
                            7 => AmpipodKind::Copper,
                            9 => AmpipodKind::Desert,
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
    Burrow { tiles, amphipods }
}

#[aoc(day23, part1)]
pub fn part1(burrow: &Burrow) -> i32 {
    todo!()
}

#[aoc(day23, part2)]
pub fn part2(burrow: &Burrow) -> i32 {
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
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
