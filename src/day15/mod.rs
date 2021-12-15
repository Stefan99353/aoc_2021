use pathfinding::prelude::dijkstra;

const NEXT: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[Vec<u8>]) -> u32 {
    dijkstra(
        &(0, 0),
        |(x, y)| {
            NEXT.iter()
                .map(|(nx, ny)| {
                    input
                        .get((y + ny) as usize)
                        .and_then(|r| r.get((x + nx) as usize))
                        .map(|c| ((x + nx, y + ny), *c as u32))
                })
                .flatten()
                .collect::<Vec<_>>()
        },
        |&p| p == (input[0].len() as i32 - 1, input.len() as i32 - 1),
    )
    .unwrap()
    .1
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[Vec<u8>]) -> u32 {
    let dim = input.len();
    let goal = (dim as i32 * 5 - 1, dim as i32 * 5 - 1);

    dijkstra(
        &(0, 0),
        |(x, y)| {
            NEXT.iter()
                .map(|&(nx, ny)| ((x + nx) as usize, (y + ny) as usize))
                .filter(|(x, y)| (x / 5 < dim && y / 5 < dim))
                .map(|(x, y)| {
                    input.get(y % dim).and_then(|r| r.get(x % dim)).map(|c| {
                        (
                            (x as i32, y as i32),
                            ((*c as usize + (x / dim) + (y / dim) - 1) % 9 + 1) as u32,
                        )
                    })
                })
                .flatten()
                .collect::<Vec<_>>()
        },
        |&p| p == goal,
    )
    .unwrap()
    .1
}

#[cfg(test)]
mod test_day15 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 40);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 315);
    }
}
