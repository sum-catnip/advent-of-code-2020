#![feature(iter_map_while)]

use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::fmt::Debug;
use std::io::prelude::*;

pub fn file_as_nums<T>(file: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err : Debug {
    BufReader::new(File::open(file).unwrap())
        .lines()
        .map_while(|x| x.ok().map(|y| y.parse().unwrap()))
        .collect::<Vec<T>>()
}
