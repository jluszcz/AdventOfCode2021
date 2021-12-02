use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::str::FromStr;

const PATH: &str = "input/day02.txt";

#[derive(Debug, Clone, Copy)]
enum Action {
    Horizontal(i32),
    Vertical(i32),
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, amt) = s
            .split_once(" ")
            .ok_or_else(|| anyhow!("Failed to parse {}", s))?;

        let amt = amt.parse()?;

        if "forward".eq_ignore_ascii_case(action) {
            Ok(Self::Horizontal(amt))
        } else if "down".eq_ignore_ascii_case(action) {
            Ok(Self::Vertical(amt))
        } else if "up".eq_ignore_ascii_case(action) {
            Ok(Self::Vertical(-amt))
        } else {
            Err(anyhow!("Invalid action: {}", action))
        }
    }
}

#[derive(Debug, Default)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position {
    fn _apply_a(&mut self, action: Action) {
        match action {
            Action::Horizontal(amt) => self.x += amt,
            Action::Vertical(amt) => self.y += amt,
        }
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::Horizontal(amt) => {
                self.x += amt;
                self.y += self.aim * amt;
            }
            Action::Vertical(amt) => self.aim += amt,
        }
    }
}

fn main() -> Result<()> {
    let path = Path::new(PATH);

    let mut pos = Position::default();

    for line in BufReader::new(File::open(&path)?).lines() {
        pos.apply(line?.parse()?);
    }

    println!("{}", pos.x * pos.y);

    Ok(())
}
