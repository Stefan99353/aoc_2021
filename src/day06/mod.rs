use std::collections::HashMap;

#[aoc_generator(day06)]
pub fn input_generator(input: &str) -> Vec<Fish> {
    input
        .split(',')
        .map(|l| {
            let timer: u32 = l.parse()?;
            Ok(Fish::new(timer))
        })
        .collect::<Result<Vec<Fish>, std::num::ParseIntError>>()
        .unwrap()
}

#[aoc(day06, part1)]
pub fn solve_part1(input: &[Fish]) -> u128 {
    Fish::simulate(input, 80)
}

#[aoc(day06, part2)]
pub fn solve_part2(input: &[Fish]) -> u128 {
    Fish::simulate(input, 256)
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Fish {
    pub timer: u32,
}

impl Fish {
    fn new(timer: u32) -> Self {
        Self { timer }
    }

    fn simulate(start_population: &[Fish], days: u32) -> u128 {
        let mut fish = HashMap::new();

        for i in start_population.to_vec() {
            *fish.entry(i).or_insert(0) += 1;
        }

        for _ in 0..days {
            let mut new_fish = HashMap::new();
            for i in &fish {
                if i.0.timer == 0 {
                    *new_fish.entry(Fish::new(6)).or_insert(0) += *i.1;
                    *new_fish.entry(Fish::new(8)).or_insert(0) += *i.1;
                    continue;
                }

                *new_fish.entry(Fish::new(i.0.timer - 1)).or_insert(0) += *i.1;
            }
            fish = new_fish;
        }

        fish.values().sum()
    }
}

#[cfg(test)]
mod test_day06 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 5934);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 26984457539);
    }
}
