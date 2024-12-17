use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = String;

#[derive(Debug, Default)]
pub struct Solution {
    lines: Vec<String>,
}
impl Solution {
    fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            solution.add_line(line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut r = String::new();
        let mut px = 1;
        let mut py = 1;
        for line in &self.lines {
            for c in line.chars() {
                let (dx, dy) = match c {
                    'U' => (0, -1),
                    'D' => (0, 1),
                    'L' => (-1, 0),
                    'R' => (1, 0),
                    _ => panic!(),
                };
                if px + dx >= 0 && px + dx <= 2 {
                    px += dx;
                }
                if py + dy >= 0 && py + dy <= 2 {
                    py += dy;
                }
            }
            debug!(px, py);
            r.push(match (px, py) {
                (0, 0) => '1',
                (1, 0) => '2',
                (2, 0) => '3',
                (0, 1) => '4',
                (1, 1) => '5',
                (2, 1) => '6',
                (0, 2) => '7',
                (1, 2) => '8',
                (2, 2) => '9',
                _ => panic!(),
            });
        }
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut r = String::new();
        let mut px = 0;
        let mut py = 2;
        for line in &self.lines {
            for c in line.chars() {
                let (dx, dy) = match c {
                    'U' => (0, -1),
                    'D' => (0, 1),
                    'L' => (-1, 0),
                    'R' => (1, 0),
                    _ => panic!(),
                };
                let tx = px + dx;
                let ty = py + dy;
                let valid = match (tx, ty) {
                    (0, 0) => false,
                    (1, 0) => false,
                    (3, 0) => false,
                    (4, 0) => false,
                    (0, 1) => false,
                    (4, 1) => false,
                    (0, 3) => false,
                    (4, 3) => false,
                    (0, 4) => false,
                    (1, 4) => false,
                    (3, 4) => false,
                    (4, 4) => false,
                    (x, _) if x < 0 => false,
                    (x, _) if x > 4 => false,
                    (_, y) if y < 0 => false,
                    (_, y) if y > 4 => false,
                    _ => true,
                };
                if valid {
                    px = tx;
                    py = ty;
                }
            }
            debug!(px, py);
            r.push(match (px, py) {
                (2, 0) => '1',
                (1, 1) => '2',
                (2, 1) => '3',
                (3, 1) => '4',
                (0, 2) => '5',
                (1, 2) => '6',
                (2, 2) => '7',
                (3, 2) => '8',
                (4, 2) => '9',
                (1, 3) => 'A',
                (2, 3) => 'B',
                (3, 3) => 'C',
                (2, 4) => 'D',
                _ => panic!(),
            });
        }
        // Implement for problem
        Ok(r)
    }
}
