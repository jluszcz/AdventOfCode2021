use anyhow::Result;
use log::{trace, LevelFilter};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;

fn _cost_part_one(distance: u32, weight: u32) -> u32 {
    distance * weight
}

fn cost_part_two(distance: u32, weight: u32) -> u32 {
    (distance * (distance + 1) / 2) * weight
}

fn main() -> Result<()> {
    aoc_utils::init_logger(LevelFilter::Info)?;

    let input = aoc_utils::input()?;
    assert_eq!(1, input.len());

    let input: Vec<_> = input[0]
        .split(',')
        .filter_map(|i| u32::from_str(i).ok())
        .collect();
    assert!(!input.is_empty());
    trace!("Input: {:?}", input);

    let mut min_pos = u32::MAX;
    let mut max_pos = u32::MIN;
    let mut weights = HashMap::new();
    for i in input.iter() {
        min_pos = min(min_pos, *i);
        max_pos = max(max_pos, *i);

        *weights.entry(*i).or_insert(0u32) += 1;
    }
    trace!("Weighted Input: {:?}", weights);

    // This feels like it can be improved - currently O(m * n) where m is (max_pos - min_pos) and n is count_unique(positions)
    let mut cost_for_pos = HashMap::new();
    for pos in min_pos..=max_pos {
        for (other_pos, weight) in weights.iter() {
            let other_pos = *other_pos;

            if pos == other_pos {
                continue;
            }

            let distance = max(pos, other_pos) - min(pos, other_pos);
            *cost_for_pos.entry(pos).or_insert(0u32) += cost_part_two(distance, *weight);
        }
    }
    trace!("Cost for Position: {:?}", cost_for_pos);

    let cost_for_best_position = cost_for_pos
        .into_iter()
        .min_by_key(|(_, cost)| *cost)
        .map(|(_, cost)| cost)
        .unwrap();

    println!("{}", cost_for_best_position);

    Ok(())
}
