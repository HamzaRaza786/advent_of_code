fn main() {
    let input = include_str!("./input.txt");

    part2(input);
}

fn part1(input: &str) -> usize {
    let mut _total: usize = 0;
    let lines = input.lines();
    for line in lines {
        let arr: Vec<_> = line
            .chars()
            .filter(|&character| character >= '0' && character <= '9')
            .collect();
        let combined_string =
            String::from(*arr.get(0).unwrap()) + &String::from(*arr.get(arr.len() - 1).unwrap());
        _total += combined_string.parse::<usize>().unwrap();
    }
    return _total;
}

fn part2(input: &str) -> usize {
    let mut _total: usize = 0;
    let lines = input.lines();
    for line in lines {
        let line = line.replace("one", "o1e").replace("two","t2o")
        .replace("three","t3e")

        .replace("four","f4r")
        .replace("five","f5e")
        .replace("six","s6x")
        .replace("seven","s7n")
        .replace("eight","e8t")

        .replace("nine","9");
        let arr: Vec<_> = line
            .chars()
            .filter(|&character| character >= '0' && character <= '9')
            .collect();
        let combined_string =
            String::from(*arr.get(0).unwrap()) + &String::from(*arr.get(arr.len() - 1).unwrap());
        _total += combined_string.parse::<usize>().unwrap();
    }
    println!("{}", _total);
    return _total;
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn part_1_works() {
        let result = part1("abcd1\n, 2cdc4\n, 323rr3\n"); 
        assert_eq!(result, 68);
    }
fn part_2_works() {
        let result = part1("abcd1\n, 2cdc4\n, 323rr3\n"); 
        assert_eq!(result, 68);
    }
}
