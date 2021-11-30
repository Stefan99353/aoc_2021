type InputType = u32;

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines()
        .map(|l| {
            l.parse()
        })
        .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
        .unwrap()
}

#[aoc(day01, part1)]
pub fn solve_part1(input: &[InputType]) -> u32 {
    todo!("Solve part 1");

    panic!("No solution found");
}

#[aoc(day01, part2)]
pub fn solve_part2(input: &[InputType]) -> u32 {
    todo!("Solve part 1");

    panic!("No solution found");
}