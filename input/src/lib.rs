#![feature(iter_map_while)]

use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::fmt::Debug;
use std::iter::Iterator;
use std::io::prelude::*;

pub fn file_as_nums<T>(file: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err : Debug {
    file_lines(file)
        .map_while(|x| x.parse().ok())
        .collect::<Vec<T>>()
}

pub fn file_lines(file: &str) -> impl Iterator<Item=String> {
    BufReader::new(File::open(file).unwrap())
        .lines()
        .map_while(|l| l.ok())
}

pub fn file_split(file: &str, pat: &str) -> Vec<String> {
    let mut buf: String = String::new();
    BufReader::new(File::open(file).unwrap())
        .read_to_string(&mut buf)
        .unwrap();
    buf.split(pat).map(String::from).collect()
}
