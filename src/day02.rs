use std::fs::File;
use std::io::{BufReader, BufRead};

struct Size {
    length: u32,
    width: u32,
    height: u32,
}

impl Size {
    fn from_string(s: &str) -> Size {
        let dims: Vec<&str> = s.split("x").collect();
        let values: Vec<u32> = dims.iter()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        Size { length: values[0], width: values[1], height: values[2] }
    }

    fn sorted(&self) -> Vec<u32> {
        let mut a = vec![self.length, self.width, self.height];
        a.sort();
        a
    }
}

pub fn run() {
    println!("Day02");
    match load_input() {
        Ok(sizes) => {
            let total_wrap: u32 = sizes.iter().map(wrap_usage).sum();
            println!("\tPart 1: Total square feet of wrapping paper = {}", total_wrap);
            let total_ribbon: u32 = sizes.iter().map(ribbon_usage).sum();
            println!("\tPart 2: Total feets of ribbon = {}", total_ribbon);
        },
        Err(_) => panic!("Could not read input."),
    }
}

fn wrap_usage(size: &Size) -> u32 {
    let area_1 = size.length * size.width;
    let area_2 = size.width * size.height;
    let area_3 = size.height * size.length;
    let smallest_area = area_1.min(area_2).min(area_3);
    2 * area_1 + 2 * area_2 + 2 * area_3 + smallest_area
}

fn ribbon_usage(size: &Size) -> u32 {
    let sorted = size.sorted();
    2 * sorted[0] + 2 * sorted[1] + size.length * size.height * size.width
}

fn load_input() -> std::io::Result<Vec<Size>> {
    let file = File::open("input/day02.txt")?;
    let buffer = BufReader::new(file);
    let mut lines: Vec<Size> = Vec::new();
    for line in buffer.lines() {
        match line {
            Ok(s) => {
                lines.push(Size::from_string(s.as_str()))
            },
            Err(_) => panic!("Failed to read line."),
        }
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(wrap_usage(&Size::from_string("2x3x4")), 58);
        assert_eq!(wrap_usage(&Size::from_string("1x1x10")), 43);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(ribbon_usage(&Size::from_string("2x3x4")), 34);
        assert_eq!(ribbon_usage(&Size::from_string("1x1x10")), 14);
    }
}

