use std::{fmt::Write, str::FromStr};

use anyhow::{anyhow, Context};

pub fn part1(input: &str) -> String {
    Emulator::from_str(input)
        .expect("Parse error")
        .run()
        .expect("Run error")
        .to_string()
}

pub fn part2(input: &str) -> u32 {
    todo!()
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<&u32> for Instruction {
    type Error = anyhow::Error;
    fn try_from(value: &u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err(anyhow!("Invalid instruction: {}", value)),
        }
    }
}

#[derive(Default, Debug)]
struct Emulator {
    register: Vec<u32>,
    program: Vec<u32>,
    program_counter: usize,
    program_output: String,
}

impl Emulator {
    fn run(&mut self) -> anyhow::Result<&String> {
        while let Some(instruction) = self
            .program
            .get(self.program_counter)
            .map(|x| x.try_into())
            .transpose()?
        {
            let operand = self
                .program
                .get(self.program_counter + 1)
                .with_context(|| "Operand not found")?;

            dbg!(&instruction, operand);

            match instruction {
                Instruction::Adv => self.register[0] /= 1 << self.combo_op(*operand)?,
                Instruction::Bxl => self.register[1] ^= operand,
                Instruction::Bst => self.register[1] = self.combo_op(*operand)? % 8,
                Instruction::Jnz => {
                    if self.register[0] != 0 {
                        self.program_counter = *operand as usize;
                        continue;
                    }
                }
                Instruction::Bxc => self.register[1] ^= self.register[2],
                Instruction::Out => {
                    self.write_output((self.combo_op(*operand)? % 8).to_string())?
                }
                Instruction::Bdv => {
                    self.register[1] = self.register[0] / (1 << self.combo_op(*operand)?)
                }
                Instruction::Cdv => {
                    self.register[2] = self.register[0] / (1 << self.combo_op(*operand)?)
                }
            }
            self.program_counter += 2;
        }
        Ok(&self.program_output)
    }

    fn write_output(&mut self, text: String) -> anyhow::Result<()> {
        if !self.program_output.is_empty() {
            self.program_output.write_char(',')?;
        }
        self.program_output.write_str(&text)?;
        Ok(())
    }

    fn combo_op(&self, operand: u32) -> anyhow::Result<u32> {
        match operand {
            0..=3 => Ok(operand),
            4..7 => self
                .register
                .get((operand - 4) as usize)
                .with_context(|| "Register should exist")
                .copied(),
            7 => Err(anyhow!("Reserved")),
            _ => Err(anyhow!("Invalid combo operand: {}", operand)),
        }
    }
}

impl FromStr for Emulator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r, p) = s.split_once("\n\n").with_context(|| "Splitting chunks")?;
        let register = r
            .lines()
            .map(|l| {
                l.trim_end()
                    .get(12..)
                    .ok_or(anyhow!("Invalid length"))
                    .and_then(|x| {
                        x.parse()
                            .with_context(|| format!("Invalid register value: {}", l))
                    })
            })
            .collect::<Result<Vec<u32>, _>>()?;

        let program = p
            .get(9..)
            .with_context(|| "Invalid program")?
            .chars()
            .step_by(2)
            .map(|c| {
                c.to_digit(10)
                    .with_context(|| format!("Invalid program contents: {}", c))
            })
            .collect::<Result<_, _>>()?;

        dbg!(&register, &program);

        Ok(Self {
            register,
            program,
            ..Default::default()
        })
    }
}

aoc2024::main!("../../inputs/day_17.txt");

aoc2024::test!(
    "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
",
    "4,6,3,5,6,3,5,2,1,0",
    117440
);

#[cfg(test)]
mod test {
    use crate::Emulator;

    #[test]
    fn example_1() {
        let mut emu = Emulator {
            register: vec![0, 0, 9],
            program: vec![2, 6],
            ..Default::default()
        };
        emu.run().unwrap();
        assert_eq!(emu.register[1], 1);
    }

    #[test]
    fn example_2() {
        let mut emu = Emulator {
            register: vec![10, 0, 0],
            program: vec![5, 0, 5, 1, 5, 4],
            ..Default::default()
        };
        assert!(emu.run().is_ok_and(|x| x == "0,1,2"));
    }

    #[test]
    fn example_3() {
        let mut emu = Emulator {
            register: vec![2024, 0, 0],
            program: vec![0, 1, 5, 4, 3, 0],
            ..Default::default()
        };
        assert_eq!(emu.run().unwrap(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(emu.register[0], 0);
    }

    #[test]
    fn example_4() {
        let mut emu = Emulator {
            register: vec![0, 29, 0],
            program: vec![1, 7],
            ..Default::default()
        };
        emu.run().unwrap();
        assert_eq!(emu.register[1], 26);
    }

    #[test]
    fn example_5() {
        let mut emu = Emulator {
            register: vec![0, 2024, 43690],
            program: vec![4, 0],
            ..Default::default()
        };
        emu.run().unwrap();
        assert_eq!(emu.register[1], 44354);
    }
}
