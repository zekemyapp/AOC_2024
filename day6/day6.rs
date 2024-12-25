use input_reader::read;
use std::env;

fn main() {
    let lines = read(env::args().collect());
    let mut result1 = 0;
    let mut result2 = 0;

    // Create map
    let mut maze: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        maze.push(row);
    }

    // Find player position
    let mut player_pos = (0, 0);
    for i in 0..maze.len() {
        let row: &Vec<char> = &maze[i];
        for j in 0..row.len() {
            let c:char = row[j];
            if c == '^' {
                player_pos = (i,j);
                break;
            }
        }
    }

    // Find initial path
    let mut maze_filled = maze.clone();
    result1 = run(player_pos, &mut maze_filled);

    for row in &maze {
        println!("{:?}", row);
    }
    println!("");

    println!("First Run:");
    for row in &maze_filled {
        println!("{:?}", row);
    }
    println!("Result1 = {}", result1);
    println!("");

    let mut beaten_path: Vec<(usize, usize)> = Vec::new();
    for i in 0..maze_filled.len() {
        let row: &Vec<char> = &maze_filled[i];
        for j in 0..row.len() {
            let c:char = row[j];
            if (c == '0' || c == '1' || c == '2' || c == '3') && (i,j) != player_pos {
                beaten_path.push((i,j));
            }
        }
    }
    println!("beaten_path {:?}", beaten_path);
    println!("");

    for modif in beaten_path {
        println!("Testing {:?}", modif);
        if run_modified(player_pos, &maze, modif) {
            result2 += 1;
        }
    }
    println!("result2 = {}", result2);
}

fn run_modified(pos: (usize, usize), maze: &Vec<Vec<char>>, modif: (usize, usize)) -> bool {
    let mut maze_clone = maze.clone();
    maze_clone[modif.0][modif.1] = 'O';

    let result = run(pos, &mut maze_clone);
    if result == -1 {
        return true;
    }
    return false;
}

fn run(pos: (usize, usize), maze: &mut Vec<Vec<char>>) -> i32 {
    let mut result:i32 = 0;
    let mut player_pos = pos;
    let mut step_result = 0;

    maze[player_pos.0][player_pos.1] = 'X';
    let mut dir = 0;
    while step_result >=0 {
        step_result = step(&mut player_pos, &mut dir, maze);
        if step_result == -2 {
            return -1;
        }
    }

    for i in 0..maze.len() {
        let row: &Vec<char> = &maze[i];
        for j in 0..row.len() {
            let c:char = row[j];
            if c == '0' || c == '1' || c == '2' || c == '3' {
                result += 1;
            }
        }
    }
    result
}

fn step(pos: &mut (usize, usize), dir: &mut u32, maze: &mut Vec<Vec<char>>) -> i64 { 
    let wall: Vec<char> = vec!['O', '#'];

    // UP
    if *dir == 0 {
        if pos.0 == 0 {
            return -1
        }

        if wall.contains(&maze[pos.0-1][pos.1]) {
            *dir = 1;
            return 0;
        }
        pos.0 -= 1;
    
    // RIGHT
    } else if *dir == 1 {
        if pos.1 == (maze[0].len()-1) {
            return -1
        }

        if wall.contains(&maze[pos.0][pos.1+1]) {
            *dir = 2;
            return 0;
        }
        pos.1 += 1;
    
    // DOWN
    } else if *dir == 2 {
        if pos.0 == (maze.len()-1) {
            return -1
        }

        if wall.contains(&maze[pos.0+1][pos.1]) {
            *dir = 3;
            return 0;
        }
        pos.0 += 1;
    
    // LEFT
    } else {
        if pos.1 == 0 {
            return -1
        }

        if wall.contains(&maze[pos.0][pos.1-1]) {
            *dir = 0;
            return 0;
        }
        pos.1 -= 1; 
    }

    let char_to_set: char = dir.to_string().chars().next().unwrap();
    if maze[pos.0][pos.1] == char_to_set {
        return -2;
    }

    maze[pos.0][pos.1] = char_to_set;
    return 0;
}
