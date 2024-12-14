use anyhow::Result;
use regex::Regex;
use std::error::Error;

#[doc = include_str!("../README.md")]

fn part1(input: &str) -> Result<u32> {
    let mut part1_sum: u32 = 0;

    let part1_re = Regex::new(r"(?m)mul\((?P<m1>\d+),(?P<m2>\d+)\)").unwrap();

    for mat in part1_re.captures_iter(input) {
        // dbg!(&mat);
        let prod = mat["m1"].parse::<u32>().unwrap() * mat["m2"].parse::<u32>().unwrap();
        // dbg!(&prod);
        part1_sum += prod;
    }
    Ok(part1_sum)
}

fn part2(input: &str) -> Result<u32> {
    let mut part2_sum: u32 = 0;

    let part2_re =
        Regex::new(r"(?m)mul\((?P<m1>\d+),(?P<m2>\d+)\)|(?P<enable>do\(\))|(?P<disable>don't\(\))")
            .unwrap();

    let mut enabled: bool = true;
    for mat in part2_re.captures_iter(input) {
        // dbg!(&mat);

        // TODO: can a regex::Capture be destructured in a match-block?
        // update state based on which which capture group matched
        if mat.name("disable").is_some() {
            enabled = false;
        } else if mat.name("enable").is_some() {
            enabled = true;
        } else {
            if enabled {
                let prod = mat["m1"].parse::<u32>().unwrap() * mat["m2"].parse::<u32>().unwrap();
                // dbg!(&prod);
                part2_sum += prod;
            }
        }
    }
    Ok(part2_sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    // compile input file into binary
    const INPUT_TXT: &str = include_str!("../input.txt");

    let part1_sum: u32 = part1(INPUT_TXT).unwrap();
    println!("part1: sum of products = {part1_sum}");

    let part2_sum: u32 = part2(INPUT_TXT).unwrap();
    println!("part2: sum of products = {part2_sum}");

    Ok(())
}
