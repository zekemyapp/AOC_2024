use input_reader::read;
use std::env;

use regex::Regex;

fn main() {
    let lines = read(env::args().collect());
    let mut result1 = 0;
    let mut result2 = 0;
    
    let mut reading_pages = false;
    let mut rules: Vec<(u32, u32)> = Vec::new();

    for line in lines {
        if line.len() == 0 {
            reading_pages = true;
            println!("{:?}", rules);
            continue;
        }

        if !reading_pages {
            // Reading rules
            let rule = parse_rule(line);
            rules.push(rule);
            continue
        }

        // Reading pages
        println!("update: {}", line);
        let result = check_update(&rules, line);
        result1 += result.0;
        result2 += result.1;
    }

    println!("result1 = {}", result1);
    println!("result2 = {}", result2);
}

fn parse_rule(line: String) -> (u32, u32) {
    let re = Regex::new(r"\d+").unwrap();
    let line_slice: &str = &line;
    let numbers: Vec<String> = re.find_iter(line_slice)
        .map(|mat| mat.as_str().to_string())
        .collect();

    let left: u32 = numbers[0].parse().unwrap();
    let right: u32 = numbers[1].parse().unwrap();

    (left, right)
}

fn get_pages_by_rule(rules: &Vec<(u32, u32)>, page: u32) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();
    for rule in rules {
        if rule.0 == page {
            output.push(rule.1);
        }
    }
    output
}

fn check_update(rules: &Vec<(u32, u32)>, line: String) -> (u32, u32) {
    let re = Regex::new(r"\d+").unwrap();
    let line_slice: &str = &line;
    let mut numbers: Vec<String> = re.find_iter(line_slice)
        .map(|mat| mat.as_str().to_string())
        .collect();

    let midpoint = numbers.len() / 2;
    let mut output: u32 = numbers[midpoint].parse().unwrap();

    let mut part2 = false;
    for i in 0..numbers.len() {
        let current: u32 = numbers[i].parse().unwrap();
        let pages = get_pages_by_rule(&rules, current);

        for j in 0..i {
            let n: u32 = numbers[j].parse().unwrap();
            if pages.contains(&n) {
                part2 = true;
            }
        }
    }

    if !part2 {
        return (output, 0);
    }

    let mut exit = false;
    while !exit {
        exit = true;
        for i in 0..numbers.len() {
            let current: u32 = numbers[i].parse().unwrap();
            let pages = get_pages_by_rule(&rules, current);
    
            for j in 0..i {
                let n: u32 = numbers[j].parse().unwrap();
                if pages.contains(&n) {
                    exit = false;
                    println!("prev {:?}", numbers);
                    numbers.swap(i,j);
                    println!("next {:?}", numbers);
                    break;
                }
            }
        }
    }

    output = numbers[midpoint].parse().unwrap();
    return (0, output);
}
