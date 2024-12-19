use regex::Regex;
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::grid::{FixedGrid, Picture};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    instructions: Vec<Instruction>,
}
impl Solution {
    fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let instruction = Instruction::from(line);
            solution.add_instruction(instruction);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, is_full: bool) -> Self::Result {
        let (width, height) = if is_full { (50, 6) } else { (7, 3) };
        let mut old = ['.'; 50];

        let mut screen = FixedGrid::new(width, height);
        for y in 0..height {
            for x in 0..width {
                screen.set(x as isize, y as isize, '.');
            }
        }
        for instruction in &self.instructions {
            match instruction {
                Instruction::Rect(width, height) => {
                    for ry in 0..*height {
                        for rx in 0..*width {
                            screen.set(rx as isize, ry as isize, '#');
                        }
                    }
                }
                Instruction::RotateColumn(col, count) => {
                    for (y, c) in old.iter_mut().enumerate().take(height) {
                        *c = *screen.get(*col as isize, y as isize).unwrap();
                    }
                    debug!(?old);
                    for y in 0..height {
                        screen.set(
                            *col as isize,
                            y as isize,
                            old[(y + height - count).rem_euclid(height)],
                        );
                    }
                }
                Instruction::RotateRow(row, count) => {
                    for (x, c) in old.iter_mut().enumerate().take(width) {
                        *c = *screen.get(x as isize, *row as isize).unwrap();
                    }
                    debug!(?old);
                    for x in 0..width {
                        screen.set(
                            x as isize,
                            *row as isize,
                            old[(x + width - count).rem_euclid(width)],
                        );
                    }
                }
            }
            if event_enabled!(Level::DEBUG) {
                debug!(i=?instruction, "after");
                Picture::from(screen.clone()).display_with_mapping(|c| match c {
                    '.' => ".",
                    '#' => "#",
                    _ => " ",
                });
            }
        }

        Picture::from(screen.clone()).display_with_mapping(|c| match c {
            '.' => ".",
            '#' => "#",
            _ => " ",
        });
        let r = screen.iter().filter(|(_p, c)| *c == '#').count() as ResultType;

        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateColumn(usize, usize),
    RotateRow(usize, usize),
}
impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let rect_regex = Regex::new(r"^rect (?<x>\d+)x(?<y>\d+)$").unwrap();
        let rotate_col_regex =
            Regex::new(r"rotate column x=(?<col>\d+) by (?<count>\d+)$").unwrap();
        let rotate_row_regex = Regex::new(r"rotate row y=(?<row>\d+) by (?<count>\d+)$").unwrap();

        if let Some(c) = rect_regex.captures(&value) {
            let x = c.name("x").unwrap().as_str().parse().unwrap();
            let y = c.name("y").unwrap().as_str().parse().unwrap();
            Self::Rect(x, y)
        } else if let Some(c) = rotate_col_regex.captures(&value) {
            let col = c.name("col").unwrap().as_str().parse().unwrap();
            let count = c.name("count").unwrap().as_str().parse().unwrap();
            Self::RotateColumn(col, count)
        } else if let Some(c) = rotate_row_regex.captures(&value) {
            let col = c.name("row").unwrap().as_str().parse().unwrap();
            let count = c.name("count").unwrap().as_str().parse().unwrap();
            Self::RotateRow(col, count)
        } else {
            panic!("failed to parse {}", value);
        }
    }
}
