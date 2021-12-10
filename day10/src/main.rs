use anyhow::Result;
use log::{trace, LevelFilter};
use std::collections::HashMap;

fn first_incorrect_closing_char(line: &str) -> Option<char> {
    trace!("{}", line);

    let mut stack = Vec::new();

    for char in line.chars() {
        match char {
            '(' => stack.push('('),
            ')' => match stack.pop() {
                Some('(') => (),
                Some(_) | None => return Some(')'),
            },
            '[' => stack.push('['),
            ']' => match stack.pop() {
                Some('[') => (),
                Some(_) | None => return Some(']'),
            },
            '{' => stack.push('{'),
            '}' => match stack.pop() {
                Some('{') => (),
                Some(_) | None => return Some('}'),
            },
            '<' => stack.push('<'),
            '>' => match stack.pop() {
                Some('<') => (),
                Some(_) | None => return Some('>'),
            },
            _ => panic!("Unexpected char: {}", char),
        }
    }

    None
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let mut points = HashMap::new();
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);

    let score = aoc_utils::input()?
        .into_iter()
        .filter_map(|l| first_incorrect_closing_char(&l))
        .map(|l| points.get(&l).unwrap())
        .sum::<u32>();

    println!("{}", score);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_corrupted() -> Result<()> {
        aoc_utils::init_test_logger()?;

        let result = first_incorrect_closing_char("{([(<{}[<>[]}>{[]{[(<()>");
        assert!(result.is_some());
        assert_eq!('}', result.unwrap());

        Ok(())
    }
}
