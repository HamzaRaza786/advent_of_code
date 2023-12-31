use std::{i64::{MAX, self}, ops::Range};

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
    time: Vec<i64>,
    distance: Vec<i64>,
}
impl GridRepresentation {
    pub fn new() -> Self {
        Self::default()
    }
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
        let lines = read_lines("./input_test.txt");
        self.time = lines[0].split_once(": ").unwrap().1.split_whitespace().map(|s| s.parse().unwrap()).collect();
        self.distance = lines[1].split_once(": ").unwrap().1.split_whitespace().map(|s| s.parse().unwrap()).collect();
    }
    fn part1(&mut self) 
    {
        let mut sum = 1;
        for (i, val) in self.time.iter().enumerate(){
            let mut count = 0;
            for j in 1..val + 1{
                if ((val - j) * j) > self.distance[i]{
                    count+=1
                }
        }
            if count > 0 {
                sum *=count 
            }
    }
        println!("{}", sum);
    }
    fn part2(&mut self) {
        let combined_time = self.time.iter().map(|m| m.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap();
        let combined_distance = self.distance.iter().map(|m| m.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap();

        let mut count = 0;
            for j in 1..combined_time + 1{
                if ((combined_time - j) * j) > combined_distance{
                    count+=1
                }
        }
        println!("{}", count);
        }
    // 
}
