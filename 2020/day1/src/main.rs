extern crate clap;
use clap::{App, Arg, SubCommand};

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
    assert!(input_file_name.exists());
    assert!(input_file_name.is_file());
    let f = File::open(input_file_name);
    let b = BufReader::new(f.unwrap()).lines();

    let mut v: Vec<i32> = Vec::new();
    for line in b {
        if let Ok(l) = line {
            let num:i32 = l.parse::<i32>().unwrap();
            v.push(num);
            println!("{}", num);
        }
    }

    v.sort();

    for (idx, n) in v.iter().enumerate() {
        // println!("outer::{}", n);

        for n in idx..v.len() {
            if v[idx] + v[n] == 2020 {
                println!("inner2::{}, {}", v[idx], v[n]);
                println!("product2::{}", v[idx] * v[n]);
            }

            for p in n..v.len() {
                if v[idx] + v[n] + v[p] == 2020 {
                    println!("inner3::{}, {}, {}", v[idx], v[n], v[p]);
                    println!("product3::{}", v[idx] * v[n] * v[p]);
                }
            }
        }


    }

    // Ok(())
}
