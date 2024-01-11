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
    include_str!("./input_test.txt")
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
            let length = uniq_card.len();
            let mut score = 0;
            for letter in uniq_card {
                let mut count = card.card.iter().filter(|&ch| ch == letter).count();
                if count == 0 {
                    count = 1;
                }
                if count >= 3 && count > score {
                    if count == 3 && length != 2usize {
                        score = count;
                    } else {
                        score = count + 1;
                    }
                } else if count > 1 {
                    score = count;
                }
            }
            let mut temp_score = score;
            let base: usize = 10usize; // an explicit type is required
            for (i, val) in card.card.iter().enumerate() {
                temp_score += temp_score << 4 | dict_mapping.get(val).unwrap() 
                // println!("Temp score: {}, {}, {}", i, temp_score, val);
            }
            card.score = temp_score; 
        }
        self.hands.sort_by(|a, b| a.score.cmp(&b.score));
    }
    fn part1(&mut self) {
        let mut rank = 0;
        for (i, hand) in &mut self.hands.iter().enumerate() {

            println!("Hand : {:?}, {}", hand.card.iter().map(|ch| from_char(*ch)).collect::<Vec<_>>(), hand.score);
            
            rank += hand.bid * (i as i64 + 1);
        }
        
        println!("Rank {}", rank);
    }

    fn part2(&mut self) {

    }

    //
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace, // 0xd
}

fn from_char(ch: char) -> Card {
    match ch {
        '2' => Card::Two, // 0x0
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("code bug"),
    }
}

