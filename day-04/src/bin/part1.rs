use std::collections::HashSet;
fn main() {
    let line = read_lines();
    part2(line);
}
pub fn read_lines() -> Vec<String> {
    include_str!("./input.txt")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn part1(vector: Vec<String>) {
    let mut totalCopies = 0;
    for game in vector {
        let cards = game.split(':').collect::<Vec<&str>>()[1].split('|').collect::<Vec<&str>>();
        let wining_numbers = cards[0].split_whitespace();
        let check_number = cards[1].split_whitespace();
        let mut total_wining_numbers = 0;
        let mut repeated_numbers = 0;
        for wn in wining_numbers {
            if check_number
                .clone()
                .filter(|&n| {
                    println!("{} : {}", n, wn);
                    *n == *wn
                })
                .count()
                >= 1
            {
                repeated_numbers += 1;
            }
        }
        println!("repeated numbers {}", repeated_numbers);
        if repeated_numbers != 0 {
            total_wining_numbers += 2_u32.pow(repeated_numbers - 1);
        }
        totalCopies += total_wining_numbers;
    }
    println!("{}", totalCopies);
}
fn part2(vector: Vec<String>) {
    let mut multiplier = vec![1usize; vector.len()];
    for (index, game) in vector.iter().enumerate() {
        let cards = game.split(':').collect::<Vec<&str>>()[1].split('|').collect::<Vec<&str>>();
        let wining_numbers = cards[0].split_whitespace();
        let check_number = cards[1].split_whitespace();
        let mut repeated_numbers = 0;
        for wn in wining_numbers {
            if check_number.clone().filter(|&n| *n == *wn).count() >= 1 {
                repeated_numbers += 1;
            }
        }
        for i in index + 1..index + 1 + repeated_numbers{
            multiplier[i] += multiplier[index];
        
        println!("{repeated_numbers} : {i} : {}", multiplier[i]);
        }
        println!("In index: {}", index);
    }
    println!("{:?}", &multiplier);
    println!("{}", multiplier.iter().sum::<usize>());
}
#[cfg(test)]

mod tests {
    use super::*;
    // fn part_1_works() {
    //     let result = part1("abcd1\n, 2cdc4\n, 323rr3\n");
    //     assert_eq!(result, 68);
    // }
    #[test]

    fn part_2_works() {
        let result = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let ans = part2( 
        result.split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect());

        assert_eq!("", "1");
    }
}
