use anyhow::{anyhow, Result};
use log::{debug, info, LevelFilter};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

#[derive(Default)]
struct Transparency {
    grid: Vec<Vec<bool>>,
    folds: Vec<Fold>,
}

impl Transparency {
    fn fold(&mut self) {
        if let Some(fold) = self.folds.pop() {
            match fold {
                Fold::Vertical(at) => {
                    let mut bottom = self.grid.get_mut(at..).unwrap().to_vec();
                    bottom.reverse();

                    for (y, row) in bottom.iter().enumerate() {
                        for (x, val) in row.iter().enumerate() {
                            self.grid[y][x] |= *val;
                        }
                    }

                    self.grid.truncate(at);
                }
                Fold::Horizontal(at) => {
                    for row in self.grid.iter_mut() {
                        let right = row.drain(at + 1..).collect::<Vec<_>>();
                        for (i, val) in right.iter().rev().enumerate() {
                            row[i] |= *val;
                        }
                    }
                }
            }
        }
    }

    fn count_visible(&self) -> usize {
        self.grid
            .iter()
            .map(|r| r.iter().filter(|b| **b).count())
            .sum()
    }
}

impl TryFrom<Vec<String>> for Transparency {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut transparency = Self::default();

        let mut points = Vec::new();
        for row in value {
            if let Some((x, y)) = row.split_once(',') {
                points.push((usize::from_str(x)?, usize::from_str(y)?));
            } else if let Some((_, fold)) = row.split_once("fold along ") {
                if let Some((dir, val)) = fold.split_once('=') {
                    let val = usize::from_str(val)?;
                    if "x" == dir {
                        transparency.folds.push(Fold::Horizontal(val));
                    } else if "y" == dir {
                        transparency.folds.push(Fold::Vertical(val));
                    } else {
                        return Err(anyhow!("Unexpected direction: {}", row));
                    }
                } else {
                    return Err(anyhow!("Unexpected direction: '{}'", row));
                }
            } else if row.is_empty() {
                continue;
            } else {
                return Err(anyhow!("Unexpected line: '{}'", row));
            }
        }

        let max_x = points.iter().map(|(x, _)| *x).max().unwrap();
        let max_y = points.iter().map(|(_, y)| *y).max().unwrap();

        for _ in 0..=max_y {
            transparency.grid.push(vec![false; max_x + 1]);
        }

        for (x, y) in points {
            transparency.grid[y][x] = true;
        }

        // Reverse the folds so we can use pop() on them
        transparency.folds.reverse();

        Ok(transparency)
    }
}

impl fmt::Debug for Transparency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid: {{")?;
        for row in &self.grid {
            writeln!(
                f,
                "\t{}",
                row.iter()
                    .map(|x| if *x { "# " } else { ". " }.trim())
                    .collect::<String>()
            )?;
        }
        writeln!(f, "}}")?;
        writeln!(f, "")?;

        let mut folds = self.folds.clone();
        folds.reverse();
        writeln!(f, "Folds: {:?}", folds)
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let mut transparency = Transparency::try_from(aoc_utils::input()?)?;
    transparency.fold();
    debug!("{:?}", transparency);
    info!("Count: {}", transparency.count_visible());

    Ok(())
}
