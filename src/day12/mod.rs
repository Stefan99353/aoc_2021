use std::collections::HashMap;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    input.lines().for_each(|line| {
        let (start, end) = line.split_once('-').unwrap();

        if map.contains_key(start) {
            map.get_mut(start).unwrap().push(end.to_string());
        } else {
            map.insert(start.to_string(), vec![end.to_string()]);
        }

        if map.contains_key(end) {
            map.get_mut(end).unwrap().push(start.to_string());
        } else {
            map.insert(end.to_string(), vec![start.to_string()]);
        }
    });

    map
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &HashMap<String, Vec<String>>) -> u32 {
    let mut num_paths = 0;

    visit_cave(
        input,
        "start".to_string(),
        &mut vec![],
        &mut num_paths,
        false,
    );

    num_paths
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &HashMap<String, Vec<String>>) -> u32 {
    let mut num_paths = 0;

    visit_cave(
        input,
        "start".to_string(),
        &mut vec![],
        &mut num_paths,
        true,
    );

    num_paths
}

fn visit_cave(
    map: &HashMap<String, Vec<String>>,
    pos: String,
    path: &mut Vec<String>,
    num_paths: &mut u32,
    small_twice: bool,
) {
    if pos == "end" {
        *num_paths += 1;
        return;
    }

    if let Some(next) = map.get(&pos) {
        for next_path in next {
            if next_path.eq("start") || is_path_invalid(next_path, path, small_twice) {
                continue;
            }
            path.push(next_path.clone());
            visit_cave(map, next_path.clone(), path, num_paths, small_twice);
            path.pop();
        }
    }
}

#[allow(clippy::ptr_arg)]
#[allow(clippy::if_same_then_else)]
fn is_path_invalid(next_path: &String, path: &Vec<String>, small_twice: bool) -> bool {
    let mut count: HashMap<&String, u32> = HashMap::new();

    for p in path {
        *count.entry(p).or_insert(0) += 1;
    }
    *count.entry(next_path).or_insert(0) += 1;

    let mut seen_twice = false;
    for (pos, count) in count.iter() {
        if !pos.to_lowercase().eq(*pos) {
            continue;
        }

        if small_twice {
            if *count == 1 {
                continue;
            } else if *count == 2 && !seen_twice {
                seen_twice = true;
                continue;
            } else if *count == 2 && seen_twice {
                return true;
            } else if *count > 2 {
                return true;
            }
        } else if *count > 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test_day12 {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    fn part1() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part1(&input), 10);
    }

    #[test]
    fn part2() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part2(&input), 36);
    }
}
