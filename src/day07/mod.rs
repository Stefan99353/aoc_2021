#[aoc_generator(day07)]
pub fn input_generator(input: &str) -> Vec<Crab> {
    input
        .split(',')
        .map(|l| {
            let pos: u32 = l.parse()?;
            Ok(Crab::new(pos))
        })
        .collect::<Result<Vec<Crab>, std::num::ParseIntError>>()
        .unwrap()
}

#[aoc(day07, part1)]
pub fn solve_part1(input: &[Crab]) -> u32 {
    let min = input.iter().map(|c|c.pos).min().unwrap();
    let max = input.iter().map(|c|c.pos).max().unwrap();

    (min..=max)
        .into_iter()
        .map(|target| Crab::fuel_cost(input, target))
        .min()
        .unwrap()
}

#[aoc(day07, part2)]
pub fn solve_part2(input: &[Crab]) -> u32 {
    let min = input.iter().map(|c|c.pos).min().unwrap();
    let max = input.iter().map(|c|c.pos).max().unwrap();

    (min..=max)
        .into_iter()
        .map(|target| Crab::exp_fuel_cost(input, target))
        .min()
        .unwrap()
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Crab {
    pub pos: u32,
}

impl Crab {
    pub fn new(pos: u32) -> Self {
        Self {
            pos
        }
    }

    pub fn fuel_cost(crabs: &[Crab], target: u32) -> u32 {
        crabs
            .iter()
            .map(|crab| (crab.pos as i32 - target as i32).abs() as u32)
            .sum()
    }

    pub fn exp_fuel_cost(crabs: &[Crab], target: u32) -> u32 {
        crabs
            .iter()
            .map(|crab| {
                let dist = (crab.pos as i32 - target as i32).abs() as u32;
                (1..=dist).into_iter().sum::<u32>()
            })
            .sum()
    }
}

#[cfg(test)]
mod test_day07 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 37);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 168);
    }
}