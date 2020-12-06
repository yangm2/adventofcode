extern crate clap;
use clap::{App, Arg};

use std::{fs::File, io::prelude::*, io::BufReader, path::Path};

fn main() {
    println!("Hello, world!");

    let matches = App::new("AoC Day 1")
        .version("1.0")
        .author("Mike Yang <yangm2@users.noreply.github.com>")
        .about("product of sums")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .required(true)
                .help("input data")
                .takes_value(true),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let input_file_name: &Path = Path::new(matches.value_of("input").unwrap());
    println!("Value for input: {}", input_file_name.display());

    // validate argument
    // FIXME: replace with proper error checking on File::open
    assert!(input_file_name.exists());
    assert!(input_file_name.is_file());

    let f = File::open(input_file_name);
    let b = BufReader::new(f.unwrap()).lines();

    // ingest/parse file (one integer per line)
    // NOTE: does not scale well if file is huge
    let mut v: Vec<i32> = Vec::new();
    for line in b {
        if let Ok(l) = line {
            let num: i32 = l.parse::<i32>().unwrap();
            v.push(num);
            // println!("{}", num);
        }
    }

    v.sort();

    // brute-force checks of combinations
    for (idx, _n) in v.iter().enumerate() {
        // println!("outer::{}", n);

        for n in idx..v.len() {
            // part 1: pairs that sum to 2020
            if v[idx] + v[n] == 2020 {
                println!("part 1 pair::{}, {}", v[idx], v[n]);
                println!("part 1 product::{}", v[idx] * v[n]);
            } else {
                if v[idx] + v[n] > 2020 {
                    continue;
                }
            }

            // part 2: triples that sum to 2020
            for p in n..v.len() {
                if v[idx] + v[n] + v[p] == 2020 {
                    println!("part 2 pair::{}, {}, {}", v[idx], v[n], v[p]);
                    println!("part 2 product::{}", v[idx] * v[n] * v[p]);
                } else {
                    if v[idx] + v[n] + v[p] > 2020 {
                        continue;
                    }
                }
            }
        }
    }
}
