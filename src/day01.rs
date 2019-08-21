use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run() {
    println!("Day01");
    match load_input() {
        Ok(s) => {
            let level = walk(s.as_str());
            println!("\tPart 1: Got to level: {}", level);
            let position = find_floor(s.as_str(), -1);
            println!("\tPart 2: Basement reached at position: {}", position);
        },
        Err(e) => panic!(e),
    }
}

fn walk(path: &str) -> i32 {
    let mut floor = 0;
    for c in path.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Undefined character: \"{}\"", c)
        }
    }
    return floor;
}

fn find_floor(path: &str, level: i32) -> usize {
    let mut floor = 0;
    for (position, c) in path.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Undefined character: \"{}\"", c)
        };
        if floor == level {
            return position + 1;
        }
    }
    panic!("Desired floor not reached.")
}

fn load_input() -> std::io::Result<String> {
    let file = File::open("input/day01.txt")?;
    let mut buffer = BufReader::new(file);
    let mut input = String::new();
    match buffer.read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(e) => panic!("Read error {}.", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(walk("(())"), 0);
        assert_eq!(walk("()()"), 0);

        assert_eq!(walk("((("), 3);
        assert_eq!(walk("(()(()("), 3);
        assert_eq!(walk("))((((("), 3);

        assert_eq!(walk("())"), -1);
        assert_eq!(walk("))("), -1);

        assert_eq!(walk(")))"), -3);
        assert_eq!(walk(")())())"), -3);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(find_floor(")", -1), 1);
        assert_eq!(find_floor("()())", -1), 5);
    }
}
