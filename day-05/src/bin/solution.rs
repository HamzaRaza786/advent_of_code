use std::{i64::MAX, ops::Range};

fn main() {
    let mut partNumber = GridRepresentation::new();
    partNumber.parse();
    partNumber.part2();
}
pub trait Runner {
    fn parse(&mut self);
    fn part1(&mut self);
    fn part2(&mut self);
}
#[derive(Default, Debug)]
pub struct GridRepresentation {
    seeds: Vec<i64>,
    mapping: Vec<Map>,
    seedRange: Vec<Range<i64>>
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
        let seeds = lines[0].split_once(": ").unwrap().1;
        self.seeds = seeds.split(' ').map(|seed| seed.parse().unwrap()).collect();
        for (i, val) in self.seeds.iter().enumerate().step_by(2){
           self.seedRange.push(Range{
               start: *val,
               end: *val + self.seeds[i+1]
            }); 
        } 
        let mut currmap = Map::default();
        for line in lines[2..].iter() {
            if line.contains(':') {
                self.mapping.push(currmap);
                currmap = Map::default();
                continue;
            }
            let nums: Vec<i64> = line.split(' ').map(|num| num.parse().unwrap()).collect();
            currmap.add_mapping(nums[0], nums[1], nums[2]);
        }
        if !currmap.map.is_empty() {
            self.mapping.push(currmap);
        }
    }
    fn part1(&mut self) {
        let mut min = MAX;
        for seed in self.seeds.iter() {
            let mut enter_seed = seed.clone();
            for map in self.mapping.iter() {
                enter_seed = map.apply_mapping(enter_seed);
            }
            min = min.min(enter_seed);
        }
        println!("Min is: {}", min);
    }

    fn part2(&mut self) {
        let mut min = MAX;
        for seedStruct in self.seedRange.iter() {
           for seed in seedStruct.clone().into_iter(){ 
            let mut enter_seed = seed.clone();
            for map in self.mapping.iter() {
                enter_seed = map.apply_mapping(enter_seed);
            }
            min = min.min(enter_seed);
        }
        }
        println!("Min is: {}", min);
    }
    
}
