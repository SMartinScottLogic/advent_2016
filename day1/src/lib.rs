use regex::Regex;
use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};
use utils::point::Direction;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    actions: Vec<(char, ResultType)>,
}
impl Solution {
    fn add_action(&mut self, direction: &str, distance: ResultType) {
        self.actions
            .push((direction.chars().next().unwrap(), distance));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"^(?<dir>.)(?<move>\d+)$").unwrap();
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            for action in line.split(',') {
                let c = regex.captures(action.trim()).unwrap();
                let dir = c.name("dir").unwrap().as_str();
                let distance = c.name("move").unwrap().as_str().parse().unwrap();
                solution.add_action(dir, distance);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut dir = Direction::N;
        let mut px = 0;
        let mut py = 0;
        for (turn, distance) in &self.actions {
            dir = match (dir, turn) {
                (Direction::N, 'L') => Direction::W,
                (Direction::N, 'R') => Direction::E,
                (Direction::E, 'L') => Direction::N,
                (Direction::E, 'R') => Direction::S,
                (Direction::S, 'L') => Direction::E,
                (Direction::S, 'R') => Direction::W,
                (Direction::W, 'L') => Direction::S,
                (Direction::W, 'R') => Direction::N,
                _ => panic!(),
            };
            let (dx, dy) = match dir {
                Direction::N => (0, -1),
                Direction::E => (1, 0),
                Direction::S => (0, 1),
                Direction::W => (-1, 0),
                _ => panic!(),
            };
            px += dx * distance;
            py += dy * distance;
        }
        // Implement for problem
        Ok(px.abs() + py.abs())
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut dir = Direction::N;
        let mut px = 0 as ResultType;
        let mut py = 0 as ResultType;
        let mut visits = HashSet::new();
        'abort: for (turn, distance) in &self.actions {
            dir = match (dir, turn) {
                (Direction::N, 'L') => Direction::W,
                (Direction::N, 'R') => Direction::E,
                (Direction::E, 'L') => Direction::N,
                (Direction::E, 'R') => Direction::S,
                (Direction::S, 'L') => Direction::E,
                (Direction::S, 'R') => Direction::W,
                (Direction::W, 'L') => Direction::S,
                (Direction::W, 'R') => Direction::N,
                _ => panic!(),
            };
            let (dx, dy) = match dir {
                Direction::N => (0, -1),
                Direction::E => (1, 0),
                Direction::S => (0, 1),
                Direction::W => (-1, 0),
                _ => panic!(),
            };
            for _ in 0..*distance {
                px += dx;
                py += dy;
                if visits.contains(&(px, py)) {
                    break 'abort;
                }
                visits.insert((px, py));
            }
        }
        // Implement for problem
        Ok(px.abs() + py.abs())
    }
}
