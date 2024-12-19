#[doc = include_str!("../README.md")]

use anyhow::Result;
use std::error::Error;

/// direction that the guard is facing
#[derive(Clone, Debug, PartialEq)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Iterator for Orientation {
    type Item = Orientation;

    /// enums don't have a built-in iterator (for algebraic type theory reasons)
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Orientation::Up => Some(Self::Item::Right),
            Orientation::Right => Some(Self::Item::Down),
            Orientation::Down => Some(Self::Item::Left),
            Orientation::Left => Some(Self::Item::Up),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Guard {
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

/// Metadata and State of a position (within the grid)
#[derive(Clone, Copy, Debug)]
struct PositionMetadataState {
    obstruction: bool,
    visited: u32,
}
impl PositionMetadataState {
    fn new(obs: bool) -> Self {
        Self {
            obstruction: obs,
            visited: 0,
        }
    }
}

/// Not to be confused with the generic std::Map datastructure, naming is hard 😭
#[derive(Clone, Debug)]
pub struct Map<const ROWS: usize, const COLS: usize> {
    /// coordinate grid, with metadata and state about each position
    grid: [[PositionMetadataState; COLS]; ROWS],
    /// state to track the guard
    guard: Guard,
}
impl<const ROWS: usize, const COLS: usize> Map<ROWS, COLS> {
    /// general constructor
    pub fn from_str(input_txt: &str) -> Self {
        let mut g: Option<Guard> = None;
        let mut result: [[Option<PositionMetadataState>; COLS]; ROWS] = [[None; COLS]; ROWS];

        for (rr, row) in input_txt.lines().enumerate() {
            for (cc, col) in row.chars().enumerate() {
                let obs = col == '#';
                if col == '^' {
                    g = Some(Guard::new(rr, cc, Orientation::Up));
                }
                result[rr][cc] = Some(PositionMetadataState::new(obs));
            }
        }

        Map {
            grid: result.map(|r| r.map(|p| p.unwrap())),
            guard: g.unwrap(),
        }
    }

    /// return (row, col) in the Map of whatever is "in front" of the guard
    fn coord_in_front_of_guard(&self) -> Option<(usize, usize)> {
        match self.guard.orientation {
            Orientation::Up => {
                if self.guard.row > 0 {
                    let new_row = self.guard.row - 1;
                    let new_col = self.guard.col;
                    Some((new_row, new_col))
                } else {
                    None
                }
            }
            Orientation::Right => {
                if self.guard.col < COLS - 1 {
                    let new_row = self.guard.row;
                    let new_col = self.guard.col + 1;
                    Some((new_row, new_col))
                } else {
                    None
                }
            }
            Orientation::Down => {
                if self.guard.row < ROWS - 1 {
                    let new_row = self.guard.row + 1;
                    let new_col = self.guard.col;
                    Some((new_row, new_col))
                } else {
                    None
                }
            }
            Orientation::Left => {
                if self.guard.col > 0 {
                    let new_row = self.guard.row;
                    let new_col = self.guard.col - 1;
                    Some((new_row, new_col))
                } else {
                    None
                }
            }
        }
    }

    /// didn't really need a count of times visited, so this just reduces the count
    pub fn count_positions_visited(&self) -> usize {
        self.grid
            .as_flattened()
            .iter()
            .filter(|p| p.visited > 0)
            .count()
    }
}

impl<const ROWS: usize, const COLS: usize> Iterator for &mut Map<ROWS, COLS> {
    type Item = Guard;

    /// next action for the guard: step, turn or done
    fn next(&mut self) -> Option<Self::Item> {
        match self.coord_in_front_of_guard() {
            Some((r, c)) => {
                if self.grid[r][c].obstruction {
                    self.guard.orientation = self.guard.orientation.next().unwrap();
                } else {
                    self.grid[r][c].visited += 1;
                    self.guard.row = r;
                    self.guard.col = c;
                }
                Some(self.guard.clone())
            }
            None => None,
        }
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
            //123456789
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

        assert_eq!(
            Guard::new(5, 4, Orientation::Up),
            m.into_iter().next().unwrap()
        );
        assert_eq!(
            Guard::new(4, 4, Orientation::Up),
            m.into_iter().next().unwrap()
        );

        let mut distinct_positions_visited = m.count_positions_visited();
        assert_eq!(2, distinct_positions_visited);

        // check the first several steps
        assert_eq!(
            Guard::new(3, 4, Orientation::Up),
            m.into_iter().next().unwrap()
        );
        assert_eq!(
            Guard::new(2, 4, Orientation::Up),
            m.into_iter().next().unwrap()
        );
        assert_eq!(
            Guard::new(1, 4, Orientation::Up),
            m.into_iter().next().unwrap()
        );

        // check the first turn
        assert_eq!(
            Guard::new(1, 4, Orientation::Right),
            m.into_iter().next().unwrap()
        );
        assert_eq!(
            Guard::new(1, 5, Orientation::Right),
            m.into_iter().next().unwrap()
        );
        assert_eq!(
            Guard::new(1, 6, Orientation::Right),
            m.into_iter().next().unwrap()
        );
        assert_eq!(
            Guard::new(1, 7, Orientation::Right),
            m.into_iter().next().unwrap()
        );

        distinct_positions_visited = m.count_positions_visited();
        assert_eq!(8, distinct_positions_visited);

        // keep going
        for nstep in &mut m {
            dbg!(nstep);
        }

        distinct_positions_visited = m.count_positions_visited();

        assert!(ARY_ROWS_COLS * ARY_ROWS_COLS > distinct_positions_visited);
        assert_eq!(FINAL_ANSWER, distinct_positions_visited);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // compile input file into binary
    const INPUT_TXT: &str = include_str!("../input.txt");
    const ARY_ROWS_COLS: usize = 130;

    let mut map = Map::<ARY_ROWS_COLS, ARY_ROWS_COLS>::from_str(INPUT_TXT);

    // walk the guard through the map
    for step in &mut map {
        dbg!(step);
    }

    let part1_distinct_positions_visited = map.count_positions_visited();
    println!("part 1 distinct_positions_visited = {part1_distinct_positions_visited}");

    Ok(())
}
