use std::fs::read_to_string;

pub fn read(args: Vec<String>) -> Vec<String> {
        let input = &args[1];
        let lines = read_lines(input);    
        lines
}

fn read_lines(filename: &str) -> Vec<String> {
        let mut result = Vec::new();

        for line in read_to_string(filename).unwrap().lines() {
                result.push(line.to_string())
        }
        result
}