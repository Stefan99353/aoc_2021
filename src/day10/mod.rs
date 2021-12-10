use std::collections::VecDeque;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<Bracket>> {
    input
        .lines()
        .map(|line| Bracket::from_sequence(line))
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Vec<Bracket>]) -> u64 {
    let mut sum = 0;

    for line in input {
        let mut open_stack: VecDeque<&Bracket> = VecDeque::new();

        // Check line if corrupt
        for bracket in line {
            if bracket.is_open() {
                open_stack.push_front(bracket);
                continue;
            }

            // Bracket is closing: Check if matches last opened
            let last_opened = *open_stack.front().unwrap();
            if bracket.cmp_type(last_opened) {
                // Remove from stack
                open_stack.pop_front();
                continue;
            }

            // Corrupted line found
            sum += bracket.score_p1();
            break;
        }
    }

    sum
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Vec<Bracket>]) -> u64 {
    let mut scores = input
        .to_vec()
        .into_iter()
        .filter_map(|line| {
            let mut open_stack: VecDeque<Bracket> = VecDeque::new();
            // Check line if corrupt
            for bracket in line {
                if bracket.is_open() {
                    open_stack.push_front(bracket);
                    continue;
                }

                // Bracket is closing: Check if matches last opened
                let last_opened = open_stack.front().unwrap();
                if bracket.cmp_type(last_opened) {
                    // Remove from stack
                    open_stack.pop_front();
                    continue;
                }

                // Corrupted line found
                return None;
            }

            Some(open_stack)
        })
        .map(|remaining_brackets| {
            let mut score = 0;

            remaining_brackets.iter().for_each(|b| {
                score = score * 5 + b.score_p2();
            });

            score
        })
        .collect::<Vec<u64>>();

    scores.sort_unstable();

    *scores.get(scores.len() / 2).unwrap()
}

#[derive(Debug, Clone)]
pub enum Bracket {
    // ()
    Round(OpenClose),
    // []
    Square(OpenClose),
    // {}
    Curly(OpenClose),
    // <>
    Angle(OpenClose),
}

impl Bracket {
    pub fn from_sequence(s: &str) -> Vec<Self> {
        s.chars().map(|c| c.into()).collect::<Vec<Bracket>>()
    }

    pub fn is_open(&self) -> bool {
        matches!(
            self,
            Bracket::Round(OpenClose::Open)
                | Bracket::Square(OpenClose::Open)
                | Bracket::Curly(OpenClose::Open)
                | Bracket::Angle(OpenClose::Open)
        )
    }

    pub fn score_p1(&self) -> u64 {
        match self {
            Bracket::Round(_) => 3,
            Bracket::Square(_) => 57,
            Bracket::Curly(_) => 1197,
            Bracket::Angle(_) => 25137,
        }
    }

    pub fn score_p2(&self) -> u64 {
        match self {
            Bracket::Round(_) => 1,
            Bracket::Square(_) => 2,
            Bracket::Curly(_) => 3,
            Bracket::Angle(_) => 4,
        }
    }

    pub fn cmp_type(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Bracket::Round(_), Bracket::Round(_))
                | (Bracket::Square(_), Bracket::Square(_))
                | (Bracket::Curly(_), Bracket::Curly(_))
                | (Bracket::Angle(_), Bracket::Angle(_))
        )
    }
}

impl From<char> for Bracket {
    fn from(c: char) -> Self {
        match c {
            '(' => Bracket::Round(OpenClose::Open),
            ')' => Bracket::Round(OpenClose::Close),
            '[' => Bracket::Square(OpenClose::Open),
            ']' => Bracket::Square(OpenClose::Close),
            '{' => Bracket::Curly(OpenClose::Open),
            '}' => Bracket::Curly(OpenClose::Close),
            '<' => Bracket::Angle(OpenClose::Open),
            '>' => Bracket::Angle(OpenClose::Close),
            _ => panic!("Unvalid input"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OpenClose {
    Open,
    Close,
}

#[cfg(test)]
mod test_day10 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 26397);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 288957);
    }
}
