use std::fs::read_to_string;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let lines = read_lines(input);
    let mut result = 0;
    let mut result2 = 0;
    for line in lines {
        let vals = parse_line(line);
        println!("{:?}", vals);

        let size = vals.len();
        let mut first_vals = vals.clone();

        let mut last_v:i32 = first_vals.remove(0).parse().unwrap();
        let mut dir: i32 = -1;
        let mut exit = false;
        while !first_vals.is_empty() && !exit {
            let v:i32 = first_vals.remove(0).parse().unwrap();
            if dir==1 && v < last_v {
                exit = true;
            } else if dir==0 && v > last_v {
                exit = true;
            } else if dir==-1{
                dir = if v > last_v {1} else {0};
            }
            let abs = (v-last_v).abs();
            if abs < 1 || abs > 3 {
                exit = true;
            }
            last_v = v;
        }

        if !exit {
            result+=1;
        }

        let mut exit2 = false;
        let mut n:i32 = -1;
        while n < size.try_into().unwrap() && !exit2 {
            let mut second_vals = vals.clone();
            if n >= 0 {
                second_vals.remove(n.try_into().unwrap());
            }

            last_v = second_vals.remove(0).parse().unwrap();
            dir = -1;
            exit = false;
            while !second_vals.is_empty() && !exit {
                let v:i32 = second_vals.remove(0).parse().unwrap();
                if dir==1 && v < last_v {
                    exit = true;
                } else if dir==0 && v > last_v {
                    exit = true;
                } else if dir==-1{
                    dir = if v > last_v {1} else {0};
                }
                let abs = (v-last_v).abs();
                if abs < 1 || abs > 3 {
                    exit = true;
                }
                last_v = v;
            }

            if !exit {
                result2+=1;
                exit2 = true;
            }

            n+=1;
        }
    }

    println!("Result Part One: {}", result);
    println!("Result Part One: {}", result2);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn parse_line(line: String) -> Vec<String> {
    let re = Regex::new(r"\d+").unwrap();
    let line_slice: &str = &line;
    let numbers: Vec<String> = re.find_iter(line_slice)
        .map(|mat| mat.as_str().to_string())
        .collect();
    return numbers
}