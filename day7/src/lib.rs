use itertools::Itertools;
use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    addresses: Vec<String>,
}
impl Solution {
    fn add_address(&mut self, address: String) {
        self.addresses.push(address);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            solution.add_address(line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut r = 0;
        for address in &self.addresses {
            let mut start = 0;
            let mut outside = true;
            let mut contains_abba_outside = false;
            let mut contains_abba_inside = false;
            for (end, c) in address.chars().enumerate() {
                match c {
                    '[' if outside => {
                        let abba = contains_abba(&address[start..end]);
                        if abba {
                            contains_abba_outside = true;
                        }
                        debug!(section = &address[start..end], outside, abba);
                        start = end;
                        outside = !outside;
                    }
                    '[' => panic!(),
                    ']' if !outside => {
                        let abba = contains_abba(&address[start..end]);
                        if abba {
                            contains_abba_inside = true;
                        }
                        debug!(section = &address[start..end], outside, abba);
                        start = end;
                        outside = !outside;
                    }
                    ']' => panic!(),
                    _ => {}
                }
            }
            let abba = contains_abba(&address[start..]);
            if abba {
                if outside {
                    contains_abba_outside = true;
                } else {
                    contains_abba_inside = true;
                }
            }
            debug!(section = &address[start..], outside, abba);
            if contains_abba_outside && !contains_abba_inside {
                r += 1;
            }
        }
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut r = 0;
        for address in &self.addresses {
            let mut start = 0;
            let mut outside = true;
            let mut outside_abas = HashSet::new();
            let mut inside_abas = HashSet::new();
            for (end, c) in address.chars().enumerate() {
                match c {
                    '[' if outside => {
                        for aba in get_abas(&address[start..end], !outside) {
                            outside_abas.insert(aba);
                        }
                        start = end;
                        outside = !outside;
                    }
                    '[' => panic!(),
                    ']' if !outside => {
                        for aba in get_abas(&address[start..end], !outside) {
                            inside_abas.insert(aba);
                        }
                        start = end;
                        outside = !outside;
                    }
                    ']' => panic!(),
                    _ => {}
                }
            }
            for aba in get_abas(&address[start..], !outside) {
                if outside {
                    outside_abas.insert(aba);
                } else {
                    inside_abas.insert(aba);
                }
            }
            if outside_abas.intersection(&inside_abas).count() > 0 {
                r += 1;
            }
        }
        // Implement for problem
        Ok(r)
    }
}

fn contains_abba(section: &str) -> bool {
    section
        .chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn get_abas(section: &str, reverse: bool) -> Vec<(char, char, char)> {
    section
        .chars()
        .tuple_windows()
        .filter(|(a, b, c)| a == c && a != b)
        .map(|(a, b, c)| if reverse { (b, a, b) } else { (a, b, c) })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn case1() {
        assert!(contains_abba("abba"));
    }

    #[test]
    #[traced_test]
    fn case2() {
        assert!(contains_abba("bddb"));
    }

    #[test]
    #[traced_test]
    fn case3() {
        assert!(!contains_abba("aaaa"));
    }

    #[test]
    #[traced_test]
    fn case4() {
        assert!(contains_abba("ioxxoj"));
    }
}
