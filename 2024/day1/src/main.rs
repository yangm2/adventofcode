use anyhow::Result;
use std::error::Error;

#[doc = include_str!("../README.md")]

fn part1(ary: [[Option<u32>; 1000]; 2]) -> Result<u32> {
    // compute the sum of the absolute difference between each pair
    let mut total_distance: u32 = 0;
    for (row, item) in ary[0].iter().enumerate() {
        let left = item.unwrap();
        let right = ary[1][row].unwrap();
        let distance = right.abs_diff(left);

        total_distance += distance;
    }

    Ok(total_distance)
}

fn part2(ary: [[Option<u32>; 1000]; 2]) -> Result<u32> {
    assert!(ary[1].is_sorted());

    // compute the sum of the absolute difference between each pair
    let mut similarity_sum: u32 = 0;
    for item in ary[0].iter() {
        let left_val = item.unwrap();

        // brute-force: count instances of left_val in ary[1]
        let mut count: usize = 0;
        for right_item in ary[1].iter() {
            // short-circuit since we have asserted that ary[1] is sorted
            if right_item.unwrap() > left_val {
                break;
            }

            if left_val == right_item.unwrap() {
                count += 1;
            }
        }

        let similarity = left_val * (count as u32);

        similarity_sum += similarity;
    }

    Ok(similarity_sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    // compile input file into binary
    const INPUT_TXT: &str = include_str!("../input.txt");

    // declare 2D array to hold data
    const COLS: usize = 2;
    const ROWS: usize = 1000; // FIXME: INPUT_TXT.lines().len() is not `const`
    let mut list: [[Option<u32>; ROWS]; COLS] = [[None; ROWS]; COLS];

    // parse data and store in array
    for (row, l) in INPUT_TXT.lines().enumerate() {
        for (col, v) in l.split_ascii_whitespace().enumerate() {
            list[col][row] = Some(v.parse().unwrap());
        }
    }

    // sort each list (as per instructions)
    list[0].sort();
    list[1].sort();

    // print results
    let part1_total_distance: u32 = part1(list.clone()).unwrap();
    print!("Part 1 - total distance: {part1_total_distance}\n");

    let part2_total_similarity: u32 = part2(list.clone()).unwrap();
    print!("Part 2 - total similarity: {part2_total_similarity}\n");

    Ok(())
}
