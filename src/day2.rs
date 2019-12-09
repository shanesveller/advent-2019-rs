use core::convert::From;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opcode {
    Add,
    Multiply,
    Halt,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseOpcodeError;

impl From<usize> for Opcode {
    /// # Examples
    /// ```rust
    /// use advent_of_code_2019::day2::Opcode;
    /// assert_eq!(Opcode::from(1), Opcode::Add);
    /// assert_eq!(Opcode::from(2), Opcode::Multiply);
    /// assert_eq!(Opcode::from(3), Opcode::Unknown);
    /// assert_eq!(Opcode::from(99), Opcode::Halt);
    /// ````
    fn from(n: usize) -> Self {
        match n {
            1 => Self::Add,
            2 => Self::Multiply,
            99 => Self::Halt,
            _ => Self::Unknown,
        }
    }
}

/// ```rust
/// use advent_of_code_2019::day2;
/// let str = "1,2,3\n4,5";
/// assert_eq!(day2::input_generator(str), [1,2,3,4,5]);
/// ```
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|l| l.split(','))
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve_with_noun_and_verb(input: &[usize], noun: usize, verb: usize) -> usize {
    let mut offset = 0;
    let mut registers: Vec<usize> = input.to_vec();

    registers[1] = noun;
    registers[2] = verb;

    while offset < input.len() {
        let op = registers[offset];

        match Opcode::from(op) {
            Opcode::Add => {
                let val = {
                    let arg1_pos = registers[offset + 1];
                    let arg2_pos = registers[offset + 2];
                    let arg1 = registers[arg1_pos];
                    let arg2 = registers[arg2_pos];
                    arg1 + arg2
                };
                let res = registers[offset + 3];
                registers[res] = val;
            }
            Opcode::Multiply => {
                let val = {
                    let arg1_pos = registers[offset + 1];
                    let arg2_pos = registers[offset + 2];
                    let arg1 = registers[arg1_pos];
                    let arg2 = registers[arg2_pos];
                    arg1 * arg2
                };
                let res = registers[offset + 3];
                registers[res] = val;
            }
            Opcode::Halt => break,
            _ => (),
        }

        offset += 4;
    }

    registers[0]
}

#[aoc(day2, part1)]
fn solve_part1(input: &[usize]) -> usize {
    const NOUN: usize = 12;
    const VERB: usize = 2;

    solve_with_noun_and_verb(input, NOUN, VERB)
}

#[aoc(day2, part2)]
fn solve_part2(input: &[usize]) -> usize {
    let mut noun = 1;
    let mut verb = 1;
    let target = 19_690_720;

    'outer: for noun_candidate in 0..100 {
        for verb_candidate in 0..100 {
            let mut nums: Vec<usize> = input.to_vec();
            nums[1] = noun_candidate;
            nums[2] = verb_candidate;

            let output = solve_with_noun_and_verb(input, noun_candidate, verb_candidate);

            if output == target {
                noun = noun_candidate;
                verb = verb_candidate;
                break 'outer;
            }
        }
    }

    100 * noun + verb
}
