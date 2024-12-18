use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = String;

#[derive(Debug, Default)]
pub struct Solution {
    messages: Vec<String>,
}
impl Solution {
    fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            solution.add_message(line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut freq: HashMap<usize, HashMap<char, usize>> = HashMap::new();
        for message in &self.messages {
            for (i, c) in message.chars().enumerate() {
                *freq.entry(i).or_default().entry(c).or_default() += 1;
            }
        }
        let mut r = String::new();
        for i in 0..freq.len() {
            let c = freq[&i].iter().max_by_key(|v| v.1).unwrap().0;
            r.push(*c);
        }
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut freq: HashMap<usize, HashMap<char, usize>> = HashMap::new();
        for message in &self.messages {
            for (i, c) in message.chars().enumerate() {
                *freq.entry(i).or_default().entry(c).or_default() += 1;
            }
        }
        let mut r = String::new();
        for i in 0..freq.len() {
            let c = freq[&i].iter().min_by_key(|v| v.1).unwrap().0;
            r.push(*c);
        }
        Ok(r)
    }
}
