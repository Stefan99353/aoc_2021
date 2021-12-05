use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

type InputType = LineSegment;

#[aoc_generator(day05)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<InputType>, ()>>()
        .unwrap()
}

#[aoc(day05, part1)]
pub fn solve_part1(input: &[InputType]) -> usize {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    let segments = input
        .iter()
        .filter(|ls| ls.x1 == ls.x2 || ls.y1 == ls.y2)
        .cloned()
        .collect::<Vec<InputType>>();

    // Draw lines
    for segment in segments {
        // X
        if segment.x1 != segment.x2 {
            let mut x_coords = vec![segment.x1, segment.x2];
            x_coords.sort_unstable();
            for x in x_coords[0]..x_coords[1] + 1 {
                increase_map_value(x, segment.y1, &mut map);
            }
        }

        // Y
        if segment.y1 != segment.y2 {
            let mut y_coords = vec![segment.y1, segment.y2];
            y_coords.sort_unstable();
            for y in y_coords[0]..y_coords[1] + 1 {
                increase_map_value(segment.x1, y, &mut map);
            }
        }
    }

    map.iter().filter(|&(_, v)| v > &1).count()
}

#[aoc(day05, part2)]
pub fn solve_part2(input: &[InputType]) -> usize {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    // Only horizontal and vertical
    let segments = input
        .iter()
        .filter(|ls| ls.x1 == ls.x2 || ls.y1 == ls.y2)
        .cloned()
        .collect::<Vec<InputType>>();
    // Draw lines
    for segment in segments {
        // X
        if segment.x1 != segment.x2 {
            let mut x_coords = vec![segment.x1, segment.x2];
            x_coords.sort_unstable();
            for x in x_coords[0]..x_coords[1] + 1 {
                increase_map_value(x, segment.y1, &mut map);
            }
        }

        // Y
        if segment.y1 != segment.y2 {
            let mut y_coords = vec![segment.y1, segment.y2];
            y_coords.sort_unstable();
            for y in y_coords[0]..y_coords[1] + 1 {
                increase_map_value(segment.x1, y, &mut map);
            }
        }
    }

    let segments = input
        .iter()
        .filter(|ls| ls.x1 != ls.x2 && ls.y1 != ls.y2)
        .cloned()
        .collect::<Vec<InputType>>();

    for s in segments {
        let x_adder = match s.x1.cmp(&s.x2) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => {
                unreachable!();
            }
        };

        let y_adder = match s.y1.cmp(&s.y2) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => {
                unreachable!();
            }
        };

        let mut x = s.x1;
        let mut y = s.y1;

        while x != s.x2 && y != s.y2 {
            increase_map_value(x, y, &mut map);
            x += x_adder;
            y += y_adder;
        }

        increase_map_value(s.x2, s.y2, &mut map);
    }

    map.iter().filter(|&(_, v)| v > &1).count()
}

fn increase_map_value(x: i32, y: i32, map: &mut HashMap<(i32, i32), i32>) {
    match map.get_mut(&(x, y)) {
        None => {
            map.insert((x, y), 1);
        }
        Some(v) => *v += 1,
    };
}

#[derive(Debug, Clone)]
pub struct LineSegment {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl FromStr for LineSegment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s
            .split(" -> ")
            .map(|x| x.split(",").map(|y| y.to_string()).collect::<Vec<String>>())
            .flatten()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        Ok(Self {
            x1: line[0],
            y1: line[1],
            x2: line[2],
            y2: line[3],
        })
    }
}

#[cfg(test)]
mod test_day05 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 5);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 12);
    }
}
