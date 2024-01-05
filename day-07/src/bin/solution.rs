use std::{
    char,
    collections::{HashMap, HashSet},
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
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleHand {
    card: Vec<char>,
    bid: i64,
    score: usize,
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
        let dict_mapping: HashMap<char, usize> = HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
            ('1', 1),
        ]);

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
        for card in &mut self.hands {
            let uniq_card = card
                .card
                .iter()
                .map(|c| c)
                .collect::<HashSet<&char>>()
                .into_iter()
                .collect::<Vec<&char>>();
            let mut score = 0;
            for letter in uniq_card {
                let count = card.card.iter().filter(|&ch| ch == letter).count();
                if count > 1 && count > score {
                    score = count;
                }
            }
            let mut temp_score = 0;
            let base: usize = 10usize; // an explicit type is required
            for (i, val) in card.card.iter().enumerate() {
                temp_score += dict_mapping.get(val).unwrap() * base.pow(5 - i as u32);
            }
            card.score = temp_score << score;
        }
        self.hands.sort_by(|a,b| a.score.cmp(&b.score));
    }
    fn part1(&mut self) {
       let mut rank = 0;
        for (i,hand) in self.hands.iter().enumerate() {
            
            rank+= hand.bid * (i as i64 + 1);
        }
        println!("Rank {}", rank);
    }

    fn part2(&mut self) {}
    //
}
