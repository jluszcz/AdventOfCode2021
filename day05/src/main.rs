use anyhow::{anyhow, Result};
use log::{debug, trace, LevelFilter};
use std::cmp::{max, min, Ordering};
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        let (first, second) = s
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("Start/End delimiter not found in {}", s))?;

        let first = Point::from_str(first)?;
        let second = Point::from_str(second)?;

        let start = min(first, second);
        let end = max(first, second);

        Ok(Vent { start, end })
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

    fn track_vent(&mut self, vent: &Vent) {
        let (start, end) = (&vent.start, &vent.end);
        if start.x == end.x {
            let x = start.x as usize;
            for y in start.y..=end.y {
                self.inner[y as usize][x] += 1;
            }
        } else if start.y == end.y {
            let y = start.y as usize;
            for x in start.x..=end.x {
                self.inner[y][x as usize] += 1;
            }
        } else {
            let mut x = start.x;
            let mut y = start.y;
            while x != end.x && y != end.y {
                self.inner[y as usize][x as usize] += 1;

                match start.x.cmp(&end.x) {
                    Ordering::Less => x += 1,
                    Ordering::Greater => x -= 1,
                    Ordering::Equal => (),
                }

                match start.y.cmp(&end.y) {
                    Ordering::Less => y += 1,
                    Ordering::Greater => y -= 1,
                    Ordering::Equal => (),
                }
            }
            self.inner[y as usize][x as usize] += 1;
        }
        debug!("Applied: {:?}", vent);
        trace!("Grid: {:?}", self);
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
        grid.track_vent(vent);
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
