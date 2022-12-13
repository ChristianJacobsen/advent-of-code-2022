use std::{
    cmp::{max, min},
    error::Error,
    str::FromStr,
};

use utils::read_input_file;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn len(&self) -> u32 {
        self.end - self.start
    }

    fn overlapping_range(&self, r: &Self) -> Option<Self> {
        if self.start > r.end || self.end < r.start {
            return None;
        }

        Some(Self {
            start: max(self.start, r.start),
            end: min(self.end, r.end),
        })
    }

    fn overlaps(&self, r: &Self) -> bool {
        self.start <= r.end && self.end >= r.start
    }

    fn fully_overlaps(&self, r: &Self) -> bool {
        if let Some(overlap) = self.overlapping_range(r) {
            let overlap_length = overlap.len();
            return self.len() == overlap_length || r.len() == overlap_length;
        }

        false
    }
}

impl FromStr for Range {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or("could not extract start/end")?;
        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = read_input_file()?;

    let mut sum_part_1 = 0;
    for pair in file_content.lines() {
        let (elf_1, elf_2) = pair.split_once(',').ok_or("could not split pair")?;

        let elf_1 = elf_1.parse::<Range>()?;
        let elf_2 = elf_2.parse::<Range>()?;

        if elf_1.fully_overlaps(&elf_2) {
            sum_part_1 += 1;
        }
    }

    let mut sum_part_2 = 0;
    for pair in file_content.lines() {
        let (elf_1, elf_2) = pair.split_once(',').ok_or("could not split pair")?;

        let elf_1 = elf_1.parse::<Range>()?;
        let elf_2 = elf_2.parse::<Range>()?;

        if elf_1.overlaps(&elf_2) {
            sum_part_2 += 1;
        }
    }

    println!("Part 1: {}", sum_part_1);
    println!("Part 2: {}", sum_part_2);

    Ok(())
}
