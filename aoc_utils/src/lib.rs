use anyhow::{anyhow, Result};
use env_logger::Target;
use log::LevelFilter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_PATH: &str = "input/input.txt";
const TEST_INPUT_PATH: &str = "input/test.txt";

pub fn init_logger(level: LevelFilter) -> Result<()> {
    inner_init_logger(Some(level), false)
}

pub fn init_test_logger() -> Result<()> {
    inner_init_logger(Some(LevelFilter::Debug), true)
}

fn inner_init_logger(level: Option<LevelFilter>, is_test: bool) -> Result<()> {
    let _ = env_logger::builder()
        .target(Target::Stdout)
        .filter_level(level.unwrap_or(LevelFilter::Info))
        .is_test(is_test)
        .try_init();

    Ok(())
}

pub fn input() -> Result<Vec<String>> {
    read_lines(INPUT_PATH)
}

pub fn test_input() -> Result<Vec<String>> {
    read_lines(TEST_INPUT_PATH)
}

fn read_lines(path: &'static str) -> Result<Vec<String>> {
    let lines: Vec<_> = BufReader::new(File::open(&Path::new(path))?)
        .lines()
        .filter_map(|l| l.ok())
        .collect();

    if !lines.is_empty() {
        Ok(lines)
    } else {
        Err(anyhow!("No input: {}", path))
    }
}
