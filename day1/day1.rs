use std::fs::read_to_string;
use regex::Regex;
use std::env;

fn main() {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let lines = read_lines(input);
    for line in lines {
        let vals = parse_line(line);
        left.push(vals.0);
        right.push(vals.1);
        println!("left {} right {}", vals.0, vals.1);
    }

    left.sort();
    right.sort();
    let mut left2 = left.clone();
    let right2 = right.clone();

    println!("{:?}", left);
    println!("{:?}", right);

    if left.len() != right.len() {
        println!("Both colomns have different sizes");
    }

    let mut result: i32 = 0;
    while !left.is_empty() {
        let l = left.remove(0);
        let r = right.remove(0);
        println!("{} vs {}", l, r);

        result += (l-r).abs();
    }

    println!("Resut Part One: {}", result);

    let mut result2: i32 = 0;
    while !left2.is_empty() {
        let l = left2.remove(0);
        let count:i32 = right2.iter().filter(|&&x| x == l).count() as i32;
        println!("{} appears {} times", l, count);

        result2 += l*count;
    }

    println!("Resut Part Two: {}", result2);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn parse_line(line: String) -> (i32, i32) {
    let re = Regex::new(r"\d+").unwrap();
    let line_slice: &str = &line;
    let numbers: Vec<String> = re.find_iter(line_slice)
        .map(|mat| mat.as_str().to_string())
        .collect();

    let left: i32 = numbers[0].parse().unwrap();
    let right: i32 = numbers[1].parse().unwrap();

    (left, right)
}