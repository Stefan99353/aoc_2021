#[aoc_generator(day00)]
pub fn input_generator(input: &str) -> Vec<()> {
    vec![]
}

#[aoc(day00, part1)]
pub fn solve_part1(input: &[()]) -> u32 {
    todo!("Solve part 1");
}

#[aoc(day00, part2)]
pub fn solve_part2(input: &[()]) -> u32 {
    todo!("Solve part 2");
}

#[cfg(test)]
mod test_day00 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 0);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 0);
    }
}