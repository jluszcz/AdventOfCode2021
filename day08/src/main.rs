use anyhow::{anyhow, Result};
use log::{debug, LevelFilter};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Input {
    _signal_patterns: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signal_patterns, output) = s
            .split_once(" | ")
            .ok_or_else(|| anyhow!("Failed to split {}", s))?;

        let signal_patterns = signal_patterns
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        if signal_patterns.len() != 10 {
            return Err(anyhow!("{:?} should have 10 values", signal_patterns));
        }

        let output = output
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        if output.len() != 4 {
            return Err(anyhow!("{:?} should have 4 values", output));
        }

        Ok(Self {
            _signal_patterns: signal_patterns,
            output,
        })
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let mut segment_to_digit_map = HashMap::new();
    segment_to_digit_map.insert("abcefg", '0');
    segment_to_digit_map.insert("cf", '1');
    segment_to_digit_map.insert("acdeg", '2');
    segment_to_digit_map.insert("acdfg", '3');
    segment_to_digit_map.insert("bcdf", '4');
    segment_to_digit_map.insert("abdfg", '5');
    segment_to_digit_map.insert("abdefg", '6');
    segment_to_digit_map.insert("acf", '7');
    segment_to_digit_map.insert("abcdefg", '8');
    segment_to_digit_map.insert("abcdfg", '9');

    debug!("Digit Map: {:?}", segment_to_digit_map);

    let input = aoc_utils::input()?
        .into_iter()
        .filter_map(|l| Input::from_str(&l).ok())
        .collect::<Vec<_>>();
    assert!(!input.is_empty());

    let mut count = 0;
    for v in input.iter() {
        for i in &v.output {
            match i.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
    }

    println!("{}", count);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_from_str() -> Result<()> {
        aoc_utils::init_test_logger()?;

        let input = Input::from_str(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )?;

        debug!("{:?}", input);

        Ok(())
    }
}
