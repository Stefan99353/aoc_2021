type InputType = Command;

#[aoc_generator(day02)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|l| {
            let line: Vec<String> = l.split(' ').map(|c| c.to_string()).collect();
            let cmd = line.get(0).unwrap();
            let val: i32 = line.get(1).unwrap().parse()?;

            match cmd.as_str() {
                "forward" => Ok(Command::Forward(val)),
                "down" => Ok(Command::Down(val)),
                "up" => Ok(Command::Up(val)),
                _ => panic!("Invalid input"),
            }
        })
        .collect::<Result<Vec<InputType>, std::num::ParseIntError>>()
        .unwrap()
}

#[aoc(day02, part1)]
pub fn solve_part1(input: &[InputType]) -> i32 {
    let mut h_pos = 0;
    let mut depth = 0;

    for cmd in input {
        match cmd {
            InputType::Forward(v) => h_pos += v,
            InputType::Down(v) => depth += v,
            InputType::Up(v) => depth -= v,
        }
    }

    h_pos * depth
}

#[aoc(day02, part2)]
pub fn solve_part2(input: &[InputType]) -> i32 {
    let mut h_pos = 0;
    let mut aim = 0;
    let mut depth = 0;

    for cmd in input {
        match cmd {
            InputType::Forward(v) => {
                h_pos += v;
                depth += aim * v;
            }
            InputType::Down(v) => {
                aim += v;
            }
            InputType::Up(v) => {
                aim -= v;
            }
        }
    }

    h_pos * depth
}

pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}
