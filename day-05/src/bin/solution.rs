use std::collections::HashSet;

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
#[derive(Default)]
pub struct GridRepresentation {
    nums: Vec<PartNumbers>,
    syms: HashSet<(i64, i64)>,
    gearRatios: HashSet<(i64, i64)>,
}
impl GridRepresentation {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
struct PartNumbers {
    value: i64,
    points: HashSet<(i64, i64)>,
}
impl PartNumbers {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1),
            (row - 1, col),
            (row + 1, col),
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1),
        ]);
        Self {
            value: (ch as u8 - b'0') as i64,
            points,
        }
    }
    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value = self.value * 10 + (ch as u8 - b'0') as i64;
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)])
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
        let lines = read_lines("./input.txt");
        let mut curr_number: Option<PartNumbers> = None;
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch.is_ascii_digit() {
                    if let Some(ref mut num) = curr_number {
                        println!("{:?}", num);
                        num.add_digit(row as i64, col as i64, ch);
                    } else {
                        curr_number = Some(PartNumbers::new(row as i64, col as i64, ch));
                    }
                } else {
                    if let Some(num) = curr_number.take() {
                        self.nums.push(num);
                    }
                    if ch != '.' {
                        self.syms.insert((row as i64, col as i64));
                    }
                    if ch == '*' {
                        self.gearRatios.insert((row as i64, col as i64));
                    }
                }
            }
        }
    }
    fn part1(&mut self) {
        let mut total = 0;
        for num in &self.nums {
            if num.points.intersection(&self.syms).next().is_some() {
                total += num.value;
            }
        }
        println!("{}", total);
    }

    fn part2(&mut self) {
        let mut total = 0;
        'next_gear: for gears in &self.gearRatios {
            let mut matched = Vec::new();
            for num in &self.nums {
                if num.points.contains(gears) {
                    if matched.len() == 2 {
                        continue 'next_gear;
                    }
                    matched.push(num.value);
                }
            }
            if matched.len() == 2 {
                total += matched[0] * matched[1];
            }
        }
        println!("{}", total);
    }
}


