use anyhow::Result;
use env_logger::Target;
use log::{debug, trace, LevelFilter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

const PATH: &str = "input/day03.txt";

fn power_consumption() -> Result<u32> {
    let mut line_ct = 0;
    let mut ones_ct: Vec<u32> = Vec::new();
    for line in BufReader::new(File::open(&Path::new(PATH))?).lines() {
        let line = line?;

        trace!("Input: {}", line);
        for (i, c) in line.chars().enumerate() {
            if ones_ct.len() < i + 1 {
                ones_ct.push(0);
            }
            ones_ct[i] += if c == '1' { 1 } else { 0 };
        }

        line_ct += 1;
    }

    debug!("Ones Count: {:?}, Line Count: {}", ones_ct, line_ct);

    let gamma: String = ones_ct
        .iter()
        .map(|i| if *i >= line_ct / 2 { '1' } else { '0' })
        .collect();
    let epsilon: String = gamma
        .chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect();

    let gamma = u32::from_str_radix(&gamma, 2)?;
    let epsilon = u32::from_str_radix(&epsilon, 2)?;

    debug!("Gamma:  {:0b} ({})", gamma, gamma);
    debug!("Epsilon {:0b} ({})", epsilon, epsilon);

    Ok(gamma * epsilon)
}

fn main() -> Result<()> {
    env_logger::builder()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .try_init()?;

    println!("{}", power_consumption()?);

    Ok(())
}
