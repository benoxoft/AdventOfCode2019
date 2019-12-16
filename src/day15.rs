use std::collections::HashMap;
use crate::intcode::{run_intcode, InputGenerator, OutputHandler};
use std::sync::Mutex;
use rand::Rng;
use std::fs::File;
use std::io::Write;
use leemaze::{AllowedMoves2D, boolify_2d_maze, maze_directions2d};

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<i64> {    
    let ret = input.split(",").map(|token| token.parse::<i64>().unwrap()).collect();
    ret
}

fn parse_maze_input() -> (Vec<Vec<bool>>, (usize, usize), (usize, usize)) {
    let maze_str = "###########################################
###########################################
##.........#.....#...#...#.............####
####.#####.#.#.#.#.#.#.#.#.#######.###.#.##
##...#...#.#.#.#.#.#...#.#.....#.#.#.....##
##.#####.#.###.#.#######.#####.#.#.#####.##
##.#...#.....#.#.......#.........#...#...##
##.#.#.#####.#.#######.#####.#####.#.######
##...#...#.....#...#.......#.#...#.#.#...##
##.##.##.#######.#.#######.###.#.#.#.#.#.##
##.#...#.#.....#.#...#...#...#.#.#.#...#.##
##..##.#.#.###.#.###.#.#.###.#.#.#.#####.##
##.....#.#...#...#...#.#.#.#...#.#.#...#.##
###..###.###.#####.###.#.#.#####.#.#.#.#.##
##...#.....#...#.......#.#.....#.#...#.#.##
##..##.###.###.#########.###.#.#.#######.##
##.#...#.#...#.....#...#.#...#.#...#...#.##
##.#.###.###.#####.##..#.#.#######.#.#.#.##
##.#.#.....#.....#.#...#.#.#.........#.#.##
####.#####.#####.#.#..##.#.#.#########.#.##
##...#...#...#...#.#.....#...#.....#.#.#.##
##.###.#.#.###.###.#####.#####.###.#.#.#.##
##.#...#.#.#.....#.#..O#.......#...#.#.#.##
##.#.###.#.#.#####.#.##.########.###.#.#.##
##.....#.#...#.#...#...#.#.......#.....#.##
######.#.#.###.#.#####.#.#.#######.#####.##
##...#.#.#.....#.......#.#.......#.#...#.##
##.#.#.#.###############.#######.#.#.#.#.##
##.#.#.#.#...............#.#.....#...#.#.##
##.#.###.#.###########.#.#.#.#########.#.##
##.#.....#.........#...#.#.#...#...#...#.##
##.###########.#####.###.#.###.#.###.###.##
##.#.......#...#.....#...#...#.#...#.#...##
##.#.###.#.#.###.#####.###.#.#.###.#.#.#.##
##.....#.#!#...#.#.....#...#.#...#.#...#.##
##.#####.#####.#.#.#####.#######.#.#####.##
##.#.#...#.....#.#.......#.....#.#...#...##
##.#.#.###.#####.#########.#.###.#.#.#.####
##...#...#...#.#.#...#...#.#.....#.#.#.#.##
####.###.###.#.#.#.#.#.#.#.#######.###.#.##
##.....#.......#...#...#...#.............##
###########################################
###########################################";

    let mut init_pos = (11111110, 11111110);
    let mut goal = (111110, 111110);

    let mut maze = Vec::new();

    let mut y = 0;
    for line in maze_str.lines() {
        let mut x = 0;
        let mut new_line = Vec::new();        
        for c in line.chars() {
            if c == '.' {
                new_line.push(0);
            } else if c == '#' {
                new_line.push(1);
            } else if c == 'O' {
                new_line.push(0);
                init_pos = (x, y);
            } else if c == '!' {
                new_line.push(0);
                goal = (x, y);
            } else {
                panic!("Unknown char! {}", c);
            }
            x += 1;
        }
        y += 1;
        maze.push(new_line);
    }
    let bool_maze = boolify_2d_maze(&0, &maze);
    (bool_maze, init_pos, goal)
}

#[aoc(day15, part2)]
fn find_solution2(input: &Vec<i64>) -> i64 {
    let (bool_maze, init_pos, goal) = parse_maze_input();
    let mut oxygened: HashMap<(usize, usize), bool> = HashMap::new();
    let mut count = 0;
    println!("{:?}", bool_maze);
    oxygened.insert(goal, true);
    loop {
        let mut new_oxygened: HashMap<(usize, usize), bool> = HashMap::new();
        for (pos, _) in &oxygened {
            new_oxygened.insert(*pos, true);
            let up = (pos.0, pos.1 - 1);
            let down = (pos.0, pos.1 + 1);
            let right = (pos.0 + 1, pos.1);
            let left = (pos.0 - 1, pos.1);
            println!("pos: {:?}, up: {:?}, down: {:?}, left: {:?}, right: {:?}", pos, up, down, left, right);
            if !bool_maze[up.1][up.0] {
                new_oxygened.insert(up, true);
            }
            if !bool_maze[down.1][down.0] {
                new_oxygened.insert(down, true);
            }
            if !bool_maze[right.1][right.0] {
                new_oxygened.insert(right, true);
            }
            if !bool_maze[left.1][left.0] {
                new_oxygened.insert(left, true);
            }
        }
        panic!("pg {:?}", new_oxygened);
        if oxygened.len() == new_oxygened.len() {
            break;
        }
        count += 1;
        oxygened = new_oxygened;
    }
    count
}

#[aoc(day15, part1)]
fn find_solution1(input: &Vec<i64>) -> i64 {
    //explore_maze(&input);

    let (bool_maze, init_pos, goal) = parse_maze_input();

    let allowed_moves = AllowedMoves2D {
        moves: vec![
            (0, 1),
            (0, -1),
            (-1, 0),
            (1, 0)
        ]
    };

    let road = maze_directions2d(&bool_maze, &allowed_moves, &init_pos, &goal).unwrap();
    road.len() as i64
}

fn explore_maze (input: &Vec<i64>) {
    lazy_static! { 
        static ref positions: Mutex<HashMap<(i64, i64), i64>> = Mutex::new(HashMap::new());
    }

    static mut pos: (i64, i64) = (0, 0);
    static mut direction: i64 = 1;
    static mut last_output: i64 = 1;
    static mut auto: bool = false;
    static mut steps: usize = 0;
    static mut oxygen_found: bool = false;
    static mut current_move: usize = 0;
    let mut program = input.clone();

    unsafe {
        let ig = || -> InputGenerator {
            Box::new(|| {
                //let moves: Vec<i64> = vec![2, 2, 0, 0, 3, 3, 0, 0, 2, 2, 2, 2, 2, 2, 1, 1, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 3, 3, 3, 3, 0, 0, 3, 3, 1, 1, 1, 1, 3, 3, 0, 0, 3, 3, 0, 0, 2, 2, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 3, 0, 3, 3, 3, 3, 3, 3, 1, 1, 3, 3, 3, 3, 0, 0, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 2, 2, 2, 2, 0, 0, 3, 3, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 2, 2, 0, 0, 0, 0, 2, 2, 1, 1, 2, 2, 0, 0, 2, 2, 1, 1, 2, 2, 0, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 1, 1, 3, 3, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 3, 3, 3, 3, 0, 0, 2, 2, 0, 0, 3, 3, 0, 0, 2, 2, 2, 2, 0, 0, 3, 3, 0, 0, 2, 2, 2, 1, 2, 1, 2, 2, 1, 1, 3, 3, 1, 1, 1, 1, 3, 3, 0, 0];
                //direction = moves[current_move] + 1;
                //current_move += 1;
                direction = if last_output == 1 {
                    if rand::random() {
                        direction
                    } else {
                        rand::thread_rng().gen_range(1, 5)
                    }
                } else {
                    rand::thread_rng().gen_range(1, 5)
                };

                if steps % 10000 == 0 {
                    let mut out = String::with_capacity(2000);
                    for y in pos.1-10..pos.1+10 {
                        for x in pos.0-20..pos.0+20 {
                            if pos == (x, y) {
                                out.push_str("D");
                            } else if (x, y) == (0, 0) {
                                out.push_str("O");
                            } else {
                                let tile = *positions.lock().unwrap().get(&(x, y)).unwrap_or(&1i64);
                                let s = match tile {
                                    0 => "#",
                                    1 => ".",
                                    2 => "!",
                                    _ => ""
                                };
                                out.push_str(s);    
                            }
                        }
                        out.push_str("\n");
                    }
                    println!("{}[2J{}\nOxygen found: {} Direction: {} Steps: {} Position: ({}, {})" , 27 as char, out, oxygen_found, direction, steps, pos.0, pos.1);    
                }
                direction
            })
        };
        
        let oh = || -> OutputHandler {
            Box::new(|o: i64| {
                steps += 1;
                if steps % 1000 == 0 {
                    let mut out = String::with_capacity(2000);
                    for y in pos.1 - 400..pos.1 + 400 {
                        for x in pos.0 - 400..pos.0 + 400 {
                            if pos == (x, y) {
                                out.push_str("D");
                            } else if (x, y) == (0, 0) {
                                out.push_str("O");
                            } else {
                                let tile = *positions.lock().unwrap().get(&(x, y)).unwrap_or(&1i64);
                                let s = match tile {
                                    0 => "#",
                                    1 => ".",
                                    2 => {
                                        oxygen_found = true;
                                        "!"
                                    },
                                    _ => ""
                                };
                                out.push_str(s);    
                            }
                        }
                        out.push_str("\n");
                    }
                    let mut f = File::create("C:\\Users\\Benoit\\output.txt").expect("Unable to create file");
                    f.write_all(out.as_bytes()).expect("Unable to write data");
                    //fs::write("data.json", out).expect("Unable to write file");
                }
                last_output = o;
                let new_pos = match direction {
                    1 => (pos.0, pos.1 - 1),
                    2 => (pos.0, pos.1 + 1),
                    3 => (pos.0 - 1, pos.1),
                    4 => (pos.0 + 1, pos.1),
                    _ => panic!("Unknown direction")
                };
                *positions.lock().unwrap().entry(new_pos).or_insert(0i64) = o;
                if o != 0 {
                    pos = new_pos;
                }
            })
        };
        run_intcode(&mut program.clone(), &ig(), &oh());     
    }
}

/*
#[aoc(day15, part2)]
fn find_solution2(input: &Vec<i128>) -> i128 {
    0
}
*/
