use anyhow::Result;
use env_logger::Target;
use log::{debug, LevelFilter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

const PATH: &str = "input/day03.txt";

fn get_ones_cts(lines: &[String]) -> Vec<u32> {
    let mut ones_cts = Vec::new();
    for line in lines.iter() {
        for (i, c) in line.chars().enumerate() {
            if ones_cts.len() < i + 1 {
                ones_cts.push(0);
            }
            ones_cts[i] += if c == '1' { 1 } else { 0 };
        }
    }
    ones_cts
}

fn get_ones_cts_and_lines() -> Result<(Vec<u32>, Vec<String>)> {
    let lines: Vec<String> = BufReader::new(File::open(&Path::new(PATH))?)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let ones_cts = get_ones_cts(&lines);

    Ok((ones_cts, lines))
}

fn _power_consumption() -> Result<u32> {
    let (ones_ct, lines) = get_ones_cts_and_lines()?;
    let line_ct = lines.len() as u32;

    let gamma: String = ones_ct
        .iter()
        .map(|i| if 2 * *i >= line_ct { '1' } else { '0' })
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

fn life_support() -> Result<u32> {
    let (ones_cts, lines) = get_ones_cts_and_lines()?;
    let ones_cts_len = ones_cts.len();

    let mut o2_ones_cts = ones_cts;
    let mut co2_ones_cts = o2_ones_cts.clone();

    let mut o2_lines = lines;
    let mut co2_lines = o2_lines.clone();

    for i in 0..ones_cts_len {
        let o2_lines_ct = o2_lines.len() as u32;
        if o2_lines.len() != 1 {
            let more_ones = 2 * o2_ones_cts[i] >= o2_lines_ct;
            let o2_filter = if more_ones { '1' } else { '0' };

            o2_lines = o2_lines
                .into_iter()
                .filter(|l| l.chars().nth(i).unwrap() == o2_filter)
                .collect();

            o2_ones_cts = get_ones_cts(&o2_lines);
        }

        if co2_lines.len() != 1 {
            let co2_lines_ct = co2_lines.len() as u32;
            let more_ones = 2 * co2_ones_cts[i] >= co2_lines_ct;
            let co2_filter = if more_ones { '0' } else { '1' };

            co2_lines = co2_lines
                .into_iter()
                .filter(|l| l.chars().nth(i).unwrap() == co2_filter)
                .collect();

            co2_ones_cts = get_ones_cts(&co2_lines);
        }

        if o2_lines.len() == 1 && co2_lines.len() == 1 {
            break;
        }
    }

    assert_eq!(1, o2_lines.len());
    assert_eq!(1, co2_lines.len());

    let o2 = u32::from_str_radix(&o2_lines[0], 2)?;
    let co2 = u32::from_str_radix(&co2_lines[0], 2)?;

    debug!("O2 Scubber Rating: {:b} {}", o2, o2);
    debug!("CO2 Scubber Rating: {:b} {}", co2, co2);

    Ok(o2 * co2)
}

fn main() -> Result<()> {
    env_logger::builder()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .try_init()?;

    println!("{}", life_support()?);

    Ok(())
}
