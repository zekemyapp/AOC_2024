use std::fs::read_to_string;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let lines = read_lines(input);
    let mut result1 = 0;
    let mut result2 = 0;

    let mut letters: Vec<Vec<char>> = vec![];
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        letters.push(row);
    }

    for row in &letters{
        for c in row {
                print!("{}", c);
        }
        print!("\n");
    }

    result1 = calculate(&letters);
    result2 = calculate2(&letters);

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

fn calculate(letters: &Vec<Vec<char>>) -> i32 {
        let mut result: i32 = 0;
        for i in 0..(letters.len()){
                let row = &letters[i];
                for j in 0..(row.len()) {
                        let c = row[j];
                        if c == 'X' {
                              if get_right(&letters, i, j) {
                                result += 1;
                              }
                              if get_down(&letters, i, j) {
                                result += 1;
                              }
                              if get_up(&letters, i, j) {
                                result += 1;
                              }
                              if get_left(&letters, i, j) {
                                result += 1;
                              }
                              if get_rightdown(&letters, i, j) {
                                result += 1;
                              }
                              if get_rightup(&letters, i, j) {
                                result += 1;
                              }
                              if get_leftdown(&letters, i, j) {
                                result += 1;
                              }
                              if get_leftup(&letters, i, j) {
                                result += 1;
                              }
                        }
                }
        }
        return result;
}

fn get_right(letters: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if (j+3) >= letters[i].len().try_into().unwrap() {
                return false;
        }

        if letters[i][j] == 'X' && letters[i][j+1] == 'M' && letters[i][j+2] == 'A' && letters[i][j+3] == 'S' {
                return true;
        }

        return false;
}

fn get_down(letters: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if (i+3) >= letters.len().try_into().unwrap() {
                return false;
        }

        if letters[i][j] == 'X' && letters[i+1][j] == 'M' && letters[i+2][j] == 'A' && letters[i+3][j] == 'S' {
                return true;
        }

        return false;
}

fn get_rightdown(letters: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if (j+3) >= letters[i].len().try_into().unwrap() {
                return false;
        }
        if (i+3) >= letters.len().try_into().unwrap() {
                return false;
        }

        if letters[i][j] == 'X' && letters[i+1][j+1] == 'M' && letters[i+2][j+2] == 'A' && letters[i+3][j+3] == 'S' {
                return true;
        }

        return false;
}

fn get_up(letters: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if (i) < 3  {
                return false;
        }

        if letters[i][j] == 'X' && letters[i-1][j] == 'M' && letters[i-2][j] == 'A' && letters[i-3][j] == 'S' {
                return true;
        }

        return false;
}

fn get_rightup(letters: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if (j+3) >= letters[i].len().try_into().unwrap() {
                return false;
        }
        if (i) < 3  {
                return false;
        }

        if letters[i][j] == 'X' && letters[i-1][j+1] == 'M' && letters[i-2][j+2] == 'A' && letters[i-3][j+3] == 'S' {
                return true;
        }

        return false;
}

fn get_left(letters: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if (j) < 3 {
                return false;
        }

        if letters[i][j] == 'X' && letters[i][j-1] == 'M' && letters[i][j-2] == 'A' && letters[i][j-3] == 'S' {
                return true;
        }

        return false;
}

fn get_leftdown(letters: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if (j) < 3 {
                return false;
        }
        if (i+3) >= letters.len().try_into().unwrap() {
                return false;
        }

        if letters[i][j] == 'X' && letters[i+1][j-1] == 'M' && letters[i+2][j-2] == 'A' && letters[i+3][j-3] == 'S' {
                return true;
        }

        return false;
}

fn get_leftup(letters: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if (j) < 3 {
                return false;
        }
        if (i) < 3  {
                return false;
        }

        if letters[i][j] == 'X' && letters[i-1][j-1] == 'M' && letters[i-2][j-2] == 'A' && letters[i-3][j-3] == 'S' {
                return true;
        }

        return false;
}

fn calculate2(letters: &Vec<Vec<char>>) -> i32 {
        let mut result: i32 = 0;
        for i in 0..(letters.len()){
                let row = &letters[i];
                for j in 0..(row.len()) {
                        let c = row[j];
                        if c == 'A' {
                                if get_mas(&letters, i, j) {
                                        result += 1;
                                }
                        }
                }
        }
        return result;
}

fn get_mas(letters: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if (j+1) >= letters[i].len().try_into().unwrap() {
                return false;
        }
        if (i+1) >= letters.len().try_into().unwrap() {
                return false;
        }
        if (j) < 1 {
                return false;
        }
        if (i) < 1  {
                return false;
        }

        if (letters[i+1][j+1] == 'M' && letters[i-1][j-1] == 'S') || (letters[i+1][j+1] == 'S' && letters[i-1][j-1] == 'M') {
                if (letters[i-1][j+1] == 'M' && letters[i+1][j-1] == 'S') || (letters[i-1][j+1] == 'S' && letters[i+1][j-1] == 'M') {
                        return true;
                }
        }

        return false;
}
