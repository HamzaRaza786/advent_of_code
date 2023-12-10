
fn main() {
    let input = include_str!("./input.txt");
    part2(input);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut total = 0;
    for line in lines {
        println!("{}", line);
        let mut con = false;
        let game: Vec<_> = line.split(':').collect();
        let round = game.get(1).unwrap().split(';');
        for r in round {
            println!("{}", r);
            let balls: Vec<_> = r.split(',').collect();
            for b in balls {
                let vectorr = b.split_whitespace().collect::<Vec<_>>();
                let numberInIt = vectorr.get(0).unwrap().parse::<u32>().unwrap();
                if b.contains("red") && numberInIt > 12 {
                    con = true;
                    break;
                }

                if b.contains("green") && numberInIt > 13 {
                    con = true;
                    break;
                }

                if b.contains("blue") && numberInIt > 14 {
                    con = true;
                    break;
                }
            }
            if con == true {
                break;
            }
        }
        if con == false {
            total = total;

            let val = game
                .get(0)
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>()
                .get(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            println!("Val {}", val);
            total = total + val;
        }
    }
    println!("total : {}", total);
    return total;
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut total = 0;

    for line in lines {
        println!("{}", line);
        let mut con = false;
        let game: Vec<_> = line.split(':').collect();
        let round = game.get(1).unwrap().split(';');
            let mut max_red = 0;
            let mut max_blue = 0;
            let mut max_green = 0;
        for r in round {
            println!("{}", r);
            let balls: Vec<_> = r.split(',').collect();
            for b in balls {
                let vectorr = b.split_whitespace().collect::<Vec<_>>();
                let numberInIt = vectorr.get(0).unwrap().parse::<u32>().unwrap();
                if b.contains("red") {
                    if max_red < numberInIt {
                        max_red = numberInIt;
                    }
                }

                if b.contains("green") {
                    if max_green < numberInIt {
                        max_green = numberInIt;
                    }
                }

                if b.contains("blue") {
                    if max_blue < numberInIt {
                        max_blue = numberInIt;
                    }
                }
            }
            
        }
    total = total + max_blue * max_red * max_green;
    }
    println!("total : {}", total);
    return total;
}

#[cfg(test)]

mod tests {
    use super::*;
    fn part_1_works() {
        let result = part1("abcd1\n, 2cdc4\n, 323rr3\n");
        assert_eq!(result, 68);
    }
    #[test]

    fn part_2_works() {
        let result = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let ans = part2(result);
        assert_eq!(ans, 2286);
    }
}
