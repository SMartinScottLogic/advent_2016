use itertools::Itertools;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{BufRead, BufReader},
};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    rooms: Vec<(String, i64, String)>,
}
impl Solution {
    fn add_room(&mut self, encname: &str, id: i64, checksum: &str) {
        self.rooms
            .push((encname.to_string(), id, checksum.to_string()));
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"^(?<encname>.*)-(?<id>\d+)\[(?<checksum>[a-zA-Z]+)\]$").unwrap();
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let c = regex.captures(&line).unwrap();
            let encname = c.name("encname").unwrap().as_str();
            let id = c.name("id").unwrap().as_str().parse().unwrap();
            let checksum = c.name("checksum").unwrap().as_str();
            solution.add_room(encname, id, checksum);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut r = 0;
        for (encname, id, checksum) in &self.rooms {
            if is_real(encname, checksum) {
                r += id;
            }
        }
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut rooms = Vec::new();
        for (encname, id, checksum) in &self.rooms {
            if is_real(encname, checksum) {
                let s = encname
                    .bytes()
                    .map(|c| c - b'a')
                    .map(|b| {
                        let c = if b == 204 {
                            b' '
                        } else {
                            ((b as i64 + *id) % 26) as u8 + b'a'
                        };
                        char::from_u32(c as u32).unwrap()
                    })
                    .collect::<String>();
                debug!(?encname, ?s);
                if s.contains("north") {
                    rooms.push((*id, s));
                }
            }
        }
        info!(?rooms);
        assert!(rooms.len() <= 1);
        let r = rooms.into_iter().map(|(id, ..)| id).next().unwrap_or(-1);
        // Implement for problem
        Ok(r)
    }
}

fn is_real(encname: &str, checksum: &str) -> bool {
    let tally: HashMap<char, usize> =
        encname
            .chars()
            .filter(|c| c.is_alphabetic())
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_default() += 1;
                acc
            });
    debug!(?tally, ?encname);
    let tally = tally
        .iter()
        .sorted_by(|(a, num_a), (b, num_b)| match num_b.cmp(num_a) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.cmp(b),
            Ordering::Greater => Ordering::Greater,
        })
        .map(|(c, _)| *c)
        .take(checksum.len())
        .collect::<String>();

    debug!(?tally, ?encname);

    tally == *checksum
}
