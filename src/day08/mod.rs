use tap::Tap;

#[aoc(day08, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let input_output = line.split(" | ").collect::<Vec<&str>>();
        if let [_, output] = &input_output[..] {
            for o in output.split(' ') {
                total += match o.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                };
            }
        }
    }

    total
}

#[aoc(day08, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let mut one = None;
        let mut four = None;
        let mut seven = None;
        let mut eight = None;

        let (input, output) = line.split_once(" | ").unwrap();
        for s in input.split(' ') {
            match s.len() {
                2 => one = Some(s),
                3 => seven = Some(s),
                4 => four = Some(s),
                7 => eight = Some(s),
                _ => {}
            }
        }

        let cf = one.unwrap();
        let bd = four
            .unwrap()
            .to_owned()
            .tap_mut(|s| s.retain(|ch| !cf.contains(ch)));
        let a = seven
            .unwrap()
            .to_owned()
            .tap_mut(|s| s.retain(|ch| !cf.contains(ch)))
            .chars()
            .next()
            .unwrap();
        let eg = eight
            .unwrap()
            .to_owned()
            .tap_mut(|s| s.retain(|ch| !cf.contains(ch)))
            .tap_mut(|s| s.retain(|ch| !bd.contains(ch)))
            .tap_mut(|s| s.retain(|ch| ch != a));

        let digits: u32 = output
            .split(' ')
            .map(|s| match s.len() {
                2 => '1',
                3 => '7',
                4 => '4',
                7 => '8',
                _ => {
                    let eg = has_eg(&eg, s);
                    let cf = has_cf(cf, s);
                    let a = has_a(&a.to_string(), s);
                    let bd = has_bd(&bd, s);

                    if eg && cf && a {
                        '0'
                    } else if a && cf && bd {
                        '9'
                    } else if a && cf {
                        '3'
                    } else if a && eg && bd {
                        '6'
                    } else if a && bd {
                        '5'
                    } else if a && eg {
                        '2'
                    } else {
                        unreachable!();
                    }
                }
            })
            .collect::<String>()
            .parse()
            .unwrap();

        result += digits;
    }

    result
}

fn has_eg(eg: &str, s: &str) -> bool {
    eg.chars().all(|c| s.contains(c))
}

fn has_bd(bd: &str, s: &str) -> bool {
    bd.chars().all(|c| s.contains(c))
}
fn has_cf(cf: &str, s: &str) -> bool {
    cf.chars().all(|c| s.contains(c))
}

fn has_a(a: &str, s: &str) -> bool {
    s.contains(a)
}

#[cfg(test)]
mod test_day08 {
    use super::{solve_part1, solve_part2};

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 26);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 61229);
    }
}
