use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    // compile input file into binary
    const INPUT_TXT: &str = include_str!("../input.txt");

    // declare 2D array to hold data
    const COLS: usize = 2;
    const ROWS: usize = 1000;  // FIXME: INPUT_TXT.lines().len() is not `const`
    let mut list: [ [ Option<i32>; ROWS]; COLS] = [[None; ROWS]; COLS];

    // parse data and store in array
    for (row, l)  in INPUT_TXT.lines().enumerate() {
        for (col, v) in l.split_ascii_whitespace().enumerate() {
            list[col][row] = Some(v.parse().unwrap());
        }
    }

    // sort each list (as per instructions)
    list[0].sort();
    list[1].sort();

    // compute the sum of the absolute difference between each pair
    let mut total_distance: i32 = 0;
    for (row, item) in list[0].iter().enumerate() {
        let left = item.unwrap();
        let right = list[1][row].unwrap();
        let distance = (right - left).abs();

        total_distance += distance;
    }

    // print result
    print!("total distance: {total_distance}\n");

    Ok(())

}
