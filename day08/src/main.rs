use anyhow::{anyhow, Result};
use log::{debug, log_enabled, trace, Level, LevelFilter};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Input {
    signal_patterns: Vec<Vec<char>>,
    output: Vec<String>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signal_patterns, output) = s
            .split_once(" | ")
            .ok_or_else(|| anyhow!("Failed to split {}", s))?;

        // "Normalize" all the patterns and output by sorting the Strings

        let signal_patterns = signal_patterns
            .split_ascii_whitespace()
            .map(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                chars.sort_unstable();

                chars
            })
            .collect::<Vec<_>>();

        if signal_patterns.len() != 10 {
            return Err(anyhow!("{:?} should have 10 values", signal_patterns));
        }

        let output = output
            .split_ascii_whitespace()
            .map(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                chars.sort_unstable();

                chars.into_iter().collect::<String>()
            })
            .collect::<Vec<_>>();

        if output.len() != 4 {
            return Err(anyhow!("{:?} should have 4 values", output));
        }

        Ok(Self {
            signal_patterns,
            output,
        })
    }
}

impl TryFrom<Input> for u32 {
    type Error = anyhow::Error;

    fn try_from(input: Input) -> Result<Self, Self::Error> {
        let one = input.signal_patterns.iter().find(|l| l.len() == 2).unwrap();
        let four = input.signal_patterns.iter().find(|l| l.len() == 4).unwrap();
        let seven = input.signal_patterns.iter().find(|l| l.len() == 3).unwrap();
        let eight = input.signal_patterns.iter().find(|l| l.len() == 7).unwrap();

        // 1 and 7 lets us find segment 'a'
        let a = seven.iter().find(|c| !one.contains(*c)).unwrap();

        // 1 and 4 lets us find segments 'b' and 'd'
        let mut b_and_d = four
            .iter()
            .filter(|c| !one.contains(*c))
            .collect::<Vec<_>>();
        b_and_d.sort_unstable();

        // Remaining 5-segment Digits: 2, 3, 5
        let five_segment_digits = input
            .signal_patterns
            .iter()
            .filter(|l| l.len() == 5)
            .collect::<Vec<_>>();

        // Of 2, 3, and 5, only 5 contains 'b'
        let five = *five_segment_digits
            .iter()
            .find(|l| l.contains(b_and_d[0]) && l.contains(b_and_d[1]))
            .unwrap();

        let mut two_or_three = five_segment_digits
            .iter()
            .filter(|l| **l != five)
            .collect::<Vec<_>>();

        let (b, d) = if two_or_three[0].contains(b_and_d[0]) {
            (b_and_d[1], b_and_d[0])
        } else {
            (b_and_d[0], b_and_d[1])
        };

        let c = one.iter().find(|c| !five.contains(*c)).unwrap();
        let f = one.iter().find(|c| five.contains(*c)).unwrap();

        // Between 2 and 3, 2 has 'e' and 3 has 'f'
        let mut three = None;
        for i in 0..two_or_three.len() {
            if two_or_three[i].contains(f) {
                three = Some(*two_or_three.remove(i));
                break;
            }
        }
        let three = three.unwrap();

        let two = *two_or_three[0];

        let e = two.iter().find(|i| !three.contains(*i)).unwrap();
        let g = eight
            .iter()
            .find(|i| *i != a && *i != b && *i != c && *i != d && *i != e && *i != f)
            .unwrap();

        let mut zero = vec![a, b, c, e, f, g];
        zero.sort_unstable();

        let mut four = vec![b, c, d, f];
        four.sort_unstable();

        let mut six = vec![a, b, d, e, f, g];
        six.sort_unstable();

        let mut nine = vec![a, b, c, d, f, g];
        nine.sort_unstable();

        let mut digit_map = HashMap::new();
        digit_map.insert(zero.iter().copied().collect::<String>(), '0');
        digit_map.insert(one.iter().copied().collect::<String>(), '1');
        digit_map.insert(two.iter().copied().collect::<String>(), '2');
        digit_map.insert(three.iter().copied().collect::<String>(), '3');
        digit_map.insert(four.iter().copied().collect::<String>(), '4');
        digit_map.insert(five.iter().copied().collect::<String>(), '5');
        digit_map.insert(six.iter().copied().collect::<String>(), '6');
        digit_map.insert(seven.iter().copied().collect::<String>(), '7');
        digit_map.insert(eight.iter().copied().collect::<String>(), '8');
        digit_map.insert(nine.iter().copied().collect::<String>(), '9');

        if log_enabled!(Level::Trace) {
            let mut digit_map = digit_map
                .iter()
                .map(|(k, v)| format!("{} -> {}", v, k))
                .collect::<Vec<_>>();
            digit_map.sort_unstable();

            trace!("{:#?}", digit_map)
        }

        let output = input
            .output
            .into_iter()
            .map(|s| *digit_map.get(&s).unwrap())
            .collect::<String>();

        Ok(u32::from_str(&output)?)
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let output = aoc_utils::input()?
        .into_iter()
        .filter_map(|l| Input::from_str(&l).ok())
        .filter_map(|i| u32::try_from(i).ok())
        .inspect(|i| debug!("{}", i))
        .sum::<u32>();

    println!("{}", output);

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
