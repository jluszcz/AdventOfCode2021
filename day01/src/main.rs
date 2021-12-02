use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

const PATH: &str = "input/day01.txt";

fn _a() -> Result<()> {
    let path = Path::new(PATH);

    let mut increases = 0;
    let mut prev = None;
    for line in BufReader::new(File::open(&path)?).lines() {
        let depth: u32 = line?.parse()?;
        if let Some(prev) = prev {
            if depth > prev {
                increases += 1;
            }
        }
        prev = Some(depth);
    }

    println!("{}", increases);

    Ok(())
}

fn main() -> Result<()> {
    let path = Path::new(PATH);

    let mut increases = 0;

    let mut prev: Option<Vec<u32>> = None;
    let mut curr = vec![];
    for line in BufReader::new(File::open(&path)?).lines() {
        let depth: u32 = line?.parse()?;
        curr.push(depth);
        if curr.len() > 3 {
            curr = curr.split_off(1);
        }

        if curr.len() == 3 {
            let curr_sum: u32 = curr.iter().sum();
            if let Some(prev) = &prev {
                let prev_sum: u32 = prev.iter().sum();

                if curr_sum > prev_sum {
                    increases += 1;
                }
            }

            prev = Some(curr.clone());
        }
    }

    println!("{}", increases);
    Ok(())
}
