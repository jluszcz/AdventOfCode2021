use anyhow::Result;
use log::{debug, trace, LevelFilter};
use std::str::FromStr;

#[derive(Default, Debug, PartialEq, Eq)]
struct Fish {
    inner: [u64; 9],
}

impl Fish {
    const BIRTH_INTERVAL: usize = 7;

    fn advance(&mut self) {
        trace!("Pre:  {:?}, count={}", self.inner, self.count());

        let to_birth = self.inner[0];

        for i in 0..(self.inner.len() - 1) {
            self.inner[i] = self.inner[i + 1];
        }

        // Reset fish which just gave birth
        self.inner[Self::BIRTH_INTERVAL - 1] += to_birth;

        // Create new fish at the last position
        self.inner[self.inner.len() - 1] = to_birth;

        debug!("Post: {:?}, count={}", self.inner, self.count());
    }

    fn count(&self) -> u64 {
        self.inner.iter().sum()
    }
}

impl FromStr for Fish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fish = Self::default();

        s.split(',')
            .filter_map(|s| u32::from_str(s).ok())
            .for_each(|i| fish.inner[i as usize] += 1);

        Ok(fish)
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let lines = aoc_utils::read_lines(aoc_utils::INPUT_PATH)?;
    assert_eq!(1, lines.len());

    let mut fish = Fish::from_str(&lines[0])?;

    for _ in 0..256 {
        fish.advance();
    }

    println!("{}", fish.count());

    Ok(())
}
