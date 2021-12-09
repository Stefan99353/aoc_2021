use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day09)]
pub fn input_generator(input: &str) -> CaveSystem {
    CaveSystem::new(input)
}

#[aoc(day09, part1)]
pub fn solve_part1(input: &CaveSystem) -> i32 {
    input
        .low_points()
        .iter()
        .map(|(x, y)| input.height_at(*x, *y).unwrap() + 1)
        .sum()
}

#[aoc(day09, part2)]
pub fn solve_part2(input: &CaveSystem) -> i32 {
    input
        .find_basins()
        .iter()
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

#[derive(Debug, Clone)]
pub struct CaveSystem {
    pub height_map: HashMap<(i32, i32), i32>,
    pub height: i32,
    pub width: i32,
}

impl CaveSystem {
    pub fn new(str: &str) -> Self {
        let mut height_map = HashMap::new();

        let tmp_map = str
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_string().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        for (y, row) in tmp_map.iter().enumerate() {
            for (x, pos) in row.iter().enumerate() {
                height_map.insert((x as i32, y as i32), *pos);
            }
        }

        Self {
            height_map,
            height: tmp_map.len() as i32,
            width: tmp_map[0].len() as i32,
        }
    }

    pub fn low_points(&self) -> Vec<(i32, i32)> {
        let mut result = vec![];

        for x in 0..self.width {
            for y in 0..self.height {
                if self.is_low_point(x, y) {
                    result.push((x, y))
                }
            }
        }

        result
    }

    pub fn height_at(&self, x: i32, y: i32) -> Option<&i32> {
        self.height_map.get(&(x, y))
    }

    pub fn is_low_point(&self, x: i32, y: i32) -> bool {
        let current_pos = self.height_at(x, y).unwrap();

        // Top
        if let Some(top) = self.height_at(x, y - 1) {
            if top <= current_pos {
                return false;
            }
        }

        // Right
        if let Some(right) = self.height_at(x + 1, y) {
            if right <= current_pos {
                return false;
            }
        }

        // Bottom
        if let Some(bottom) = self.height_at(x, y + 1) {
            if bottom <= current_pos {
                return false;
            }
        }

        // Left
        if let Some(left) = self.height_at(x - 1, y) {
            if left <= current_pos {
                return false;
            }
        }

        true
    }

    pub fn find_basins(&self) -> Vec<i32> {
        let low_points = self.low_points();
        let mut basins = vec![];

        for (x, y) in low_points {
            let mut visited_points: Vec<(i32, i32)> = vec![];
            let size = self.check_basin_neighbors(x, y, &mut visited_points);
            basins.push(size);
        }

        basins
    }

    fn check_basin_neighbors(&self, x: i32, y: i32, visited: &mut Vec<(i32, i32)>) -> i32 {
        // Check if already visited
        if visited.contains(&(x, y)) {
            return 0;
        }
        visited.push((x, y));

        // Check out of bounds
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return 0;
        }

        // Check if high_point
        if let Some(height) = self.height_at(x, y) {
            let height = height;
            if *height == 9 {
                return 0;
            }
        }

        let mut res = 1;
        // Check Top
        res += self.check_basin_neighbors(x, y - 1, visited);

        // Check Right
        res += self.check_basin_neighbors(x + 1, y, visited);

        // Check Bottom
        res += self.check_basin_neighbors(x, y + 1, visited);

        // Check Left
        res += self.check_basin_neighbors(x - 1, y, visited);

        res
    }
}

#[cfg(test)]
mod test_day08 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 15);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 1134);
    }
}
