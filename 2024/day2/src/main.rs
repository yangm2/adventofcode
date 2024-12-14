use anyhow::Result;
use std::{cmp::Ordering, error::Error};

#[doc = include_str!("../README.md")]

fn part1(levels: Vec<u8>) -> Result<bool> {
    // check for sorted or reverse sorted
    if levels.is_sorted() || levels.is_sorted_by(|a, b| a.cmp(b) == Ordering::Greater) {
        // println!("{levels:?} is sorted");

        // use overlapping sliding window to check distance of adjacent elements
        for item in levels.windows(2) {
            let first = item.first().unwrap();
            let last = item.last().unwrap();

            let diff = first.abs_diff(*last);
            // print!("  {first}.abs_diff({last}) = {diff}\n");

            if diff >= 1 && diff <= 3 {
                // println!("    safe")
            } else {
                // println!("    UNSAFE");
                return Ok(false);
            }
        }
    } else {
        // println!("{levels:?} is NOT sorted");
        return Ok(false);
    }

    Ok(true)
}

fn part2(levels: Vec<u8>) -> Result<bool> {
    // brute-force: short-circuiting, recheck after removing each element
    for idx in 0..levels.len() {
        let mut lclone = levels.clone();
        let _discard = lclone.remove(idx);

        if part1(lclone).unwrap() {
            return Ok(true);
        }
    }

    Ok(false)
}

fn main() -> Result<(), Box<dyn Error>> {
    // compile input file into binary
    const INPUT_TXT: &str = include_str!("../input.txt");

    let mut part1_count: usize = 0;
    let mut part2_count: usize = 0;
    for line in INPUT_TXT.lines() {
        let levels: Vec<u8> = line
            .split_ascii_whitespace()
            .map(|s: &str| -> u8 { s.parse().unwrap() })
            .collect();

        if part1(levels.clone()).unwrap() {
            part1_count += 1;
        } else if part2(levels).unwrap() {
            // do more expensive recheck
            part2_count += 1;
        }
    }

    println!("part1_count = {part1_count}");

    part2_count += part1_count;
    println!("part2_count = {part2_count}");

    Ok(())
}
