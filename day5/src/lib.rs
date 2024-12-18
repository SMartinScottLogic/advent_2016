use std::io::{BufRead, BufReader, Read};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = String;

#[derive(Debug, Default)]
pub struct Solution {
    door_id: String,
}
impl Solution {
    fn set_doorid(&mut self, door_id: String) {
        self.door_id = door_id;
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            solution.set_doorid(line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut r = String::new();
        let mut i = 0;
        let mut found = 0;
        loop {
            let md5 = get_md5(&self.door_id, i);
            if &md5[..5] == "00000" {
                let ch = md5.chars().nth(5).unwrap();
                debug!(i, ?ch);
                r.push(ch);
                found += 1;
                if found == 8 {
                    break;
                }
            }
            i += 1;
        }
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut r = Vec::new();
        r.resize(8, '_');
        println!("{}", r.iter().collect::<String>());

        let mut i = 0;
        loop {
            let md5 = get_md5(&self.door_id, i);
            if &md5[..5] == "00000" {
                let pos = md5.chars().nth(5).unwrap().to_digit(16).unwrap() as usize;
                let ch = md5.chars().nth(6).unwrap();
                debug!(?r, i, ?ch, pos);
                if pos < 8 && r[pos] == '_' {
                    r[pos] = ch;
                    println!("{}", r.iter().collect::<String>());
                    if r.iter().all(|c| *c != '_') {
                        break;
                    }
                }
            }
            i += 1;
        }
        // Implement for problem
        Ok(r.iter().collect())
    }
}

fn get_md5(prefix: &str, value: i32) -> String {
    let trial = format!("{}{}", prefix, value);
    let md5 = md5::compute(&trial);
    debug!(value, md5=?md5);
    md5.bytes()
        .flatten()
        .take(4)
        .fold(String::new(), |mut acc, v| {
            acc.push_str(&format!("{:02x}", v));
            acc
        })
}
