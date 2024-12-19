use std::io::{BufRead, BufReader};
use regex::Regex;
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

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
            solution.add_line(line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut r = 0;
        for line in &self.lines {
            let dec = decompress(line);
            debug!(line, dec);
            r += dec.len() as ResultType;
        }
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}

fn decompress(line: &str) -> String {
    let regex = Regex::new(r"^\((?<num>\d+)x(?<repeats>\d+)\)").unwrap();
    let mut decompressed = String::new();
    let mut pos = 0;
    loop {
        if let Some(c) = regex.captures(&line[pos..]) {
            let e: (&str, [&str; 2]) = c.extract();
            pos += e.0.len();
            debug!(r = &line[pos..], one = e.1[0], two=e.1[1], line, e = e.0);
            let num: usize = e.1[0].parse().unwrap();
            let repeats: usize = e.1[1].parse().unwrap();
            debug!(block = &line[pos..pos+num], repeats);
            for _ in 0..repeats {
                decompressed.push_str(&line[pos..pos+num]);
            }
            pos += num;
        } else {
            decompressed.push(line[pos..].chars().next().unwrap());
            pos += 1;
        }
        if pos >= line.len() {
            break;
        }
    }
    decompressed
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
