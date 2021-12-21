use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};

use lazy_static::*;

pub type Input = (u8, u8);

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let start1 = lines
        .next()
        .unwrap()
        .strip_prefix("Player 1 starting position: ")
        .unwrap();
    let start2 = lines
        .next()
        .unwrap()
        .strip_prefix("Player 2 starting position: ")
        .unwrap();
    (start1.parse().unwrap(), start2.parse().unwrap())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct GameState {
    positions: [u8; 2],
    scores: [u32; 2],
    current_player: u8,
}

impl GameState {
    pub fn new(start1: u8, start2: u8) -> Self {
        Self {
            positions: [start1, start2],
            scores: [0, 0],
            current_player: 0,
        }
    }

    pub fn step(&mut self, die_roll: u8) {
        let position = &mut self.positions[self.current_player as usize];
        let score = &mut self.scores[self.current_player as usize];
        *position = (*position + die_roll - 1) % 10 + 1;
        *score += *position as u32;
        self.current_player = if self.current_player == 0 { 1 } else { 0 };
    }

    pub fn is_done_part1(&self) -> bool {
        self.scores.iter().any(|&score| score >= 1000)
    }

    pub fn is_done_part2(&self) -> bool {
        self.did_player1_win_part2() || self.did_player2_win_part2()
    }

    pub fn did_player1_win_part2(&self) -> bool {
        self.scores[0] >= 21
    }

    pub fn did_player2_win_part2(&self) -> bool {
        self.scores[1] >= 21
    }
}

#[derive(Debug)]
struct Game1 {
    state: GameState,
    next_die: u8,
    die_rolls: u32,
}

impl Game1 {
    pub fn new(start1: u8, start2: u8) -> Self {
        Self {
            state: GameState::new(start1, start2),
            next_die: 1,
            die_rolls: 0,
        }
    }

    pub fn step(&mut self) {
        let die_roll = (0..3).map(|_| self.roll_die()).sum::<u8>();
        self.state.step(die_roll);
    }

    pub fn roll_die(&mut self) -> u8 {
        let result = self.next_die;
        // Only the ones digit of the die matters,
        // since there are only 10 positions on the board
        self.next_die = (self.next_die + 1) % 10;
        self.die_rolls += 1;
        result
    }

    pub fn is_done(&self) -> bool {
        self.state.is_done_part1()
    }

    pub fn result(&self) -> u32 {
        let losing_score = *self.state.scores.iter().min().unwrap();
        losing_score * self.die_rolls
    }
}

#[aoc(day21, part1)]
pub fn part1(&(start1, start2): &Input) -> u32 {
    let mut game = Game1::new(start1, start2);
    while !game.is_done() {
        game.step();
    }
    game.result()
}

fn roll_three_quantum_dies() -> [u64; 10] {
    let mut counts = [0u64; 10];
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                counts[a + b + c] += 1;
            }
        }
    }
    counts
}

lazy_static! {
    static ref QUANTUM_ROLLS: [u64; 10] = roll_three_quantum_dies();
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Put the state with the smallest scores first.
        // All previous states that *could* end up in this state will be sorted *before* this state.
        other
            .scores
            .iter()
            .min()
            .cmp(&self.scores.iter().min())
            .then_with(|| other.scores.iter().max().cmp(&self.scores.iter().max()))
            .then_with(|| {
                // Use other fields to break ties and ensure consistency with Eq.
                self.scores
                    .cmp(&other.scores)
                    .then_with(|| self.positions.cmp(&other.positions))
                    .then_with(|| self.current_player.cmp(&other.current_player))
            })
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day21, part2)]
pub fn part2(&(start1, start2): &Input) -> u64 {
    let mut state_counts = HashMap::<GameState, u64>::new();

    let mut queue = BinaryHeap::<GameState>::new();
    let mut queue_set = HashSet::<GameState>::new();

    let start_state = GameState::new(start1, start2);
    queue.push(start_state.clone());
    queue_set.insert(start_state.clone());
    state_counts.insert(start_state, 1);

    while let Some(state) = queue.pop() {
        queue_set.remove(&state);
        let state_count = *state_counts.get(&state).expect("missing state count");
        for (roll, &roll_count) in QUANTUM_ROLLS.iter().enumerate() {
            if roll_count == 0 {
                continue;
            }
            let mut new_state = state.clone();
            new_state.step(roll as u8);
            *state_counts.entry(new_state.clone()).or_default() += state_count * roll_count;
            if !new_state.is_done_part2() && !queue_set.contains(&new_state) {
                queue.push(new_state.clone());
                queue_set.insert(new_state);
            }
        }
    }

    let player1_wins = state_counts
        .iter()
        .filter(|(state, _)| state.did_player1_win_part2())
        .map(|(_, &count)| count)
        .sum::<u64>();
    let player2_wins = state_counts
        .iter()
        .filter(|(state, _)| state.did_player2_win_part2())
        .map(|(_, &count)| count)
        .sum::<u64>();
    max(player1_wins, player2_wins)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
Player 1 starting position: 4
Player 2 starting position: 8"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 739785);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 444356092776315);
    }
}
