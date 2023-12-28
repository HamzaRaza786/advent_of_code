use std::{i64::MAX, ops::Range};

fn main() {
    let mut partNumber = GridRepresentation::new();
    partNumber.parse();
    partNumber.part1();
}
pub trait Runner {
    fn parse(&mut self);
    fn part1(&mut self);
    fn part2(&mut self);
}
#[derive(Default, Debug)]
pub struct GridRepresentation {
    time: Vec<i64>,
    distance: Vec<i64>,
}
impl GridRepresentation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Default)]
pub struct Seeds {
    start: i64,
    length: i64
}
#[derive(Debug, Default)]
pub struct Map {
    map: Vec<SingleMap>,
}
impl Map {
    fn add_mapping(&mut self, dest: i64, src: i64, len: i64) {
        self.map.push(SingleMap {
            range: Range {
                start: src,
                end: src + len,
            },
            delta: dest - src,
        });
    }
    fn apply_mapping(&self, seed: i64) -> i64 {
        for sm in &self.map {
            if sm.range.contains(&seed) {
                return seed + sm.delta;
            }
        }
        seed
    }
    fn reverse_lookup(&self, val: i64) -> i64 {
        for map in &self.map {
            let rev = val - map.delta;
            if map.range.contains(&rev){
                return rev;
            }
        }
        val
    }
}
#[derive(Debug)]
pub struct SingleMap {
    range: Range<i64>,
    delta: i64,
}

pub fn read_lines(_pathname: &str) -> Vec<String> {
    include_str!("./input.txt")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
impl Runner for GridRepresentation {
    fn parse(&mut self) {
        let lines = read_lines("./input.txt");
        self.time = lines[0].split_once(": ").unwrap().1.split_whitespace().map(|s| s.parse().unwrap()).collect();
        self.distance = lines[1].split_once(": ").unwrap().1.split_whitespace().map(|s| s.parse().unwrap()).collect();
    }
    fn part1(&mut self) 
    {
        for (i, val) in self.time.iter().enumerate(){
            for j in 1..self.time[i]/2{
                
            }
        }
    }

    fn part2(&mut self) {
    }
    // 
}
