use std::iter::repeat;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[u8]) -> usize {
    let mut fishes = input.to_vec();
    for _i in 0..80 {
        step(&mut fishes);
    }
    fishes.len()
}

fn step(fishes: &mut Vec<u8>) {
    let mut new_fishes = 0usize;
    for fish in fishes.iter_mut() {
        *fish = match *fish {
            0 => {
                new_fishes += 1;
                6
            }
            n => n - 1,
        };
    }
    fishes.extend(repeat(8).take(new_fishes));
}

#[aoc(day6, part2)]
pub fn part2(input: &[u8]) -> u64 {
    let mut population = create_population(input);
    simulate(&mut population, 256);
    population.iter().sum()
}

// Amount of fishes with a given timer
type Population = [u64; 9];

fn create_population(fishes: &[u8]) -> Population {
    let mut population: Population = Population::default();
    for fish in fishes {
        population[*fish as usize] += 1;
    }
    population
}

fn simulate(population: &mut Population, steps: usize) {
    for _step in 0..steps {
        // Fishes with timer > 0 decrease their timer
        // Fishes with timer 0 spawn new fishes with timer 8
        population.rotate_left(1);
        // Fishes with timer 0 spawn reset their own timer to 6
        // (After rotating, timer 0 ends up at timer 8)
        population[6] += population[8];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"3,4,3,1,2".trim();
    }

    #[test]
    fn test_part1() {
        let mut fishes = input_generator(&TEST_INPUT);
        for _i in 0..18 {
            step(&mut fishes);
        }
        assert_eq!(fishes.len(), 26);
        for _i in 18..80 {
            step(&mut fishes);
        }
        assert_eq!(fishes.len(), 5934);
    }

    #[test]
    fn test_part1_optimized() {
        let fishes = input_generator(&TEST_INPUT);
        let mut population = create_population(&fishes);
        simulate(&mut population, 18);
        assert_eq!(population.iter().sum::<u64>(), 26);
        simulate(&mut population, 80 - 18);
        assert_eq!(population.iter().sum::<u64>(), 5934);
    }

    #[test]
    fn test_part2() {
        let fishes = input_generator(&TEST_INPUT);
        let mut population = create_population(&fishes);
        simulate(&mut population, 256);
        assert_eq!(population.iter().sum::<u64>(), 26984457539);
    }
}
