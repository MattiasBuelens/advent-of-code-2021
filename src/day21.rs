type Input = (u8, u8);

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

#[derive(Debug)]
struct Game {
    positions: [u8; 2],
    scores: [u32; 2],
    current_player: u8,
    next_die: u8,
    die_rolls: u32,
}

impl Game {
    pub fn new(start1: u8, start2: u8) -> Self {
        Self {
            positions: [start1, start2],
            scores: [0, 0],
            current_player: 0,
            next_die: 1,
            die_rolls: 0,
        }
    }

    pub fn step(&mut self) {
        let steps = (0..3).map(|_| self.roll_die()).sum::<u8>();
        let position = &mut self.positions[self.current_player as usize];
        let score = &mut self.scores[self.current_player as usize];
        *position = (*position + steps - 1) % 10 + 1;
        *score += *position as u32;
        self.current_player = if self.current_player == 0 { 1 } else { 0 };
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
        self.scores.iter().any(|&score| score >= 1000)
    }

    pub fn result(&self) -> u32 {
        let losing_score = *self.scores.iter().min().unwrap();
        losing_score * self.die_rolls
    }
}

#[aoc(day21, part1)]
pub fn part1(&(start1, start2): &Input) -> u32 {
    let mut game = Game::new(start1, start2);
    while !game.is_done() {
        game.step();
    }
    game.result()
}

#[aoc(day21, part2)]
pub fn part2(&(start1, start2): &Input) -> i32 {
    todo!()
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
        assert_eq!(part2(&input), 0);
    }
}
