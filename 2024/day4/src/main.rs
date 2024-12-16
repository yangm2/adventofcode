use anyhow::Result;
use regex::Regex;
use std::{char, error::Error};

#[doc = include_str!("../README.md")]

fn txt2ary<const ROWS: usize, const COLS: usize>(
    input: &str,
) -> Result<[[Option<char>; COLS]; ROWS]> {
    let mut ary: [[Option<char>; COLS]; ROWS] = [[None; COLS]; ROWS];

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            ary[row][col] = Some(char);
        }
        // dbg!(row, ary[row]);
    }

    Ok(ary.clone())
}

fn rot_ary_90<const IN_ROWS: usize, const IN_COLS: usize>(
    ary: &[[Option<char>; IN_COLS]; IN_ROWS],
) -> Result<[[Option<char>; IN_ROWS]; IN_COLS]> {
    let mut out_ary: [[Option<char>; IN_ROWS]; IN_COLS] = [[None; IN_ROWS]; IN_COLS];

    for (rr, row) in ary.iter().enumerate() {
        for (cc, col) in row.iter().enumerate() {
            // out_ary[cc][rr] = *col;

            // 0,0 -> 9,0
            // 0,9 -> 0,0
            // 0,3 -> 6,0

            out_ary[IN_COLS - cc - 1][rr] = *col;
        }
    }

    Ok(out_ary)
}

fn seach_for_xmas<const ROWS: usize, const COLS: usize>(
    ary: &[[Option<char>; COLS]; ROWS],
) -> Result<usize> {
    let mut part1_count: usize = 0;

    // count forward and reverse
    let re = Regex::new(r"XMAS|SAMX").unwrap();
    for (line_num, row) in ary.iter().enumerate() {
        let tmp = row.map(|a| a.unwrap().to_string()).concat();
        let caps = re.captures_iter(&tmp).count();
        // dbg!(line_num + 1, &tmp, caps);
        part1_count += caps;
    }

    // also count overlapping
    let re2 = Regex::new(r"XMASAMX|SAMXMAS").unwrap();
    for (line_num, row) in ary.iter().enumerate() {
        let tmp = row.map(|a| a.unwrap().to_string()).concat();
        let caps = re2.captures_iter(&tmp).count();
        // dbg!(line_num + 1, &tmp, caps);
        part1_count += caps;
    }

    Ok(part1_count)
}

fn main() -> Result<(), Box<dyn Error>> {
    // compile input file into binary
    const INPUT_TXT: &str = include_str!("../input.txt");

    const ARY_ROWS_COLS: usize = 140;
    let ary: [[Option<char>; ARY_ROWS_COLS]; ARY_ROWS_COLS] =
        txt2ary::<ARY_ROWS_COLS, ARY_ROWS_COLS>(INPUT_TXT).unwrap();

    let mut part1_count: usize = 0;

    // count forward and backwards on the same line

    part1_count += seach_for_xmas(&ary).unwrap();

    // count forward and backwards on vertical

    let xposed = rot_ary_90(&ary).unwrap();
    part1_count += seach_for_xmas(&xposed).unwrap();

    // count forward and backwards on diagonals
    for (rr, row) in ary.iter().enumerate() {
        for (cc, _col) in row.iter().enumerate() {
            // forward
            if rr + 3 < ARY_ROWS_COLS && cc + 3 < ARY_ROWS_COLS {
                let mut tmp: [Option<char>; 4] = [None; 4];
                for ii in 0..4 {
                    tmp[ii] = Some(ary[rr + ii][cc + ii].unwrap());
                }

                // dbg!(rr, cc, tmp);
                part1_count += seach_for_xmas(&[tmp; 1]).unwrap();
            }

            // backward
            if rr + 3 < ARY_ROWS_COLS && cc >= 3 {
                let mut tmp: [Option<char>; 4] = [None; 4];
                for ii in 0..4 {
                    tmp[ii] = Some(ary[rr + ii][cc - ii].unwrap());
                }
                // dbg!(rr, cc, tmp);
                part1_count += seach_for_xmas(&[tmp; 1]).unwrap();
            }
        }
    }

    // soln <2750
    println!("part 1 count = {part1_count}");

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_1() {
        const FINAL_ANSWER: usize = 18;

        const ARY_ROWS_COLS: usize = 10;
        let tmp: String = String::from(
            "MMMSXXMASM\n\
             MSAMXMSMSA\n\
             AMXSXMAAMM\n\
             MSAMASMSMX\n\
             XMASAMXAMM\n\
             XXAMMXXAMA\n\
             SMSMSASXSS\n\
             SAXAMASAAA\n\
             MAMMMXMMMM\n\
             MXMXAXMASX",
        );
        let input_txt = tmp.as_str();
        let ary: [[Option<char>; ARY_ROWS_COLS]; ARY_ROWS_COLS] =
            txt2ary::<ARY_ROWS_COLS, ARY_ROWS_COLS>(&input_txt).unwrap();
        assert_eq!(ary[0][0], Some('M'));
        assert_eq!(ary[ARY_ROWS_COLS - 1][ARY_ROWS_COLS - 1], Some('X'));

        let mut count = 0;

        count += seach_for_xmas(&ary).unwrap();
        assert!(count < FINAL_ANSWER);

        let rot90 = rot_ary_90(&ary).unwrap();
        assert_eq!(rot90[0][0], Some('M'));
        assert_eq!(rot90[ARY_ROWS_COLS - 1][ARY_ROWS_COLS - 1], Some('M'));
        count += seach_for_xmas(&rot90).unwrap();
        assert!(count < FINAL_ANSWER);

        // count forward and backwards on diagonals
        for (rr, row) in ary.iter().enumerate() {
            for (cc, _col) in row.iter().enumerate() {
                // forward
                if rr + 3 < ARY_ROWS_COLS && cc + 3 < ARY_ROWS_COLS {
                    let mut tmp: [Option<char>; 4] = [None; 4];
                    for ii in 0..4 {
                        tmp[ii] = Some(ary[rr + ii][cc + ii].unwrap());
                    }

                    // dbg!(rr, cc, tmp);
                    count += seach_for_xmas(&[tmp; 1]).unwrap();
                }

                // backward
                if rr + 3 < ARY_ROWS_COLS && cc >= 3 {
                    let mut tmp: [Option<char>; 4] = [None; 4];
                    for ii in 0..4 {
                        tmp[ii] = Some(ary[rr + ii][cc - ii].unwrap());
                    }
                    // dbg!(rr, cc, tmp);
                    count += seach_for_xmas(&[tmp; 1]).unwrap();
                }
            }
        }

        assert_eq!(count, FINAL_ANSWER);
    }
}
