use std::convert::TryFrom;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

use std::collections::HashSet;

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

type WirePath = (Dir, isize);

type Coord = (isize, isize);

fn manhatten_distance(l: Coord, r: Coord) -> usize {
    usize::try_from((l.0 - r.0).abs() + (l.1 - r.1).abs()).unwrap()
}

fn parse_path(s: &str) -> WirePath {
    let (raw_dir, raw_distance) = s.split_at(1);

    let dir = match raw_dir {
        "U" => Dir::Up,
        "D" => Dir::Down,
        "L" => Dir::Left,
        "R" => Dir::Right,
        _ => Dir::Unknown,
    };
    (dir, raw_distance.parse().unwrap())
}

fn track_placements(paths: &[WirePath]) -> HashSet<Coord> {
    let mut coords = HashSet::new();

    let mut current_pos: Coord = ORIGIN;

    for (dir, distance) in paths {
        coords.insert(current_pos);
        let range = 1..=*distance;

        let new_coords: Vec<_> = match dir {
            Dir::Up => range.map(|n| (current_pos.0, current_pos.1 + n)).collect(),
            Dir::Right => range.map(|n| (current_pos.0 + n, current_pos.1)).collect(),
            Dir::Down => range.map(|n| (current_pos.0, current_pos.1 - n)).collect(),
            Dir::Left => range.map(|n| (current_pos.0 - n, current_pos.1)).collect(),
            _ => vec![],
        };

        for c in new_coords {
            current_pos = c;
            coords.insert(c);
        }
    }
    coords
}

const ORIGIN: Coord = (0, 0);

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let filename = Path::new(&args[1]);
    let input = fs::read_to_string(filename)?;

    let wire_paths: Vec<HashSet<_>> = input
        .trim()
        .lines() // wires
        .map(|l| l.split(',').map(parse_path).collect()) // per-wire path list
        .map(|ps: Vec<WirePath>| track_placements(&ps[..]))
        .collect();

    let collisions: HashSet<_> = wire_paths[0].intersection(&wire_paths[1]).collect();

    let closest = collisions
        .iter()
        .filter(|c| ***c != ORIGIN)
        .min_by(|l, r| manhatten_distance(***l, ORIGIN).cmp(&manhatten_distance(***r, ORIGIN)))
        .unwrap();

    dbg!(**closest);
    dbg!(manhatten_distance(ORIGIN, **closest));

    Ok(())
}
