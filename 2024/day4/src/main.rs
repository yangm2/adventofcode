#![feature(generic_const_exprs)]
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
            // 0,0 -> 9,0
            // 0,9 -> 0,0
            // 0,3 -> 6,0

            out_ary[IN_COLS - cc - 1][rr] = *col;
        }
    }

    Ok(out_ary)
}

// struct Blocky<T, const ROWS: usize, const COLS: usize>(pub [[Option<T>; COLS]; ROWS]);

// struct BlockyChar140(pub Blocky<char, 140, 140>);

// impl Iterator for BlockyChar140 {
//     type Item = [[char; 3]; 3];

//     fn next(&mut self) -> Option<Self::Item> {

//         let ary2d = &self.0.0;

//         let curr_row = ary2d.into_iter().next();

//         let curr_item = self.0.0.into_iter().next().unwrap().into_iter().next();

//         if curr_item.is_none() {
//             return None;
//         }
//         let temp_item = curr_item.clone();

//         let result = [['A'; 3];3];
//         Some(result)
//     }
// }

fn foreach3x3<const IN_ROWS: usize, const IN_COLS: usize>(
    ary: &[[Option<char>; IN_COLS]; IN_ROWS],
) -> Result<[[[[char; 3]; 3]; IN_COLS - 2]; IN_ROWS - 2]> {
    let mut result: [[[[char; 3]; 3]; IN_COLS - 2]; IN_ROWS - 2] =
        [[[['.'; 3]; 3]; IN_COLS - 2]; IN_ROWS - 2];

    for row in 0..=IN_ROWS - 3 {
        for col in 0..=IN_COLS - 3 {
            result[row][col] = [
                [
                    ary[row][col].unwrap(),
                    ary[row][col + 1].unwrap(),
                    ary[row][col + 2].unwrap(),
                ],
                [
                    ary[row + 1][col].unwrap(),
                    ary[row + 1][col + 1].unwrap(),
                    ary[row + 1][col + 2].unwrap(),
                ],
                [
                    ary[row + 2][col].unwrap(),
                    ary[row + 2][col + 1].unwrap(),
                    ary[row + 2][col + 2].unwrap(),
                ],
            ];
        }
    }

    Ok(result)
}

fn seach_for_xmas<const ROWS: usize, const COLS: usize>(
    ary: &[[Option<char>; COLS]; ROWS],
) -> Result<usize> {
    let mut part1_count: usize = 0;

    // count forward and reverse
    let re = Regex::new(r"XMAS|SAMX").unwrap();
    for (_line_num, row) in ary.iter().enumerate() {
        let tmp = row.map(|a| a.unwrap().to_string()).concat();
        let caps = re.captures_iter(&tmp).count();
        // dbg!(line_num + 1, &tmp, caps);
        part1_count += caps;
    }

    // also count overlapping
    // previous regex would already have counted these once, so increment by 1
    //   for the partial
    let re2 = Regex::new(r"XMASAMX|SAMXMAS").unwrap();
    for (_line_num, row) in ary.iter().enumerate() {
        let tmp = row.map(|a| a.unwrap().to_string()).concat();
        let caps = re2.captures_iter(&tmp).count();
        // dbg!(line_num + 1, &tmp, caps);
        part1_count += caps;
    }

    Ok(part1_count)
}

fn search_for_x_mas<const IN_ROWS: usize, const IN_COLS: usize>(
    ary: &[[[[char; 3]; 3]; IN_COLS - 2]; IN_ROWS - 2],
) -> Result<usize> {
    let mut count = 0;

    // . M .
    // M A S  => r".M.MAS.S."
    // . S .

    // M . M
    // . A .  => r"M.M.A.S.S"
    // S . S

    let re = Regex::new(
        r".M.MAS.S.|.S.MAS.M.|.S.SAM.M.|.M.SAM.S.|M.M.A.S.S|S.M.A.S.M|S.S.A.M.M|M.S.A.M.S",
    )
    .unwrap();

    for block in ary.as_flattened() {
        assert!(block[0][0] != '.');
        assert!(block[2][2] != '.');

        if block[1][1] == 'A' {
            let flattened = String::from_iter(block.as_flattened());
            let caps = re.captures_iter(&flattened).count();
            count += caps;
        }
    }

    Ok(count)
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

    let array_of_blocks = foreach3x3(&ary).unwrap();
    let part2_count = search_for_x_mas(&array_of_blocks).unwrap();

    // soln <2750
    println!("part 1 count = {part1_count}");

    // soln >1916, !1968
    println!("part 2 count = {part2_count}");

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

    #[test]
    fn test_2() {
        const FINAL_ANSWER: usize = 9;

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

        let tmp = foreach3x3(&ary).unwrap();
        let first_row_of_blocks = tmp.into_iter().next().unwrap();
        let first_elem_of_row: [[char; 3]; 3] = first_row_of_blocks.into_iter().next().unwrap();
        assert_eq!(
            String::from_iter(first_elem_of_row.as_flattened()),
            "MMMMSAAMX"
        );

        let x_mas_count = search_for_x_mas(&tmp).unwrap();
        assert_eq!(x_mas_count, FINAL_ANSWER);
    }
}
