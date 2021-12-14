use std::collections::HashMap;
use std::str::FromStr;
use itertools::{Itertools, MinMaxResult};

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> (Polymer, PairInsertions) {
    input.split_once("\n\n")
        .map(|(poly, pairs)| {
            (
                Polymer::from_str(poly).unwrap(), PairInsertions::from_str(pairs).unwrap())
        })
        .unwrap()
}

#[aoc(day14, part1)]
pub fn solve_part1((polymer, pairs): &(Polymer, PairInsertions)) -> u64 {
    let mut polymer = polymer.clone();

    for _ in 1..=10 {
        polymer = pairs.apply(polymer);
    }

    match polymer.counts.values().minmax() {
        MinMaxResult::MinMax(min, max) => (max - min) as u64,
        _ => 0,
    }
}

#[aoc(day14, part2)]
pub fn solve_part2((polymer, pairs): &(Polymer, PairInsertions)) -> u64 {
    let mut polymer = polymer.clone();

    for _ in 1..=40 {
        polymer = pairs.apply(polymer);
    }

    match polymer.counts.values().minmax() {
        MinMaxResult::MinMax(min, max) => (max - min) as u64,
        _ => 0,
    }
}

pub type Pair = [char; 2];

#[derive(Debug, Clone)]
pub struct Polymer {
    pub pairs: HashMap<Pair, usize>,
    pub counts: HashMap<char, usize>,
}

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let counts = chars.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(*c).or_insert(0) += 1;
            acc
        });
        let pairs = chars
            .windows(2)
            .fold(HashMap::new(), |mut acc, win| {
                let key: [char; 2] = (*win)
                    .try_into()
                    .unwrap();
                *acc.entry(key).or_insert(0) += 1;
                acc
            });
        Ok(Self { pairs, counts })
    }
}

pub struct PairInsertions(HashMap<Pair, char>);

impl PairInsertions {
    pub fn apply(&self, polymer: Polymer) -> Polymer {
        polymer.pairs
            .into_iter()
            .fold(Polymer { pairs: HashMap::default(), counts: polymer.counts }, |mut poly, (k, v)| {
                if let Some(insertion) = self.0.get(&k) {
                    *poly.counts.entry(*insertion).or_insert(0) += v;
                    for pair in [[k[0], *insertion], [*insertion, k[1]]] {
                        *poly.pairs.entry(pair).or_insert(0) += v;
                    }
                }
                poly
            })
    }
}

impl FromStr for PairInsertions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|l| {
                l.split_once(" -> ")
                    .map(|(left, right)| {
                        (
                            left.chars().collect::<Vec<char>>(),
                            right.chars().collect::<Vec<char>>(),
                        )
                    })
                    .map(|(l, r)| {
                        l.try_into().ok().zip(r.first().copied()).unwrap()
                    })
                    .unwrap()
            })
            .collect();

        Ok(Self(map))
    }
}


#[cfg(test)]
mod test_day14 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 1588);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 2188189693529);
    }
}