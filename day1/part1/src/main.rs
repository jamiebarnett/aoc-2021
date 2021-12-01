use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = match File::open("./input.txt") {
        Ok(f) => f,
        Err(err) => panic!("{}", err),
    };

    let mut input = Vec::new();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            let i: isize = line.parse().expect("could not parse input");
            input.push(i);
        }
    }

    let mut increase_count = 0;
    let mut previous: isize = 0;
    for (i, val) in input.iter().enumerate() {
        if i > 0 {
            previous = input[i - 1];
            if previous < val.to_owned() {
                increase_count += 1
            }
        }
    }

    println!("{}", increase_count)
}
