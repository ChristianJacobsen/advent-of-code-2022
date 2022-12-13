use std::{error::Error, fmt::Display, str::FromStr};

use utils::read_input_file;

enum Instruction {
    AddX(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((instruction, count)) = s.split_once(' ') {
            let count = count.parse::<i32>()?;
            match instruction {
                "addx" => Ok(Instruction::AddX(count)),
                _ => Err("invalid instruction".into()),
            }
        } else {
            match s {
                "noop" => Ok(Instruction::Noop),
                _ => Err("invalid instruction".into()),
            }
        }
    }
}

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

struct Crt {
    display: [char; CRT_WIDTH * CRT_HEIGHT],
    sprite_center: i32,
    cycle_counter: usize,
}

impl Crt {
    fn new() -> Self {
        Self {
            display: ['.'; CRT_WIDTH * CRT_HEIGHT],
            sprite_center: 1,
            cycle_counter: 0,
        }
    }

    fn paint_pixel(&mut self) {
        let sprint_range = self.sprite_center - 1..=self.sprite_center + 1;
        if sprint_range.contains(&(((self.cycle_counter - 1) % CRT_WIDTH) as i32)) {
            self.display[self.cycle_counter - 1] = '#';
        }
    }

    fn process(&mut self, instruction: &Instruction) {
        self.cycle_counter += 1;

        self.paint_pixel();

        match instruction {
            Instruction::AddX(count) => {
                self.cycle_counter += 1;

                self.paint_pixel();

                self.sprite_center += count;
            }
            Instruction::Noop => {}
        }
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..CRT_HEIGHT {
            for j in 0..CRT_WIDTH {
                write!(f, "{}", self.display[i * CRT_WIDTH + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut instructions = Vec::with_capacity(file_content.len());
    for line in file_content.lines() {
        instructions.push(line.parse::<Instruction>()?);
    }

    let mut cycle_counter = 0;
    let mut x_register = 1;
    let mut interesting_frequencies = Vec::new();
    for instruction in &instructions {
        cycle_counter += 1;

        if cycle_counter % 40 == 20 {
            interesting_frequencies.push(cycle_counter * x_register);
        }

        match instruction {
            Instruction::AddX(count) => {
                cycle_counter += 1;
                x_register += count;
            }
            Instruction::Noop => {}
        }
    }

    let part_1_sum = interesting_frequencies.iter().sum::<i32>();

    let mut crt = Crt::new();
    for instruction in &instructions {
        crt.process(instruction);
    }

    println!("Part 1: {}", part_1_sum);
    println!("Part 2:");
    println!("{}", crt);

    Ok(())
}
