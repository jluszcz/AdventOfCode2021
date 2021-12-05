use anyhow::{anyhow, Result};
use log::{debug, trace, LevelFilter};
use std::cmp::{max, min};
use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| anyhow!("Delimiter not found in {}", s))?;

        Ok(Point {
            x: u32::from_str(x)?,
            y: u32::from_str(y)?,
        })
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Vent {
    start: Point,
    end: Point,
}

impl FromStr for Vent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("Start/End delimiter not found in {}", s))?;

        Ok(Vent {
            start: Point::from_str(start)?,
            end: Point::from_str(end)?,
        })
    }
}

impl fmt::Debug for Vent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.start, self.end)
    }
}

struct Grid {
    inner: Vec<Vec<u32>>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Self {
        Self {
            inner: vec![vec![0; width as usize + 1]; height as usize + 1],
        }
    }

    fn overlap_ct(&self) -> usize {
        self.inner
            .iter()
            .map(|r| r.iter().filter(|v| **v > 1).count())
            .sum()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for row in &self.inner {
            writeln!(
                f,
                "\t{}",
                row.iter()
                    .map(|i| if *i == 0 {
                        ".".to_string()
                    } else {
                        i.to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            )?;
        }
        writeln!(f, "}}")
    }
}

impl Index<usize> for Grid {
    type Output = [u32];

    fn index(&self, index: usize) -> &Self::Output {
        self.inner[index].as_ref()
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.inner[index].as_mut()
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let vents = aoc_utils::read_lines(aoc_utils::INPUT_PATH)?
        .into_iter()
        .map(|l| Vent::from_str(&l).ok())
        .flatten()
        .collect::<Vec<_>>();

    assert!(!vents.is_empty());

    let width = vents.iter().map(|v| max(v.start.x, v.end.x)).max().unwrap();
    let height = vents.iter().map(|v| max(v.start.y, v.end.y)).max().unwrap();
    debug!("Width: {}, Height: {}", width, height);

    let mut grid = Grid::new(width, height);
    for vent in vents.iter() {
        let (start, end) = (&vent.start, &vent.end);
        if start.x == end.x {
            let x = start.x as usize;
            let start_y = min(start.y, end.y) as usize;
            let end_y = max(start.y, end.y) as usize;
            for y in start_y..=end_y {
                grid[y][x] += 1;
            }
        } else if start.y == end.y {
            let y = start.y as usize;
            let start_x = min(start.x, end.x) as usize;
            let end_x = max(start.x, end.x) as usize;
            for x in start_x..=end_x {
                grid[y][x] += 1;
            }
        } else {
            debug!("Diagonal: {:?}", vent);
            continue;
        }
        debug!("Applied: {:?}", vent);
        trace!("Grid: {:?}", grid);
    }

    debug!("Grid: {:?}", grid);

    println!("{}", grid.overlap_ct());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vent() -> Result<()> {
        aoc_utils::init_test_logger()?;

        let vent = Vent::from_str("0,9 -> 5,9")?;
        debug!("{:?}", vent);

        Ok(())
    }
}
