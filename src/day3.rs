use std::convert::TryFrom;
use std::fmt;

use std::collections::HashMap;

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

type WirePath = (Dir, isize);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coord(pub isize, pub isize);

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coord: ({}, {})", self.0, self.1)
    }
}

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

fn track_placements(paths: &[WirePath]) -> HashMap<Coord, usize> {
    let mut coords: HashMap<Coord, usize> = HashMap::new();
    let mut current_pos: Coord = ORIGIN;
    let mut step = 0;

    for (dir, distance) in paths {
        let range = 1..=*distance;

        let new_coords: Vec<_> = match dir {
            Dir::Up => range
                .map(|n| Coord(current_pos.0, current_pos.1 + n))
                .collect(),
            Dir::Right => range
                .map(|n| Coord(current_pos.0 + n, current_pos.1))
                .collect(),
            Dir::Down => range
                .map(|n| Coord(current_pos.0, current_pos.1 - n))
                .collect(),
            Dir::Left => range
                .map(|n| Coord(current_pos.0 - n, current_pos.1))
                .collect(),
            _ => vec![],
        };

        for c in new_coords {
            current_pos = c;
            step += 1;
            coords.entry(c).or_insert(step);
        }
    }
    coords
}

const ORIGIN: Coord = Coord(0, 0);

fn collisions_on_paths(paths: &[HashMap<Coord, usize>]) -> HashMap<Coord, usize> {
    let initial = paths[0].clone();
    paths[1..].iter().fold(initial, |acc, p| {
        acc.into_iter()
            .filter_map(|(c, l)| {
                if let Some(n) = p.get(&c) {
                    Some((c, n + l))
                } else {
                    None
                }
            })
            .collect()
    })
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<HashMap<Coord, usize>> {
    input
        .trim()
        .lines() // wires
        .map(|l| l.split(',').map(parse_path).collect()) // per-wire path list
        .map(|ps: Vec<WirePath>| track_placements(&ps[..]))
        .collect()
}

#[aoc(day3, part1)]
fn solve_part1(input: &[HashMap<Coord, usize>]) -> usize {
    let collisions = collisions_on_paths(input);
    let closest = collisions
        .iter()
        .filter(|(c, _s)| **c != ORIGIN)
        .min_by(|(c, _s), (c2, _s2)| {
            manhatten_distance(**c, ORIGIN).cmp(&manhatten_distance(**c2, ORIGIN))
        })
        .unwrap()
        .0;

    manhatten_distance(ORIGIN, *closest)
}

#[aoc(day3, part2)]
fn solve_part2(input: &[HashMap<Coord, usize>]) -> usize {
    let collisions = collisions_on_paths(input);
    *collisions
        .iter()
        .min_by(|(_c, s), (_c2, s2)| s.cmp(s2))
        .unwrap()
        .1
}
