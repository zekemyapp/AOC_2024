use std::fs::read_to_string;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let lines = read_lines(input);
    let mut result1 = 0;
    let mut result2 = 0;
    let mut enabled = true;
    for line in lines {
        let ops = parse_line(line);
        for op in ops {
            if op.chars().next().unwrap() == 'm' {
                let x = parse_digits(&op);
                println!("{} = {}", op, x);
                result1 += x;
                if enabled {
                    result2 += x;
                }
            } else {
                if op.len() == 4 {
                    enabled = true;
                } else {
                    enabled = false;
                }
                println!("{}", op);
            }
        }
    }
    println!("result1 = {}", result1);
    println!("result2 = {}", result2);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn parse_line(line: String) -> Vec<String> {
    let re = Regex::new(r"mul\((-?\d+),\s*(-?\d+)\)|do\(\)|don't\(\)").unwrap();
    let line_slice: &str = &line;
    let ops: Vec<String> = re.find_iter(line_slice)
        .map(|mat| mat.as_str().to_string())
        .collect();
    return ops
}

fn parse_digits(line: &String) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    let line_slice: &str = &line;
    let numbers: Vec<String> = re.find_iter(line_slice)
        .map(|mat| mat.as_str().to_string())
        .collect();

    let x:i32 = numbers[0].parse().unwrap();
    let y:i32 = numbers[1].parse().unwrap();
    return  x*y;
}
