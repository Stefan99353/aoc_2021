type InputType = u32;

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines()
        .map(|l| {
            l.parse()
        })
        .collect::<Result<Vec<InputType>, std::num::ParseIntError>>()
        .unwrap()
}

#[aoc(day01, part1)]
pub fn solve_part1(input: &[InputType]) -> u32 {
    let mut counter = 0;

    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            counter += 1;
        }
    }

    counter
}

#[aoc(day01, part2)]
pub fn solve_part2(input: &[InputType]) -> u32 {
    let mut counter = 0;

    for i in 0..input.len()-3 {
        let a = input[i] + input[i+1] + input[i+2];
        let b = input[i+1] + input[i+2] + input[i+3];

        if b > a {
            counter += 1;
        }
    }

    counter
}