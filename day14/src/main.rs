use anyhow::{anyhow, Result};
use log::{debug, trace, LevelFilter};
use std::collections::HashMap;

#[derive(Debug)]
struct Manual {
    original_polymer: String,
    polymer: HashMap<String, usize>,
    insertion_rules: HashMap<String, String>,
}

impl Manual {
    fn step(&mut self) {
        let mut new_polymer = HashMap::new();

        for (pair, ct) in self.polymer.iter() {
            if let Some(element) = self.insertion_rules.get(pair) {
                let new_a = pair[0..1].to_string() + element;
                let new_b = element.to_string() + &pair[1..2];

                *new_polymer.entry(new_a).or_insert(0) += ct;
                *new_polymer.entry(new_b).or_insert(0) += ct;
            }
        }

        trace!("Prev: {:?}", self.polymer);
        self.polymer = new_polymer;
        trace!("Next: {:?}", self.polymer);
    }

    fn step_by(&mut self, by: usize) {
        for s in 0..by {
            debug!("Step {}", s + 1);
            self.step();
        }
    }

    #[cfg(test)]
    fn polymer_len(&self) -> usize {
        self.polymer.iter().map(|(_, ct)| ct).sum::<usize>() + 1
    }

    fn counts(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        self.polymer.iter().for_each(|(pair, ct)| {
            for c in pair.chars() {
                *counts.entry(c).or_insert(0) += *ct;
            }
        });

        counts.iter_mut().for_each(|(_, ct)| *ct /= 2);

        *counts
            .get_mut(&self.original_polymer.chars().next().unwrap())
            .unwrap() += 1;

        *counts
            .get_mut(&self.original_polymer.chars().last().unwrap())
            .unwrap() += 1;

        counts
    }
}

impl TryFrom<Vec<String>> for Manual {
    type Error = anyhow::Error;

    fn try_from(mut value: Vec<String>) -> Result<Self, Self::Error> {
        let original_polymer = value.remove(0);
        let polymer = {
            let mut m = HashMap::new();
            for i in 0..original_polymer.len() - 1 {
                let pair = original_polymer[i..=i + 1].to_string();
                *(m.entry(pair).or_insert(0)) += 1;
            }
            m
        };

        let mut insertion_rules = HashMap::new();

        for line in value.into_iter().filter(|l| !l.is_empty()) {
            let (from, to) = line
                .split_once(" -> ")
                .ok_or_else(|| anyhow!("Failed to parse {}", line))?;

            insertion_rules.insert(from.to_string(), to.to_string());
        }

        Ok(Self {
            original_polymer,
            polymer,
            insertion_rules,
        })
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let mut manual = Manual::try_from(aoc_utils::input()?)?;
    manual.step_by(40);

    let counts = manual.counts();
    debug!("{:?}", counts);

    let most_common = counts.iter().max_by_key(|(_, v)| **v).unwrap();
    let least_common = counts.iter().min_by_key(|(_, v)| **v).unwrap();

    println!("{}", most_common.1 - least_common.1);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step() -> Result<()> {
        aoc_utils::init_test_logger()?;

        let mut manual = Manual::try_from(aoc_utils::test_input()?)?;

        manual.step_by(5);
        assert_eq!(97, manual.polymer_len());

        manual.step_by(5);
        assert_eq!(3073, manual.polymer_len());

        let counts = manual.counts();
        debug!("{:?}", counts);

        assert_eq!(1749, *counts.get(&'B').unwrap());
        assert_eq!(298, *counts.get(&'C').unwrap());
        assert_eq!(161, *counts.get(&'H').unwrap());
        assert_eq!(865, *counts.get(&'N').unwrap());

        Ok(())
    }
}
