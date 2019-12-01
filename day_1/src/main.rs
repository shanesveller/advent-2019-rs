use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

use ::day_1::{Fuel, Mass};

fn part_1(masses: &[Mass]) -> Fuel {
    masses
        .iter()
        .map(|m| m.required_fuel())
        .fold(Fuel(0), |acc, f| acc + f)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = Path::new(&args[1]);

    let f = File::open(filename)?;
    let bf = BufReader::new(f);

    let masses: Vec<Mass> = bf
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .map(Mass)
        .collect();

    let part_1 = part_1(&masses);

    dbg!(part_1);

    Ok(())
}
