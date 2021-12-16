use anyhow::Result;
use core::fmt;
use log::{debug, log_enabled, trace, Level, LevelFilter};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Default)]
struct Cave {
    grid: Vec<Vec<usize>>,
}

impl Cave {
    fn least_risky_path(&self) -> (Vec<(usize, usize)>, usize) {
        debug!(
            "Calculating path through {}x{} cave",
            self.grid.len(),
            self.grid.len()
        );

        let mut unvisited = HashSet::new();
        let mut distances = HashMap::new();
        let mut prev = HashMap::new();

        let target = (self.grid.len() - 1, self.grid.len() - 1);

        for (y, row) in self.grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                unvisited.insert((x, y));
                distances.insert((x, y), usize::MAX);
            }
        }
        distances.insert((0, 0), 0);

        let progress_interval = distances.len() / 20;

        trace!("Remaining unvisited: {}", unvisited.len());
        while !unvisited.is_empty() {
            let curr = *unvisited
                .iter()
                .min_by_key(|pt| distances.get(pt).unwrap())
                .unwrap();

            unvisited.remove(&curr);

            if log_enabled!(Level::Trace) && unvisited.len() % progress_interval == 0 {
                trace!("Remaining unvisited: {}", unvisited.len());
            }

            if curr == target {
                break;
            }

            let curr_dist = *distances.get(&curr).unwrap();

            for neighbor in aoc_utils::grid_neighbors(&self.grid, curr.0, curr.1, false) {
                if !unvisited.contains(&neighbor) {
                    continue;
                }

                let neighbor_dist = *distances.get(&neighbor).unwrap();
                let dist = curr_dist + self.grid[neighbor.1][neighbor.0];
                if dist < neighbor_dist {
                    distances.insert(neighbor, dist);
                    prev.insert(neighbor, curr);
                }
            }
        }

        let mut path = vec![target];

        let mut curr = target;
        while let Some(prev) = prev.get(&curr) {
            let prev = *prev;
            if prev == (0, 0) {
                break;
            }

            path.push(prev);
            curr = prev;
        }
        path.reverse();

        let risk = path.iter().map(|(x, y)| self.grid[*y][*x]).sum();

        debug!("Path Length: {}, Risk: {}", path.len(), risk);
        (path, risk)
    }
}

impl TryFrom<Vec<String>> for Cave {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut cave = Cave::default();

        // Load up the initial top-left grid
        for row in value {
            cave.grid.push(
                row.chars()
                    .filter_map(|c| usize::from_str(&c.to_string()).ok())
                    .collect(),
            );
        }

        // Extend the grid across
        for row in cave.grid.iter_mut() {
            let orig_len = row.len();
            for i in 0..4 {
                let mut next = row[i * orig_len..(i + 1) * orig_len]
                    .iter()
                    .map(|v| *v % 9 + 1)
                    .collect::<Vec<_>>();
                row.append(&mut next);
            }
        }

        // Extend the grid down
        let orig_len = cave.grid.len();
        for i in 0..4 {
            let mut next = Vec::new();
            for row in cave.grid[i * orig_len..(i + 1) * orig_len].iter() {
                next.push(row.iter().map(|v| *v % 9 + 1).collect::<Vec<_>>());
            }
            cave.grid.append(&mut next);
        }

        Ok(cave)
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cave: {{")?;
        for row in self.grid.iter() {
            writeln!(
                f,
                "\t{}",
                row.iter().map(|v| v.to_string()).collect::<String>()
            )?;
        }
        writeln!(f, "}}")
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let cave = Cave::try_from(aoc_utils::input()?)?;
    let (_, risk) = cave.least_risky_path();

    println!("{}", risk);

    Ok(())
}
