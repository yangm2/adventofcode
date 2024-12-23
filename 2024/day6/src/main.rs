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

    fn iter_visited<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        VisitedMapIter::<'a, ROWS, COLS> {
            map: self,
            next_state: None,
        }
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

/// State for iterating over visited coords in Map
#[derive(Clone)]
struct VisitedMapIter<'a, const ROWS: usize, const COLS: usize> {
    map: &'a Map<ROWS, COLS>,
    /// bookkeeping for iterator
    next_state: Option<(usize, usize)>,
}

impl<'a, const ROWS: usize, const COLS: usize> Iterator for VisitedMapIter<'a, ROWS, COLS> {
    type Item = (usize, usize);

    /// return next visited coord
    fn next(&mut self) -> Option<Self::Item> {
        // helper function
        fn inc<const ROWS: usize, const COLS: usize>(r: usize, c: usize) -> Option<(usize, usize)> {
            if c < COLS - 1 {
                Some((r, c + 1))
            } else if r < ROWS - 1 {
                Some((r + 1, 0))
            } else {
                None
            }
        }

        let (start_r, mut start_c) = if self.next_state.is_none() {
            (0, 0)
        } else {
            self.next_state.unwrap()
        };

        // search for next visited
        for row in start_r..ROWS {
            for col in start_c..COLS {
                if self.map.grid[row][col].visited > 0 {
                    self.next_state = inc::<ROWS, COLS>(row, col);
                    // dbg!(Some((row, col)));
                    return Some((row, col));
                }
                self.next_state = inc::<ROWS, COLS>(row, col);
                // dbg!(row, col);
            }
            start_c = 0;
        }

        // didn't find any visited positions
        None
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

    #[test]
    fn test_2() {
        const PARTIAL_ANSWER: usize = 41;

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

        // update m with patrol coverage
        for _nstep in &mut m {
            // dbg!(nstep);
        }

        let distinct_positions_visited = m.count_positions_visited();

        assert!(ARY_ROWS_COLS * ARY_ROWS_COLS > distinct_positions_visited);
        assert_eq!(PARTIAL_ANSWER, distinct_positions_visited);

        // test iterating over visited coords
        let cm = m.clone();

        let mut iter = cm.iter_visited();
        assert_eq!((1, 4), iter.next().unwrap());
        assert_eq!((1, 5), iter.next().unwrap());
        assert_eq!((1, 6), iter.next().unwrap());
        assert_eq!((1, 7), iter.next().unwrap());
        assert_eq!((1, 8), iter.next().unwrap());
        assert_eq!((2, 4), iter.next().unwrap());

        let distinct_positions_visited = cm.count_positions_visited();
        assert_eq!(distinct_positions_visited, cm.iter_visited().count());
    }

    #[test]
    fn test_3() {
        const PARTIAL_ANSWER: usize = 41;
        const FINAL_ANSWER: usize = 1;

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

        let map_start = m.clone();

        // update m with patrol coverage
        for _nstep in &mut m {
            // dbg!(nstep);
        }

        let distinct_positions_visited = m.count_positions_visited();

        assert!(ARY_ROWS_COLS * ARY_ROWS_COLS > distinct_positions_visited);
        assert_eq!(PARTIAL_ANSWER, distinct_positions_visited);

        let mut count_infinite_loops = 0;
        let coord = (6, 3);

        dbg!(coord);

        // on tmp Map:
        //   change coord to be an obstruction
        //   check if guard loops

        let mut sim = map_start.clone();
        sim.grid[coord.0][coord.1].obstruction = true;

        dbg! {&sim.guard};

        for guard_step in &mut sim {
            println!("{:?}", &guard_step);

            if map_start.guard == guard_step {
                count_infinite_loops += 1;
                println!(
                    " -- FOUND infinite loop with obstruction at [{}][{}]",
                    coord.0, coord.1
                );
                break;
            }
        }

        dbg!(sim.grid[coord.0][coord.1]);
        println!(
            " -- NO infinite loop with obstruction at [{}][{}]",
            coord.0, coord.1
        );

        assert_eq!(FINAL_ANSWER, count_infinite_loops);
    }

    #[test]
    fn test_4() {
        const PARTIAL_ANSWER: usize = 41;
        const FINAL_ANSWER: usize = 6;

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

        let map_unvisited = Map::<ARY_ROWS_COLS, ARY_ROWS_COLS>::from_str(input_txt);

        let mut map_visited = map_unvisited.clone();

        // update m with patrol coverage
        for _nstep in &mut map_visited {
            // dbg!(nstep);
        }

        let distinct_positions_visited = map_visited.count_positions_visited();

        assert!(ARY_ROWS_COLS * ARY_ROWS_COLS > distinct_positions_visited);
        assert_eq!(PARTIAL_ANSWER, distinct_positions_visited);

        let mut count_infinite_loops = 0;
        for coord in map_visited
            .clone()
            .iter_visited()
            .filter(|uu| (uu.0, uu.1) != (map_unvisited.guard.row, map_unvisited.guard.col))
        {
            dbg!(coord);

            // on tmp Map:
            //   change coord to be an obstruction
            //   check if guard loops

            let mut sim = map_unvisited.clone();
            sim.grid[coord.0][coord.1].obstruction = true;

            // record start position (optional) and turns
            // NOTE: some infinite loops are a subset of the whole path; so they may not return to start position
            // FIXME: this should be stored in the iterator struct and return something like Err<InfiniteLoop>
            let mut previous_steps = Vec::<Guard>::new();
            // previous_steps.push(sim.guard.clone()); // optimization ... some infnite loops return to start position
            let mut prev_orientation: Orientation = sim.guard.orientation.clone();

            for guard_step in &mut sim {
                // retracing turn-step indicates that the guard is in a loop
                if previous_steps.contains(&guard_step) {
                    count_infinite_loops += 1;
                    println!(
                        " -- FOUND infinite loop with obstruction at [{}][{}]",
                        coord.0, coord.1
                    );
                    break;
                } else {
                    // optimization ... only record turns intead of every step
                    if prev_orientation != guard_step.orientation {
                        prev_orientation = guard_step.orientation.clone();
                        previous_steps.push(guard_step);
                    }
                }
            }

            println!(
                " -- NO infinite loop with obstruction at [{}][{}]",
                coord.0, coord.1
            )
        }

        assert_eq!(FINAL_ANSWER, count_infinite_loops);
    }
}

#[doc = include_str!("../README.md")]
fn main() -> Result<(), Box<dyn Error>> {
    // compile input file into binary
    const INPUT_TXT: &str = include_str!("../input.txt");
    const ARY_ROWS_COLS: usize = 130;

    let map_unvisited = Map::<ARY_ROWS_COLS, ARY_ROWS_COLS>::from_str(INPUT_TXT);

    let mut map_visited = map_unvisited.clone();
    // walk the guard through the map
    for _step in &mut map_visited {
        // dbg!(step);
    }

    let part1_distinct_positions_visited = map_visited.count_positions_visited();
    println!("part 1 distinct_positions_visited = {part1_distinct_positions_visited}");

    let mut part2_count_infinite_loops = Vec::<(usize, usize)>::new();
    // NOTE: cannot put obstruction at the Guard's starting point!!!
    for coord in map_visited
        .clone()
        .iter_visited()
        .filter(|uu| (uu.0, uu.1) != (map_unvisited.guard.row, map_unvisited.guard.col))
    {
        // dbg!(coord);

        // on tmp Map:
        //   change coord to be an obstruction
        //   check if guard loops

        let mut sim = map_unvisited.clone();
        sim.grid[coord.0][coord.1].obstruction = true;

        // record start position (optional) and turns
        // NOTE: some infinite loops are a subset of the whole path; so they may not return to start position
        // FIXME: this should be stored in the iterator struct and return something like Err<InfiniteLoop>
        let mut previous_steps = Vec::<Guard>::new();
        previous_steps.push(sim.guard.clone()); // optimization ... some infnite loops return to start position

        for guard_step in &mut sim {
            // retracing turn-step indicates that the guard is in a loop
            if previous_steps.contains(&guard_step) {
                part2_count_infinite_loops.push(coord);
                // println!(
                //     " -- FOUND infinite loop with obstruction at [{}][{}]",
                //     coord.0, coord.1
                // );
                break;
            } else {
                // optimization ... only record turns intead of every step
                if previous_steps.last().unwrap().orientation != guard_step.orientation {
                    // prev_orientation = guard_step.orientation.clone();
                    previous_steps.push(guard_step);
                }
            }
        }
    }

    // < 1656
    // > 931
    println!(
        "part 2 count infinite loops from 1 new obstruction: {}",
        part2_count_infinite_loops.iter().count()
    );

    Ok(())
}
