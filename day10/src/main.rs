use anyhow::Result;
use lazy_static::lazy_static;
use log::{debug, log_enabled, trace, Level, LevelFilter};
use std::collections::HashMap;

lazy_static! {
    static ref PART_ONE_SCORING: HashMap<char, u64> = {
        let mut m = HashMap::new();
        m.insert(')', 3);
        m.insert(']', 57);
        m.insert('}', 1197);
        m.insert('>', 25137);

        m
    };
    static ref PART_TWO_SCORING: HashMap<char, u64> = {
        let mut m = HashMap::new();
        m.insert('(', 1);
        m.insert('[', 2);
        m.insert('{', 3);
        m.insert('<', 4);

        m
    };
    static ref PAIR_MAP: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('(', ')');
        m.insert(')', '(');
        m.insert('[', ']');
        m.insert(']', '[');
        m.insert('{', '}');
        m.insert('}', '{');
        m.insert('<', '>');
        m.insert('>', '<');

        m
    };
}

#[derive(Debug, PartialEq, Eq)]
enum Line {
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn check_line(line: &str) -> Line {
    trace!("{}", line);

    let mut stack = Vec::new();

    for char in line.chars() {
        if let Some(corrupt_char) = match char {
            '(' | '[' | '{' | '<' => {
                stack.push(char);
                None
            }
            ')' => match stack.pop() {
                Some('(') => None,
                Some(_) | None => Some(char),
            },
            ']' => match stack.pop() {
                Some('[') => None,
                Some(_) | None => Some(char),
            },
            '}' => match stack.pop() {
                Some('{') => None,
                Some(_) | None => Some(char),
            },
            '>' => match stack.pop() {
                Some('<') => None,
                Some(_) | None => Some(char),
            },
            _ => panic!("Unexpected char: {}", char),
        } {
            return Line::Corrupted(corrupt_char);
        }
    }

    Line::Incomplete(stack)
}

fn score_part_two(line: &Line) -> u64 {
    if let Line::Incomplete(stack) = line {
        if log_enabled!(Level::Debug) {
            debug!(
                "{:?}",
                stack
                    .iter()
                    .rev()
                    .map(|c| PAIR_MAP.get(c).unwrap())
                    .collect::<String>()
            );
        }

        let mut score = 0;
        for char in stack.iter().rev() {
            score *= 5;
            score += *PART_TWO_SCORING.get(char).unwrap();
        }
        score
    } else {
        0
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let mut scores = aoc_utils::input()?
        .into_iter()
        .map(|l| check_line(&l))
        .filter(|l| matches!(l, Line::Incomplete(_)))
        .map(|l| score_part_two(&l))
        .collect::<Vec<_>>();

    assert_ne!(0, scores.len() % 2, "score list length must be odd");

    scores.sort_unstable();

    println!("{}", scores[scores.len() / 2]);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_line_corrupted() -> Result<()> {
        aoc_utils::init_test_logger()?;

        let result = check_line("{([(<{}[<>[]}>{[]{[(<()>");
        assert_eq!(Line::Corrupted('}'), result);

        Ok(())
    }

    #[test]
    fn test_check_line_incomplete() -> Result<()> {
        aoc_utils::init_test_logger()?;

        let result = check_line("<{([{{}}[<[[[<>{}]]]>[]]");
        assert!(matches!(result, Line::Incomplete(_)));

        Ok(())
    }

    #[test]
    fn test_score_part_two() -> Result<()> {
        aoc_utils::init_test_logger()?;

        assert_eq!(
            288957,
            score_part_two(&check_line("[({(<(())[]>[[{[]{<()<>>"))
        );

        assert_eq!(5566, score_part_two(&check_line("[(()[<>])]({[<{<<[]>>(")));

        assert_eq!(
            1480781,
            score_part_two(&check_line("(((({<>}<{<{<>}{[]{[]{}"))
        );

        assert_eq!(
            995444,
            score_part_two(&check_line("{<[[]]>}<{[{[{[]{()[[[]"))
        );

        assert_eq!(294, score_part_two(&check_line("<{([{{}}[<[[[<>{}]]]>[]]")));

        Ok(())
    }
}
