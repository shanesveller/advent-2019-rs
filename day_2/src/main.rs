use std::env;
use std::fs;
use std::io;
use std::path::Path;

use ::day_2::Opcode;

/// ```rust
/// let ns = vec![1,0,0,0,99];
/// let expected = vec![2,0,0,0,99];
/// assert_eq!(part_1(ns), expected);
/// ````
fn part_1(mut ns: Vec<usize>) -> usize {
    let mut offset = 0;

    while offset < ns.len() {
        let op = ns[offset];

        match Opcode::from(op) {
            Opcode::Add => {
                let val = {
                    let arg1_pos = ns[offset + 1];
                    let arg2_pos = ns[offset + 2];
                    let arg1 = ns[arg1_pos];
                    let arg2 = ns[arg2_pos];
                    arg1 + arg2
                };
                let res = ns[offset + 3];
                ns[res] = val;
            }
            Opcode::Multiply => {
                let val = {
                    let arg1_pos = ns[offset + 1];
                    let arg2_pos = ns[offset + 2];
                    let arg1 = ns[arg1_pos];
                    let arg2 = ns[arg2_pos];
                    arg1 * arg2
                };
                let res = ns[offset + 3];
                ns[res] = val;
            }
            Opcode::Halt => break,
            _ => (),
        }

        offset += 4;
    }

    ns[0]
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
