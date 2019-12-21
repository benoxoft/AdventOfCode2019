use leemaze::{AllowedMoves2D, boolify_2d_maze, maze_directions2d};
use std::collections::HashMap;
use std::sync::Mutex;
use threadpool::ThreadPool;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::{thread, time};
use rand::Rng;

type Maze = Vec<Vec<bool>>;
type Doors = HashMap<char, (usize, usize)>;
type Keys = HashMap<(usize, usize), char>;
type Pos = (usize, usize);

//#[aoc_generator(day18)]
fn parse_input(input: &str) -> (Maze, Vec<Pos>, Doors, Keys) {

    let mut maze = Vec::new();
    let mut doors = HashMap::new();
    let mut keys = HashMap::new();
    let mut init_pos = Vec::new();

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        let mut new_line = Vec::new();
        for c in line.chars() {
            if c == '.' {
                new_line.push(0);
            } else if c == '#' {
                new_line.push(1);
            } else if c.is_uppercase() {
                let lower_c = c.to_lowercase().collect::<Vec<_>>()[0];
                doors.insert(lower_c, (x, y));
                new_line.push(1);
            } else if c == '@' {
                init_pos.push((x, y));
                new_line.push(0);
            } else {
                keys.insert((x, y), c);
                new_line.push(0);
            }
            x += 1;
        }
        y += 1;
        maze.push(new_line);
    }
    let bool_maze = boolify_2d_maze(&0, &maze);
    (bool_maze, init_pos, doors, keys)
}

#[aoc(day18, part1)]
fn find_solution1(input: &str) -> usize {
    //test_example3();
    find_smallest_path(input)
    //find_smallest_steps(input);
    
}

/*
#[aoc(day18, part2)]
fn find_solution2(input: &Vec<usize>) -> usize {
    0
}
*/

fn get_allowed_moves() -> AllowedMoves2D {
    AllowedMoves2D {
        moves: vec![
            (0, 1),
            (0, -1),
            (-1, 0),
            (1, 0)
        ]
    }
}

#[test]
fn test_example1() {
    let input = "#########
#b.A.@.a#
#########";
    assert_eq!(find_smallest_path(input), 8);
}

#[test]
fn test_example2() {
    let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
    assert_eq!(find_smallest_path(input), 86);
}

//#[test]
fn test_example3() {
    let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
    assert_eq!(find_smallest_path(input), 132);

}

fn test_part2_example1() {
    let input = "#######
#@.#Cd#
##.#@##
#######
##@#@##
#cB#.b#
#######";


}

fn find_smallest_steps(input: &str) {
    lazy_static! {
        static ref final_score: Mutex<usize> = Mutex::new(0);
        static ref found_keys: Mutex<Vec<char>> = Mutex::new(Vec::new());
        static ref current_maze: Mutex<Vec<Vec<bool>>> = Mutex::new(Vec::new());
    }

    let (maze, init_pos, doors, keys) = parse_input(input);
    *current_maze.lock().unwrap() = maze.clone();

    let pool = ThreadPool::new(init_pos.len());
    let mut rng = rand::thread_rng();

    let mut smallest = 1697;
    loop {
        let mut new_init_pos = init_pos.clone();
        new_init_pos.shuffle(&mut thread_rng());
        for robot in new_init_pos.clone() {
            let new_keys = keys.clone();
            let new_doors = doors.clone();
            pool.execute(move || explore_maze2(&current_maze, robot.clone(), &new_doors, new_keys, &final_score, &found_keys));
            let ten_millis = time::Duration::from_millis(rng.gen_range(0, 3));
            thread::sleep(ten_millis);

        }
        pool.join();
        if *final_score.lock().unwrap() < smallest {
            smallest = *final_score.lock().unwrap();
            println!("current smallest value: {}", smallest);
        }
        *final_score.lock().unwrap() = 0;
        found_keys.lock().unwrap().clear();
        *current_maze.lock().unwrap() = maze.clone();
    }
}

fn explore_maze2(maze: &Mutex<Maze>, mut current_pos: Pos, doors: &Doors, keys: Keys, final_score: &Mutex<usize>, found_keys: &Mutex<Vec<char>>) {
    let mut rng = rand::thread_rng();
    loop {
        for (pos, key) in keys.iter() {
            let ten_millis = time::Duration::from_millis(rng.gen_range(0, 3));
            thread::sleep(ten_millis);
            if found_keys.lock().unwrap().contains(key) {
                continue;
            }
            let road = lookup_new_road(&(*maze.lock().unwrap()), &current_pos, pos);
            match road {
                Some(road_len) => {
                    match doors.get(&key) {
                        Some(door_pos) => {
                            let mut guarded_maze = maze.lock().unwrap();
                            guarded_maze[door_pos.1][door_pos.0] = !guarded_maze[door_pos.1][door_pos.0];
                            drop(guarded_maze);
                            found_keys.lock().unwrap().push(*key);
                            current_pos = pos.clone();
                            *final_score.lock().unwrap() += road_len;
                            let ten_millis = time::Duration::from_millis(rng.gen_range(0, 3));
                            thread::sleep(ten_millis);                
                        }
                        None => ()
                    }
                },
                None => continue
            }
        }
    
        if found_keys.lock().unwrap().len() == keys.len() {
            return;
        }    
    }
}

fn find_smallest_path(input: &str) -> usize {
    lazy_static! {
        static ref final_score: Mutex<usize> = Mutex::new(1697);
        static ref found_keys: Mutex<Vec<char>> = Mutex::new(Vec::new());
    }

    let (maze, init_pos, doors, keys) = parse_input(input);
    let allowed_moves = get_allowed_moves();

    let mut choices = Vec::new();

    for (pos, key) in keys.clone() {
        for i in 0..init_pos.len() {
            match maze_directions2d(&maze, &allowed_moves, &init_pos[i], &pos) {
                Some(road) => {
                    let mut new_init_pos = init_pos.clone();
                    new_init_pos[i] = pos;
                    choices.push((new_init_pos, key, road.len()));
                },
                None => continue
    
            }    
        }
    }
    let pool = ThreadPool::new(choices.len());

    for (new_init_pos, key, road_len) in choices {
        let mut path = Vec::new();
        let mut new_maze = maze.clone();
        let door_pos = &doors.get(&key).unwrap();
        new_maze[door_pos.1][door_pos.0] = !new_maze[door_pos.1][door_pos.0];
        path.push(key.clone());
        let new_keys = keys.clone();
        let new_doors = doors.clone();
        pool.execute(move || explore_maze(new_maze, new_init_pos, &new_doors, new_keys, road_len, &final_score, path, &found_keys));
    }
    pool.join();
    *final_score.lock().unwrap()
}

fn lookup_new_road(maze: &Maze, current_pos: &Pos, pos: &Pos) -> Option<usize> {
    let allowed_moves = get_allowed_moves();
    match maze_directions2d(&maze, &allowed_moves, &current_pos, &pos) {
        Some(road) => Some(road.len()),
        None => None
    }        
}

fn explore_maze(maze: Maze, current_positions: Vec<Pos>, doors: &Doors, keys: Keys, score: usize, final_score: &Mutex<usize>, path: Vec<char>, found_keys: &Mutex<Vec<char>>) {
    let mut choices = Vec::new();

    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);

    if *final_score.lock().unwrap() <= score {
        return;
    }

    for i in 0..current_positions.len() {
        let current_pos = current_positions[i];
        for (pos, key) in keys.iter() {
            //println!("looking for key {}", key);
            if path.contains(key) {
                continue;
            }
            match lookup_new_road(&maze, &current_pos, pos) {
                Some(road_len) => {
                    if *final_score.lock().unwrap() <= score + road_len {
                        continue;
                    }
                    let mut new_init_pos = current_positions.clone();
                    new_init_pos[i] = *pos;
                    choices.push((new_init_pos, key, road_len));
                },
                None => continue
            }
        }    
    }

    //choices.sort_by(|a, b| a.2.cmp(&b.2));
    choices.shuffle(&mut thread_rng());

    for (new_init_pos, key, road_len) in choices {
        let mut new_maze = maze.clone();
        match doors.get(&key) {
            Some(door_pos) => {
                new_maze[door_pos.1][door_pos.0] = !new_maze[door_pos.1][door_pos.0];
            }
            None => ()
        }
        let mut new_path = path.clone();                
        new_path.push(*key);
        let new_keys = keys.clone();
        explore_maze(new_maze, new_init_pos, doors, new_keys, score + road_len, final_score, new_path, found_keys);
    }

    if keys.len() == path.len() && score < *final_score.lock().unwrap() {
        println!("found possible score: {}", score);
        *final_score.lock().unwrap() = score;
    }
}
