#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[i32]) -> i32 {
    solve(input, |cur_pos, target_pos| (cur_pos - target_pos).abs())
}

#[aoc(day7, part2)]
pub fn part2(input: &[i32]) -> i32 {
    solve(input, |cur_pos, target_pos| {
        let steps = (cur_pos - target_pos).abs();
        // 1 + 2 + 3 + ... + steps = steps * (steps + 1) / 2
        steps * (steps + 1) / 2
    })
}

fn solve(crabs: &[i32], fuel_fn: fn(cur_pos: i32, target_pos: i32) -> i32) -> i32 {
    let min_pos = crabs.iter().min().unwrap().clone();
    let max_pos = crabs.iter().max().unwrap().clone();
    let mut best_pos = 0;
    let mut best_fuel = i32::MAX;
    'outer: for pos in min_pos..=max_pos {
        let mut fuel = 0;
        for &crab in crabs {
            if fuel >= best_fuel {
                continue 'outer;
            }
            fuel += fuel_fn(crab, pos);
        }
        if fuel < best_fuel {
            best_pos = pos;
            best_fuel = fuel;
        }
    }
    best_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 168);
    }
}
