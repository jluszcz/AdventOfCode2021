use anyhow::Result;
use log::{debug, LevelFilter};
use std::fmt;
use std::str::FromStr;

struct HeightMap {
    inner: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl HeightMap {
    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if self.inner.get(y + 1).and_then(|r| r.get(x)).is_some() {
            neighbors.push((x, y + 1));
        }

        if let Some(neighbor_y) = y.checked_sub(1) {
            neighbors.push((x, neighbor_y));
        }

        if self.inner.get(y).and_then(|r| r.get(x + 1)).is_some() {
            neighbors.push((x + 1, y));
        }

        if let Some(neighbor_x) = x.checked_sub(1) {
            neighbors.push((neighbor_x, y));
        }

        neighbors
    }

    fn is_low_spot(&self, x: usize, y: usize) -> bool {
        let spot = self.inner[y][x];

        self.neighbors(x, y)
            .into_iter()
            .all(|(x, y)| self.inner[y][x] > spot)
    }

    fn risk_level(&self, x: usize, y: usize) -> u32 {
        if self.is_low_spot(x, y) {
            let risk = 1 + self.inner[y][x];
            debug!("Risk of ({}, {}): {}", x, y, risk);
            risk
        } else {
            0
        }
    }
}

impl fmt::Debug for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for row in self.inner.iter() {
            writeln!(
                f,
                "\t{}",
                row.iter().map(|i| i.to_string()).collect::<String>()
            )?;
        }
        writeln!(f, "}}")
    }
}

impl TryFrom<Vec<String>> for HeightMap {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut rows = Vec::new();

        value
            .into_iter()
            .map(|l| {
                l.chars()
                    .filter_map(|c| u32::from_str(&c.to_string()).ok())
                    .collect::<Vec<_>>()
            })
            .for_each(|l| rows.push(l));

        let width = rows[0].len();
        for row in rows.iter().skip(1) {
            assert_eq!(width, row.len());
        }

        Ok(Self {
            height: rows.len(),
            width,
            inner: rows,
        })
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let height_map = HeightMap::try_from(aoc_utils::input()?)?;
    debug!("{:?}", height_map);

    let mut risk_level = 0;
    for y in 0..height_map.height {
        for x in 0..height_map.width {
            risk_level += height_map.risk_level(x, y);
        }
    }

    println!("{}", risk_level);

    Ok(())
}
