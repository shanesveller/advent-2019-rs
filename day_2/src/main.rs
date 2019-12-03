use std::env;
use std::fs;
use std::io;
use std::path::Path;

use std::collections::BTreeMap;

use ::day_2::Opcode;

/// ```rust
/// let ns = vec![1,0,0,0,99];
/// let expected = vec![2,0,0,0,99];
/// assert_eq!(part_1(ns), expected);
/// ````
fn part_1(ns: Vec<usize>) -> usize {
    let mut p_v: BTreeMap<usize, usize> = BTreeMap::new();

    for (i, n) in ns.iter().enumerate() {
        p_v.insert(i, *n);
    }

    let mut offset = 0;

    while offset < ns.len() {
        let op = p_v.get(&offset).unwrap();

        match Opcode::from(*op) {
            Opcode::Add => {
                let val = {
                    let arg1_pos = p_v.get(&(offset + 1)).unwrap();
                    let arg2_pos = p_v.get(&(offset + 2)).unwrap();
                    let arg1 = p_v.get(&arg1_pos).unwrap();
                    let arg2 = p_v.get(&arg2_pos).unwrap();
                    arg1 + arg2
                };
                let res = *p_v.get(&(offset + 3)).unwrap();
                p_v.insert(res, val);
                offset += 4;
            }
            Opcode::Multiply => {
                let val = {
                    let arg1_pos = p_v.get(&(offset + 1)).unwrap();
                    let arg2_pos = p_v.get(&(offset + 2)).unwrap();
                    let arg1 = p_v.get(&arg1_pos).unwrap();
                    let arg2 = p_v.get(&arg2_pos).unwrap();
                    arg1 * arg2
                };
                let res = *p_v.get(&(offset + 3)).unwrap();
                p_v.insert(res, val);
                offset += 4;
            }
            Opcode::Halt => break,
            _ => (),
        }
    }

    p_v.values().copied().next().unwrap()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = Path::new(&args[1]);
    let input = fs::read_to_string(filename)?;

    let clean_nouns = day_2::parse_input(&input);

    {
        let noun = 12;
        let verb = 2;
        let mut nums: Vec<usize> = vec![clean_nouns[0], noun, verb];
        nums.extend_from_slice(&clean_nouns[3..]);

        nums[1] = noun;
        nums[2] = verb;

        dbg!(part_1(nums));
    }

    {
        let mut noun = 1;
        let mut verb = 1;
        let mut iterations = 0;
        let target = 19_690_720;

        'outer: for noun_candidate in 1..99 {
            for verb_candidate in 1..99 {
                iterations += 1;
                let mut nums: Vec<usize> = vec![clean_nouns[0], noun_candidate, verb_candidate];
                nums.extend_from_slice(&clean_nouns[3..]);
                nums[1] = noun_candidate;
                nums[2] = verb_candidate;

                let output = part_1(nums);

                if output == target {
                    noun = noun_candidate;
                    verb = verb_candidate;
                    break 'outer;
                }
            }
        }

        dbg!(100 * noun + verb, iterations);
    }

    Ok(())
}
