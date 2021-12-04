use std::num::ParseIntError;

use nalgebra::DMatrix;

#[aoc_generator(day04)]
pub fn input_generator(input: &str) -> BingoGame {
    let mut sections = input
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    // Parse commands
    let commands_string = sections.remove(0);

    let commands = commands_string
        .split(',')
        .map(|c| c.parse())
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .unwrap();


    // Parse boards
    let boards = sections
        .iter()
        .map(|b| parse_board(b))
        .collect::<Vec<BingoBoard>>();

    BingoGame {
        commands,
        boards,
    }
}

pub fn parse_board(board_input: &str) -> BingoBoard {
    let board = board_input
        .lines()
        .map(|row| {
            row.split_whitespace()
                .map(|field| BoardField {
                    value: field.parse().unwrap(),
                    checked: false,
                })
                .collect::<Vec<BoardField>>()
        })
        .collect::<Vec<Vec<BoardField>>>();

    let size = board.len();
    let flatten = board.into_iter().flatten().collect::<Vec<BoardField>>();

    let dm = DMatrix::from_iterator(size, size, flatten);

    BingoBoard(dm)
}

#[aoc(day04, part1)]
pub fn solve_part1(input: &BingoGame) -> u32 {
    let mut game = input.clone();

    for command in game.commands {
        // Check fields
        for board in &mut game.boards {
            board.check_value(command);

            // Check if won
            if board.has_won() {
                let score = board.get_score();

                return score * command;
            }
        }
    }

    0
}

#[aoc(day04, part2)]
pub fn solve_part2(input: &BingoGame) -> u32 {
    let mut game = input.clone();
    let mut winning_boards: Vec<usize> = vec![];
    let mut winning_boards_with_command: Vec<(usize, u32)> = vec![];

    for command in game.commands {
        // Check fields
        for board_index in 0..game.boards.len() {
            let board = game.boards.get_mut(board_index).unwrap();

            if !winning_boards.contains(&board_index) {
                board.check_value(command);
            }

            // Check if won
            if board.has_won() && !winning_boards.contains(&board_index) {
                winning_boards.push(board_index);
                winning_boards_with_command.push((board_index, command));
            }
        }
    }

    let (last_index, command) = *winning_boards_with_command.last().unwrap();
    let last_winning_board = game.boards.get(last_index).unwrap();
    let score = last_winning_board.get_score();

    score * command
}

#[derive(Debug, Clone)]
pub struct BingoGame {
    pub commands: Vec<u32>,
    pub boards: Vec<BingoBoard>,
}

#[derive(Debug, Clone)]
pub struct BingoBoard(DMatrix<BoardField>);

impl BingoBoard {
    pub fn check_value(&mut self, value: u32) {
        let matrix = &mut self.0;

        matrix.iter_mut()
            .for_each(|e| {
                if e.value == value {
                    e.checked = true;
                }
            });
    }

    pub fn has_won(&self) -> bool {
        let matrix = &self.0;

        for column in matrix.column_iter() {
            let won = column.iter().all(|e| e.checked);
            if won {
                return true;
            }
        }

        for row in matrix.row_iter() {
            let won = row.iter().all(|e| e.checked);
            if won {
                return true;
            }
        }

        false
    }

    pub fn get_score(&self) -> u32 {
        let matrix = &self.0;

        matrix.iter()
            .filter(|f| !f.checked)
            .map(|f| f.value)
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BoardField {
    pub value: u32,
    pub checked: bool,
}

#[cfg(test)]
mod test_day03 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 4512);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 1924);
    }
}