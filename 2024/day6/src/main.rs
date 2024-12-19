use anyhow::Result;
use std::error::Error;

#[doc = include_str!("../README.md")]

fn main() -> Result<(), Box<dyn Error>> {
    // compile input file into binary
    const INPUT_TXT: &str = include_str!("../input.txt");
    const ARY_ROWS_COLS: usize = 130;

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug, PartialEq)]
struct Guard {
    orientation: Orientation,
    row: usize,
    col: usize,
}
impl Guard {
    fn new(rr: usize, cc: usize, orientation: Orientation) -> Self {
        Self {
            orientation: orientation,
            row: rr,
            col: cc,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Position {
    obstruction: bool,
    visited: u32,
}
impl Position {
    fn new(rr: usize, cc: usize, obs: bool) -> Self {
        Self {
            obstruction: obs,
            visited: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Map<const ROWS: usize, const COLS: usize> {
    grid: [[Position; COLS]; ROWS],
    guard: Guard,
}
impl<const ROWS: usize, const COLS: usize> Map<ROWS, COLS> {
    fn from_str(input_txt: &str) -> Self {
        let mut g: Option<Guard> = None;
        let mut result: [[Option<Position>; COLS]; ROWS] = [[None; COLS]; ROWS];

        for (rr, row) in input_txt.lines().enumerate() {
            for (cc, col) in row.chars().enumerate() {
                let obs = col == '#';
                if col == '^' {
                    g = Some(Guard::new(rr, cc, Orientation::Up));
                }
                result[rr][cc] = Some(Position::new(rr, cc, obs));
            }
        }

        Map {
            grid: result.map(|r| r.map(|p| p.unwrap())),
            guard: g.unwrap(),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Iterator for &mut Map<ROWS, COLS> {
    type Item = Map<ROWS, COLS>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_1() {
        const FINAL_ANSWER: usize = 41;

        const ARY_ROWS_COLS: usize = 10;
        let tmp: String = String::from(
            "....#.....\n\
             .........#\n\
             ..........\n\
             ..#.......\n\
             .......#..\n\
             ..........\n\
             .#..^.....\n\
             ........#.\n\
             #.........\n\
             ......#...",
        );
        let input_txt = tmp.as_str();

        let mut m = Map::<ARY_ROWS_COLS, ARY_ROWS_COLS>::from_str(input_txt);

        let step1 = m.into_iter().next().unwrap();
        assert_eq!(Guard::new(5, 5, Orientation::Up), step1.guard);

        for nstep in &mut m {
            dbg!(nstep.guard);
        }

        let distinct_positions_visited = m
            .grid
            .as_flattened()
            .iter()
            .filter(|p| p.visited > 0)
            .count();

        assert!(ARY_ROWS_COLS * ARY_ROWS_COLS > distinct_positions_visited);
        assert_eq!(FINAL_ANSWER, distinct_positions_visited);
    }
}
