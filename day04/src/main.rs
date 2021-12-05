use anyhow::{anyhow, Result};
use log::{debug, LevelFilter};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Default)]
struct Space {
    value: u32,
    marked: bool,
}

impl From<u32> for Space {
    fn from(value: u32) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:02} {}]",
            self.value,
            if self.marked { "✅" } else { "❌" }
        )
    }
}

#[derive(Default)]
struct Board {
    board: [[Space; 5]; 5],
    position_map: HashMap<u32, (usize, usize)>,
    winning_play: Option<u32>,
}

impl Board {
    fn mark(&mut self, value: u32) -> bool {
        if !self.won() {
            if let Some((x, y)) = self.position_map.get(&value) {
                self.board[*x][*y].marked = true;

                if self.check_win_conditions() {
                    self.winning_play = Some(value);
                    return true;
                }
            }
        }
        false
    }

    fn won(&self) -> bool {
        self.winning_play.is_some()
    }

    fn check_win_conditions(&self) -> bool {
        for row in &self.board {
            if row.iter().all(|s| s.marked) {
                return true;
            }
        }

        for col in 0..self.board.len() {
            if self.board.iter().all(|s| s[col].marked) {
                return true;
            }
        }

        false
    }

    fn value(&self) -> u32 {
        assert!(self.won());

        let unmarked_sum: u32 = self
            .board
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|s| if s.marked { None } else { Some(s.value) })
                    .sum::<u32>()
            })
            .sum();

        unmarked_sum * self.winning_play.unwrap()
    }
}

impl TryFrom<&[String]> for Board {
    type Error = anyhow::Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let mut board = Board::default();

        for (x, line) in value.iter().enumerate() {
            for (y, val) in line.split_ascii_whitespace().enumerate() {
                let space_val = u32::from_str(val)?;

                let prev = board.position_map.insert(space_val, (x, y));
                if prev.is_some() {
                    return Err(anyhow!("Duplicate value: {}", space_val));
                }

                board.board[x][y] = space_val.into();
            }
        }

        Ok(board)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for row in &self.board {
            writeln!(f, "\t{:?}", row)?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let mut lines = aoc_utils::read_lines(aoc_utils::INPUT_PATH)?;

    // First line is the plays
    let plays: Vec<u32> = lines
        .remove(0)
        .split(',')
        .map(|s| u32::from_str(s).unwrap())
        .collect();
    debug!("Plays: {:?}", plays);

    let mut boards = Vec::new();
    // Skip the second empty line and advance by 6 to account for the 5x5 board plus a newline
    for i in (1..lines.len()).into_iter().step_by(6) {
        let board = Board::try_from(&lines[i..i + 5])?;
        debug!("{:?}", board);
        boards.push(board);
    }

    let mut last_winner: Option<usize> = None;
    for play in plays.into_iter() {
        for (board_idx, board) in boards.iter_mut().enumerate() {
            if board.mark(play) {
                last_winner = Some(board_idx);
            }
        }
    }

    assert!(last_winner.is_some());
    let last_winning_board = &boards[last_winner.unwrap()];
    println!("{}", last_winning_board.value());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_board() -> Result<()> {
        aoc_utils::init_test_logger()?;

        let board = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let parsed = Board::try_from(board.as_ref())?;
        debug!("{:?}", parsed);

        Ok(())
    }

    #[test]
    fn test_winning_row() -> Result<()> {
        aoc_utils::init_test_logger()?;

        let board = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let mut board = Board::try_from(board.as_ref())?;

        board.mark(8);
        board.mark(2);
        board.mark(23);
        board.mark(4);
        board.mark(24);

        assert!(board.check_win_conditions());

        Ok(())
    }

    #[test]
    fn test_winning_column() -> Result<()> {
        aoc_utils::init_test_logger()?;

        let board = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        let mut board = Board::try_from(board.as_ref())?;

        board.mark(17);
        board.mark(23);
        board.mark(14);
        board.mark(3);
        board.mark(20);

        assert!(board.check_win_conditions());

        Ok(())
    }
}
