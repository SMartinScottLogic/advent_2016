use std::io::{BufRead, BufReader};
use itertools::Itertools;
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
            let (a, b, c) = line.split_whitespace().map(|v| v.parse::<ResultType>().unwrap()).next_tuple().unwrap();
            solution.add_triangle(a, b, c);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let r = self.triangles.iter().filter(|(a, b, c)| {
            *a < (*b + *c) && *b < (*a + *c) && *c < (*a + *b)
        }).count() as ResultType;
        
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    use tracing_test::traced_test;
    use utils::Solution;

    #[test]
    #[traced_test]
    fn read() {
        let input = "replace for problem";
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(0 as ResultType, s.answer_part1(false).unwrap());
    }
}
