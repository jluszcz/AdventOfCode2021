use anyhow::{anyhow, Result};
use log::{trace, LevelFilter};
use std::fmt;
use std::str::FromStr;

#[derive(Default)]
struct Octopuses {
    grid: [[usize; 10]; 10],
    step_ct: usize,
    flash_ct: usize,
}

impl Octopuses {
    fn step(&mut self) {
        self.step_ct += 1;

        trace!("Step {}", self.step_ct);

        let mut flashed = [[false; 10]; 10];

        let mut to_flash = Vec::new();

        // First, the energy level of each octopus increases by 1
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                self.grid[y][x] += 1;
                if self.grid[y][x] > 9 {
                    to_flash.push((x, y));
                }
            }
        }

        // Then, any octopus with an energy level greater than 9 flashes, increasing adjacent octopus values by 1.
        while let Some((x, y)) = to_flash.pop() {
            if flashed[y][x] {
                continue;
            }

            flashed[y][x] = true;

            for (n_x, n_y) in self.neighbors(x, y) {
                self.grid[n_y][n_x] += 1;
                if self.grid[n_y][n_x] > 9 && !flashed[n_y][n_x] {
                    to_flash.push((n_x, n_y));
                }
            }
        }

        // Finally, any octopus that flashed has its energy level set to 0.
        for (y, row) in flashed.iter().enumerate() {
            for (x, flashed) in row.iter().enumerate() {
                if *flashed {
                    self.flash_ct += 1;
                    self.grid[y][x] = 0;
                }
            }
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        // Below
        {
            let y = y + 1;
            if self.grid.get(y).and_then(|r| r.get(x)).is_some() {
                neighbors.push((x, y));

                // Lower Right
                if self.grid[y].get(x + 1).is_some() {
                    neighbors.push((x + 1, y));
                }

                // Lower Left
                if let Some(x) = x.checked_sub(1) {
                    neighbors.push((x, y));
                }
            }
        }

        // Above
        if let Some(y) = y.checked_sub(1) {
            neighbors.push((x, y));

            // Upper Right
            if self.grid[y].get(x + 1).is_some() {
                neighbors.push((x + 1, y));
            }

            // Upper Left
            if let Some(x) = x.checked_sub(1) {
                neighbors.push((x, y));
            }
        }

        // Right
        if self.grid.get(y).and_then(|r| r.get(x + 1)).is_some() {
            neighbors.push((x + 1, y));
        }

        // Left
        if let Some(x) = x.checked_sub(1) {
            neighbors.push((x, y));
        }

        neighbors
    }
}

impl fmt::Debug for Octopuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for row in &self.grid {
            writeln!(
                f,
                "\t{}",
                row.iter().map(|i| i.to_string()).collect::<String>()
            )?;
        }
        writeln!(f, "}}")
    }
}

impl TryFrom<Vec<String>> for Octopuses {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() != 10 {
            return Err(anyhow!("Expected 10 lines: {}", value.len()));
        }

        let mut octopuses = Octopuses::default();
        for (y, row) in value.into_iter().enumerate() {
            let row_len = row
                .chars()
                .filter_map(|c| usize::from_str(&c.to_string()).ok())
                .enumerate()
                .inspect(|(x, v)| octopuses.grid[y][*x] = *v)
                .count();

            if row_len != 10 {
                return Err(anyhow!("Expected 10 items in row {}: {}", y, row_len));
            }
        }

        Ok(octopuses)
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let mut octopuses = Octopuses::try_from(aoc_utils::input()?)?;

    for _ in 0..100 {
        octopuses.step();
    }

    println!("{}", octopuses.flash_ct);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    impl PartialEq<&'static str> for Octopuses {
        fn eq(&self, other: &&'static str) -> bool {
            for (y, line) in other.lines().map(|l| l.trim()).enumerate() {
                for (x, c) in line.chars().enumerate() {
                    let val = usize::from_str(&c.to_string()).unwrap();
                    if val != self.grid[y][x] {
                        return false;
                    }
                }
            }
            true
        }
    }

    #[test]
    fn test_neighbors() {
        let octopuses = Octopuses::default();

        fn assert_eq_ignore_order(
            mut expected: Vec<(usize, usize)>,
            mut neighbors: Vec<(usize, usize)>,
        ) {
            expected.sort_unstable();
            neighbors.sort_unstable();
            assert_eq!(expected, neighbors);
        }

        assert_eq_ignore_order(vec![(1, 0), (0, 1), (1, 1)], octopuses.neighbors(0, 0));

        assert_eq_ignore_order(
            vec![(4, 0), (6, 0), (5, 1), (4, 1), (6, 1)],
            octopuses.neighbors(5, 0),
        );

        assert_eq_ignore_order(vec![(8, 0), (9, 1), (8, 1)], octopuses.neighbors(9, 0));

        assert_eq_ignore_order(
            vec![(0, 4), (0, 6), (1, 5), (1, 4), (1, 6)],
            octopuses.neighbors(0, 5),
        );

        assert_eq_ignore_order(vec![(0, 8), (1, 9), (1, 8)], octopuses.neighbors(0, 9));

        assert_eq_ignore_order(
            vec![
                (3, 3),
                (3, 4),
                (3, 5),
                (4, 3),
                (4, 5),
                (5, 3),
                (5, 4),
                (5, 5),
            ],
            octopuses.neighbors(4, 4),
        );

        assert_eq_ignore_order(vec![(8, 8), (9, 8), (8, 9)], octopuses.neighbors(9, 9));
    }

    #[test]
    fn test_steps() -> Result<()> {
        aoc_utils::init_test_logger()?;

        let mut octopuses = Octopuses::try_from(aoc_utils::test_input()?)?;

        octopuses.step();

        assert_eq!(
            octopuses,
            r#"6594254334
            3856965822
            6375667284
            7252447257
            7468496589
            5278635756
            3287952832
            7993992245
            5957959665
            6394862637"#
        );

        octopuses.step();

        assert_eq!(
            octopuses,
            r#"8807476555
            5089087054
            8597889608
            8485769600
            8700908800
            6600088989
            6800005943
            0000007456
            9000000876
            8700006848"#
        );

        octopuses.step();

        assert_eq!(
            octopuses,
            r#"0050900866
            8500800575
            9900000039
            9700000041
            9935080063
            7712300000
            7911250009
            2211130000
            0421125000
            0021119000"#
        );

        // Step 4
        octopuses.step();

        assert_eq!(
            octopuses,
            r#"2263031977
            0923031697
            0032221150
            0041111163
            0076191174
            0053411122
            0042361120
            5532241122
            1532247211
            1132230211"#
        );

        // Step 10
        for _ in 4..10 {
            octopuses.step();
        }

        assert_eq!(
            octopuses,
            r#"0481112976
            0031112009
            0041112504
            0081111406
            0099111306
            0093511233
            0442361130
            5532252350
            0532250600
            0032240000"#
        );
        assert_eq!(204, octopuses.flash_ct);

        for _ in 10..100 {
            octopuses.step();
        }
        assert_eq!(1656, octopuses.flash_ct);

        Ok(())
    }
}
