use itertools::Itertools;
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    triangles: Vec<(ResultType, ResultType, ResultType)>,
}
impl Solution {
    fn add_triangle(&mut self, a: ResultType, b: ResultType, c: ResultType) {
        self.triangles.push((a, b, c));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let (a, b, c) = line
                .split_whitespace()
                .map(|v| v.parse::<ResultType>().unwrap())
                .next_tuple()
                .unwrap();
            solution.add_triangle(a, b, c);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let r = self
            .triangles
            .iter()
            .filter(|(a, b, c)| *a < (*b + *c) && *b < (*a + *c) && *c < (*a + *b))
            .count() as ResultType;

        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut r = 0;
        for (a, b, c) in self.triangles.iter().tuples() {
            if a.0 < (b.0 + c.0) && b.0 < (a.0 + c.0) && c.0 < (a.0 + b.0) {
                r += 1;
            }
            if a.1 < (b.1 + c.1) && b.1 < (a.1 + c.1) && c.1 < (a.1 + b.1) {
                r += 1;
            }
            if a.2 < (b.2 + c.2) && b.2 < (a.2 + c.2) && c.2 < (a.2 + b.2) {
                r += 1;
            }
        }
        // Implement for problem
        Ok(r)
    }
}
