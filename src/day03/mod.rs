type InputType = Vec<u8>;

#[aoc_generator(day03)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines()
        .map(|l| {
            l
                .chars()
                .map(|c| c.to_string().parse())
                .collect::<Result<Vec<u8>, std::num::ParseIntError>>()
        })
        .collect::<Result<Vec<InputType>, std::num::ParseIntError>>()
        .unwrap()
}

#[aoc(day03, part1)]
pub fn solve_part1(input: &[InputType]) -> u32 {
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for i in 0..input[0].len() {
        let mut zero_count = 0;
        let mut one_count = 0;

        for x in input {
            match x[i] {
                0 => zero_count += 1,
                1 => one_count += 1,
                _ => panic!("Invalid Input"),
            }
        }

        if zero_count > one_count {
            gamma_rate = format!("{}{}", gamma_rate, 0);
            epsilon_rate = format!("{}{}", epsilon_rate, 1);
        } else {
            gamma_rate = format!("{}{}", gamma_rate, 1);
            epsilon_rate = format!("{}{}", epsilon_rate, 0);
        }
    }

    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap();

    gamma_rate * epsilon_rate
}

#[aoc(day03, part2)]
pub fn solve_part2(input: &[InputType]) -> u32 {
    let mut ogr_candidates: Vec<InputType> = input.to_vec();
    let mut csr_candidates: Vec<InputType> = input.to_vec();

    for i in 0..input[0].len() {
        let most_common_ogr = most_common_bit(&ogr_candidates, i);
        let most_common_csr = most_common_bit(&csr_candidates, i);

        if ogr_candidates.len() > 1 {
            ogr_candidates = ogr_candidates
                .into_iter()
                .filter(|x| x[i] == most_common_ogr.unwrap_or(1))
                .collect::<Vec<InputType>>();
        }

        if csr_candidates.len() > 1 {
            csr_candidates = csr_candidates
                .into_iter()
                .filter(|x| x[i] != most_common_csr.unwrap_or(1))
                .collect::<Vec<InputType>>();
        }
    }

    let ogr: String = ogr_candidates[0].iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    let csr: String = csr_candidates[0].iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");

    let ogr = u32::from_str_radix(&ogr, 2).unwrap();
    let csr = u32::from_str_radix(&csr, 2).unwrap();

    ogr * csr
}

fn most_common_bit(input: &[InputType], index: usize) -> Option<u8> {
    let mut zero_count = 0;
    let mut one_count = 0;

    for x in input {
        match x[index] {
            0 => zero_count += 1,
            1 => one_count += 1,
            _ => panic!("Invalid Input"),
        }
    }

    match zero_count.cmp(&one_count) {
        std::cmp::Ordering::Less => Some(1),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(0),
    }
}

#[cfg(test)]
mod test_day03 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 198);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 230);
    }
}