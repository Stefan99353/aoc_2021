use std::collections::HashSet;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (Vec<Dot>, Vec<(char, u32)>) {
    let (dots_str, folds_str) = input.split_once("\n\n").unwrap();

    let mut dots = vec![];
    dots_str
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
        })
        .for_each(|(x, y)| dots.push(Dot(x, y)));

    let mut folds = vec![];
    let all_folds = folds_str.split('\n').collect::<Vec<&str>>();
    for all_fold in all_folds {
        let (axis, val) = all_fold
            .strip_prefix("fold along ")
            .unwrap()
            .split_once('=')
            .unwrap();
        let axis = match axis {
            "x" => 'x',
            "y" => 'y',
            _ => panic!("Invalid input"),
        };

        folds.push((axis, val.parse::<u32>().unwrap()));
    }

    (dots, folds)
}

#[aoc(day13, part1)]
pub fn solve_part1((dots, folds): &(Vec<Dot>, Vec<(char, u32)>)) -> usize {
    let mut new_dots = HashSet::new();

    let (axis, val) = folds.first().unwrap();
    for dot in dots {
        new_dots.insert(dot.fold(*axis, *val));
    }

    new_dots.len()
}

#[aoc(day13, part2)]
pub fn solve_part2((dots, folds): &(Vec<Dot>, Vec<(char, u32)>)) -> String {
    let mut dots = dots.clone();

    for (axis, val) in folds {
        let mut new_dots = HashSet::new();
        for dot in dots {
            new_dots.insert(dot.fold(*axis, *val));
        }

        dots = new_dots.iter().cloned().collect();
    }

    Dot::print(&dots)
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Dot(u32, u32);

impl Dot {
    pub fn fold(&self, axis: char, value: u32) -> Self {
        match axis {
            'x' => {
                // Only fold if point is right of axis (x > value)
                if self.0 <= value {
                    return self.clone();
                }
                let new_x = value - (self.0 - value);
                Self(new_x, self.1)
            }
            'y' => {
                // Only fold if point is below of axis (y > value)
                if self.1 <= value {
                    return self.clone();
                }
                let new_y = value - (self.1 - value);
                Self(self.0, new_y)
            }
            _ => panic!("Invalid axis"),
        }
    }

    pub fn print(dots: &[Dot]) -> String {
        let mut largest_x = 0;
        let mut largest_y = 0;

        for dot in dots {
            if dot.0 > largest_x {
                largest_x = dot.0
            }
            if dot.1 > largest_y {
                largest_y = dot.1
            }
        }

        let mut result = String::from("\n");

        for y in 0..=largest_y {
            for x in 0..=largest_x {
                let is_dot = dots.iter().any(|d| d.0 == x && d.1 == y);
                match is_dot {
                    true => result = format!("{}#", result),
                    false => result = format!("{} ", result),
                }
            }

            result = format!("{}\n", result);
        }

        result
    }
}

#[cfg(test)]
mod test_day13 {
    use super::{input_generator, solve_part1};

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 17);
    }
}
