use regex::Regex;
use std::io::{BufRead, BufReader};
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
        let mut r = 0;
        for line in &self.lines {
            r += decompress_v2(line);
        }
        Ok(r)
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
            debug!(r = &line[pos..], one = e.1[0], two = e.1[1], line, e = e.0);
            let num: usize = e.1[0].parse().unwrap();
            let repeats: usize = e.1[1].parse().unwrap();
            debug!(block = &line[pos..pos + num], repeats);
            for _ in 0..repeats {
                decompressed.push_str(&line[pos..pos + num]);
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

fn decompress_v2(line: &str) -> ResultType {
    debug!(line);
    let regex = Regex::new(r"^\((?<num>\d+)x(?<repeats>\d+)\)").unwrap();
    let mut decompressed_count = 0;
    let mut pos = 0;
    loop {
        if let Some(c) = regex.captures(&line[pos..]) {
            let e: (&str, [&str; 2]) = c.extract();
            pos += e.0.len();
            debug!(r = &line[pos..], one = e.1[0], two = e.1[1], line, e = e.0);
            let num: usize = e.1[0].parse().unwrap();
            let repeats: usize = e.1[1].parse().unwrap();
            debug!(block = &line[pos..pos + num], repeats);
            decompressed_count += repeats as ResultType * decompress_v2(&line[pos..pos + num]);

            pos += num;
        } else {
            decompressed_count += 1;
            pos += 1;
        }
        if pos >= line.len() {
            break;
        }
    }
    decompressed_count
}
#[cfg(test)]
mod test {
    use super::*;

    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn decompress_v2_test1() {
        let input = "(3x3)XYZ";
        assert_eq!(9, decompress_v2(input));
    }

    #[test]
    #[traced_test]
    fn decompress_v2_test2() {
        let input = "X(8x2)(3x3)ABCY";
        assert_eq!(20, decompress_v2(input));
    }

    #[test]
    #[traced_test]
    fn decompress_v2_test3() {
        let input = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
        assert_eq!(241920, decompress_v2(input));
    }

    #[test]
    #[traced_test]
    fn decompress_v2_test4() {
        let input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
        assert_eq!(445, decompress_v2(input));
    }
}
