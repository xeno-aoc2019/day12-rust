extern crate regex;

use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, Result, Lines};
use regex::Regex;



pub fn get_moons(file_name: &str) -> Vec<(i32, i32, i32)> {
    let re = Regex::new(r"<x=([^\d]?\d+), y=([^\d]?\d+), z=([^\d]?\d+)>").unwrap();
    let mut result = vec!();
    if let Ok(lines) = get_lines(file_name) {
        for maybeLine in lines {
            if let Ok(line) = maybeLine {
                let x = line.clone();
                let matches = re.captures(&line);
                if let Some(captures) = matches {
                    let x: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
                    let y: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
                    let z: i32 = captures.get(3).unwrap().as_str().parse().unwrap();
                    println!("X={} Y={} Z={}", x, y, z);
                    result.push((x, y, z));
                }
            }
        }
    }
    result
}


fn get_lines<P>(file_name: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file).lines())
}