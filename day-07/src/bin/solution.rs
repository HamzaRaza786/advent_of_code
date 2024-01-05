use std::{
    i64::{self, MAX},
    ops::Range,
};

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
    hands: Vec<SingleHand>,
}
impl GridRepresentation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Default, Debug)]
pub struct SingleHand {
    card: Vec<char>,
    bid: i64,
    score: i64,
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
        for line in lines {
            let line_split = line.split_whitespace().collect::<Vec<_>>();
            let cards = line_split[0].chars().collect::<Vec<_>>();
            let pair_count = 0;

            self.hands.push(SingleHand {
                card: cards,
                bid: line_split[1].parse().unwrap(),
                score: 0,
            })
        }
        for card in &self.hands {
            for letter in card.card {
            }
            println!("{:?}", card.card);
        }
    }
    fn part1(&mut self) {}
    fn part2(&mut self) {}
    //
}
