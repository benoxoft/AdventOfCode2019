use leemaze::{AllowedMoves2D, boolify_2d_maze, maze_directions2d};
use std::collections::HashMap;
use std::sync::Mutex;
use threadpool::ThreadPool;

type Maze = Vec<Vec<bool>>;
type Doors = HashMap<char, (usize, usize)>;
type Keys = HashMap<(usize, usize), char>;
type Pos = (usize, usize);
type Moves = Mutex<HashMap<((usize, usize), (usize, usize)), (usize, Vec<char>)>>;

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

fn find_smallest_path(input: &str) -> usize {
    lazy_static! {
        static ref final_score: Mutex<usize> = Mutex::new(3274);
        static ref moves: Moves = Mutex::new(HashMap::new());
    }

    let (maze, init_pos, doors, keys) = parse_input(input);
    let allowed_moves = get_allowed_moves();

    let mut choices = Vec::new();

    for (pos, key) in keys.clone() {
        match maze_directions2d(&maze, &allowed_moves, &init_pos[0], &pos) {
            Some(road) => {
                choices.push((pos, key, road.len()));
            },
            None => continue

        }
    }
    let pool = ThreadPool::new(choices.len());

    for (pos, key, road_len) in choices {
        let mut path = Vec::new();
        let mut new_maze = maze.clone();
        let door_pos = &doors.get(&key).unwrap();
        new_maze[door_pos.1][door_pos.0] = !new_maze[door_pos.1][door_pos.0];
        path.push(key.clone());
        let new_keys = keys.clone();
        let new_doors = doors.clone();
        pool.execute(move || explore_maze(new_maze, pos.clone(), &new_doors, new_keys, road_len, &final_score, path, &moves));
    }
    pool.join();
    *final_score.lock().unwrap()
}

fn lookup_traveled_roads(current_pos: &Pos, pos: &Pos, moves: &Moves, path: &Vec<char>) -> Option<usize> {
    match moves.lock().unwrap().get(&(*current_pos, *pos)) {
        Some(mov) => {
            let needed_keys = &mov.1;
            for needed_key in needed_keys {
                if !path.contains(needed_key) {
                    return None;
                }
            }
            Some(mov.0)
        },
        None => None
    }
}

fn lookup_new_road(maze: &Maze, current_pos: &Pos, pos: &Pos) -> Option<usize> {
    let allowed_moves = get_allowed_moves();
    match maze_directions2d(&maze, &allowed_moves, &current_pos, &pos) {
        Some(road) => Some(road.len()),
        None => None
    }        
}

fn explore_maze(maze: Maze, current_pos: Pos, doors: &Doors, keys: Keys, score: usize, final_score: &Mutex<usize>, path: Vec<char>, moves: &Moves) {
    let mut choices = Vec::new();

    if *final_score.lock().unwrap() <= score {
        return;
    }

    for (pos, key) in keys.iter() {
        //println!("looking for key {}", key);
        if path.contains(key) {
            continue;
        }
        //let traveled = lookup_traveled_roads(&current_pos, pos, moves, &path);
        //match traveled {
          //  Some(road_len) => choices.push((pos, key, road_len)),
            //None => {
                match lookup_new_road(&maze, &current_pos, pos) {
                    Some(road_len) => {
                        if *final_score.lock().unwrap() <= score + road_len {
                            return;
                        }
                        choices.push((pos, key, road_len));
                    },
                    None => continue
                }
         //   }
        //}
    }

    choices.sort_by(|a, b| a.2.cmp(&b.2));

    for (pos, key, road_len) in choices {
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
        explore_maze(new_maze, *pos, doors, new_keys, score + road_len, final_score, new_path, moves);
    }

    if keys.len() == path.len() && score < *final_score.lock().unwrap() {
        println!("found possible score: {}", score);
        *final_score.lock().unwrap() = score;
    }
}