use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Board {
    numbers: [[u8; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .take(5)
            .map(|line| {
                line.split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok(Board {
            numbers,
            marked: Default::default(),
        })
    }
}

impl Board {
    fn mark(&mut self, number: u8) {
        for (y, row) in self.numbers.iter().enumerate() {
            if let Some(x) = row.iter().position(|&n| n == number) {
                self.marked[y][x] = true;
                return;
            }
        }
    }

    fn is_winner(&self) -> bool {
        // Rows
        let mut rows = self.marked.iter();
        if rows.any(|row| row.iter().all(|mark| *mark)) {
            return true;
        }
        // Columns
        for x in 0..self.marked.len() {
            let mut column = self.marked.iter().map(|row| row[x]);
            if column.all(|mark| mark) {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0u32;
        for (y, row) in self.numbers.iter().enumerate() {
            for (x, number) in row.iter().enumerate() {
                if !self.marked[y][x] {
                    sum += *number as u32;
                }
            }
        }
        sum
    }
}

#[derive(Debug)]
pub struct Input {
    draw: Vec<u8>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    let mut chunks = input.split("\n\n");
    let draw = chunks
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u8>>();
    let boards = chunks.map(|x| x.parse().unwrap()).collect::<Vec<Board>>();
    Input { draw, boards }
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> u32 {
    let mut boards = input.boards.to_vec();
    for &number in &input.draw {
        boards.iter_mut().for_each(|board| board.mark(number));
        if let Some(board) = boards.iter().find(|board| board.is_winner()) {
            return (number as u32) * board.sum_unmarked();
        }
    }
    panic!("no winner");
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> u32 {
    let mut boards = input.boards.to_vec();
    for &number in &input.draw {
        boards.iter_mut().for_each(|board| board.mark(number));
        // Remove all boards that have won in this turn
        while let Some(pos) = boards.iter().position(|board| board.is_winner()) {
            let board = boards.remove(pos);
            // If this was the last board, return its score
            if boards.is_empty() {
                return (number as u32) * board.sum_unmarked();
            }
        }
    }
    panic!("no winner");
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 4512);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 1924);
    }
}
