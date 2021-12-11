use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use colored::*;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> OctopusMap {
    let mut map = HashMap::new();

    let tmp_map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    for (y, row) in tmp_map.iter().enumerate() {
        for (x, energy) in row.iter().enumerate() {
            map.insert((x, y), Octopus::new(*energy));
        }
    }

    OctopusMap {
        map
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &OctopusMap) -> u32 {
    let mut input = input.clone();
    let mut flashes = 0;

    // println!("Before any steps:");
    // println!("{}", &input);

    for _x in 1..101 {
        flashes += input.do_step();
        // println!("After step {}:", x);
        // println!("{}", &input);
    }

    flashes
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &OctopusMap) -> u32 {
    let mut input = input.clone();
    let mut all_flashed = false;

    let mut step = 0;
    while !all_flashed {
        input.do_step();
        step += 1;

        // Check if all flashed
        all_flashed = input.map
            .iter()
            .all(|((_, _), o)| o.has_flashed);
    }

    step
}

#[derive(Debug, Clone)]
pub struct OctopusMap {
    pub map: HashMap<(usize, usize), Octopus>,
}

impl OctopusMap {
    pub fn do_step(&mut self) -> u32 {
        let mut flashes = 0;

        // Increase energy by one and reset has_flashed
        self.map.iter_mut()
            .for_each(|((_, _), octopus)| {
                octopus.energy += 1;
                octopus.has_flashed = false;
            });

        let mut x: usize = 0;
        let mut y: usize = 0;

        'x: while x < 10 {
            while y < 10 {
                let octopus = self.map.get_mut(&(x, y)).unwrap();
                if octopus.energy > 9 && !octopus.has_flashed {
                    // Flash
                    octopus.has_flashed = true;
                    flashes += 1;

                    for nx in -1..=1 {
                        for ny in -1..=1 {
                            let (nx, ny) = (x as i64 + nx, y as i64 + ny);
                            // Check out of bounds
                            if nx < 0 || ny < 0 || nx > 9 || ny > 9 { continue; }

                            let neighbor = self.map.get_mut(&(nx as usize, ny as usize)).unwrap();
                            neighbor.energy += 1;
                        }
                    }

                    // Start again
                    x = 0;
                    y = 0;
                    continue 'x;
                }

                y += 1;
            }
            y = 0;
            x += 1;
        }

        // Reset has_flashed and energy levels
        self.map.iter_mut()
            .for_each(|((_, _), octopus)| {
                if octopus.energy > 9 {
                    octopus.energy = 0;
                }
            });

        flashes
    }
}

impl Display for OctopusMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        for y in 0..10 {
            for x in 0..10 {
                let octo = self.map.get(&(x,y)).unwrap();

                match octo.has_flashed {
                    true => res = format!("{}{}", res, octo.energy.to_string().as_str().color("green")),
                    false => res = format!("{}{}", res, octo.energy),
                }
            }
            res = format!("{}\n", res);
        }

        write!(f, "{}", res)
    }
}

#[derive(Debug, Clone)]
pub struct Octopus {
    pub energy: u32,
    pub has_flashed: bool,
}

impl Octopus {
    pub fn new(energy: u32) -> Self {
        Self {
            energy,
            has_flashed: false,
        }
    }
}

#[cfg(test)]
mod test_day11 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 1656);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 195);
    }
}