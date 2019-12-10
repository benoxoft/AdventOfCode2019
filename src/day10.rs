use gcd::Gcd;
use std::f64;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
enum PositionType {
    Asteroid = 0,
    Empty = 1
}

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
struct Position {
    type_of_position: PositionType,
    x: i32, 
    y: i32
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Position> {
    let mut ret: Vec<Position> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    for y in 0..lines.len() {
        let line = lines[y];
        for x in 0..line.len() {
            let c = line.chars().nth(x).unwrap();
            if c == '#' {
                ret.push(Position {
                    type_of_position: PositionType::Asteroid,
                    x: x as i32,
                    y: y as i32
                });
            } else if c == '.' {
                ret.push(Position {
                    type_of_position: PositionType::Empty,
                    x: x as i32,
                    y: y as i32
                });
            } else {
                panic!("Unknown character");
            }
        }
    }
    ret
}

#[aoc(day10, part1)]
fn find_solution1(input: &Vec<Position>) -> usize {
    find_best_asteroid(&input).0
}

#[aoc(day10, part2)]
fn find_solution2(input: &Vec<Position>) -> usize {
    let pos = find_200th_vapourized_asteroids(&input);
    (pos.x * 100 + pos.y) as usize
}

fn find_200th_vapourized_asteroids(asteroids: &Vec<Position>) -> Position {
    let (_, location) = find_best_asteroid(&asteroids);
    let mut mod_asteroids = asteroids.clone();
    let mut count = 0;

    let calc_angle = |a: Position| -> f64 {
        let dy_a: f64 = f64::from(a.y - location.y);
        let dx_a: f64 = f64::from(a.x - location.x);
        let mut theta_a = dx_a.atan2(dy_a);
        theta_a *= f64::from(180) / std::f64::consts::PI;
        theta_a
    };

    let obfs = find_all_obfuscated(location, &mod_asteroids);
    let mut visible = find_all_visible_asteroids(location, &mod_asteroids, obfs);

    visible.sort_by(|a, b| {
        let theta_a = calc_angle(*a);
        let theta_b = calc_angle(*b);
        theta_b.partial_cmp(&theta_a).unwrap()
    });

    for ast in visible {
        for target_idx in 0..mod_asteroids.len() {
            let target = mod_asteroids[target_idx];
            if target == ast {
                println!("VAPOURIZING {:?}", target);
                mod_asteroids[target_idx] = Position {x: target.x, y: target.y, type_of_position: PositionType::Empty};
                count += 1;
                if count == 200 {
                    return target;
                }
                break;
            }
        }
    }
    panic!("Could not find 200th asteroid!");
}

fn find_best_asteroid(asteroids: &Vec<Position>) -> (usize, Position) {
    let mut best = 0;
    let mut location: Position = Position {x: 0, y: 0, type_of_position: PositionType::Empty};
    
    for ast in asteroids {
        if ast.type_of_position == PositionType::Empty {
            continue;
        }
        let obfs = find_all_obfuscated(*ast, &asteroids);
        let visible = find_all_visible_asteroids(*ast, asteroids, obfs).len();
        if visible > best {
            best = visible;
            location = *ast;
        }
    }
    (best, location)
}

fn find_all_obfuscated(ast: Position, asteroids: &Vec<Position>) -> Vec<Position> {
    let mut modified_asteroids = asteroids.clone();
    let mut obfs_asts = Vec::new();

    for compare_ast_ref in asteroids {
        let compare_ast = *compare_ast_ref;
        if ast == compare_ast {
            continue;
        }
        if compare_ast.type_of_position == PositionType::Empty {
            continue;
        }
        for obfuscated_ast in find_all_obfuscated_by_position(ast, compare_ast, &asteroids) {
            match find_current_position(obfuscated_ast.x, obfuscated_ast.y, &obfs_asts) {
                None => obfs_asts.push(obfuscated_ast),
                Some(_) => ()
            }
        }
    }
    obfs_asts
}

fn find_all_visible_asteroids(ast: Position, asteroids: &Vec<Position>, obfs_asts: Vec<Position>) -> Vec<Position> {
    //println!("{:?}", obfs_asts);
    let mut asts = Vec::new();
    for comp_ast in asteroids {
        if comp_ast.type_of_position == PositionType::Empty {
            continue;
        }
        if *comp_ast == ast {
            continue;
        }
        if find_current_position(comp_ast.x, comp_ast.y, &obfs_asts) == None {
            asts.push(*comp_ast);
        }
    }
    asts
}

fn find_all_obfuscated_by_position(base_ast: Position, tested_ast: Position, asteroids: &Vec<Position>) -> Vec<Position> {
    let steps_x = (tested_ast.x - base_ast.x) as i32;
    let steps_y = (tested_ast.y - base_ast.y) as i32;
    let gcd = (i32::abs(steps_x) as u32).gcd(i32::abs(steps_y) as u32) as i32;
    let steps_x = steps_x / gcd;
    let steps_y = steps_y / gcd;
    let mut obfuscated = Vec::new();

    let mut current_x = tested_ast.x;
    let mut current_y = tested_ast.y;
    loop {
        current_x += steps_x;
        current_y += steps_y;
        // println!("TESTING {} {} WITH {:?}", current_x, current_y, tested_ast);
        let tested_position = find_current_position(current_x, current_y, asteroids);
        if tested_position == None {
            return obfuscated;
        } else {
            obfuscated.push(*tested_position.unwrap());
        }
    }
}

fn find_current_position(x: i32, y: i32, asteroids: &Vec<Position>) -> Option<&Position> {
    for ast in asteroids {
        if ast.x == x && ast.y == y {
            return Some(ast);
        }
    }
    None
}

#[test]
fn test_find_200() {
let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
    let asteroids = parse_input(input);
    let pos = find_200th_vapourized_asteroids(&asteroids);
    assert_eq!(802, pos.x * 100 + pos.y);
}

#[test]
fn test_find_all_visible() {
    let input = ".#..#\n.....\n#####\n....#\n...##";
    let asteroids = parse_input(input);
    assert_eq!(asteroids[4].type_of_position, PositionType::Asteroid);
    println!("{:?}", asteroids);
    let ast = *find_current_position(3, 4, &asteroids).unwrap();
    let obfs = find_all_obfuscated(ast, &asteroids);
    let result = find_all_visible_asteroids(ast, &asteroids, obfs).len();
    assert_eq!(result, 8);
}

#[test]
fn test_find_all_obfuscated() {
    let input = ".#..#\n.....\n#####\n....#\n...##";
    let asteroids = parse_input(input);
    let result = find_all_obfuscated_by_position(*find_current_position(3, 4, &asteroids).unwrap(), 
                                     *find_current_position(2, 2, &asteroids).unwrap(), 
                                     &asteroids);
    assert_eq!(result.len(), 1);
    assert_eq!(&result[0], find_current_position(1, 0, &asteroids).unwrap());
}

#[test]
fn test_find_best() {
    let input = ".#..#\n.....\n#####\n....#\n...##";
    let asteroids = parse_input(input);
    assert_eq!(find_best_asteroid(&asteroids).0, 8);
}

#[test]
fn test_find_best2() {
let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
let asteroids = parse_input(input);
assert_eq!(find_best_asteroid(&asteroids).0, 210);

}

#[test]
fn test_find_current_position() {
    let input = ".#.\n...";
    let parsed = parse_input(input);
    let ast = find_current_position(1, 0, &parsed);
    assert_eq!(ast.unwrap(), &parsed[1]);
    assert_eq!(None, find_current_position(111, 111, &parsed));
}

#[test]
fn test_parse() {
    let input = "##\n#.\n..";
    let parsed = parse_input(input);
    assert_eq!(parsed[0], Position {
        type_of_position: PositionType::Asteroid,
        x: 0,
        y: 0
    });
    assert_eq!(parsed[1], Position {
        type_of_position: PositionType::Asteroid,
        x: 1,
        y: 0
    });
    assert_eq!(parsed[2], Position {
        type_of_position: PositionType::Asteroid,
        x: 0,
        y: 1
    });
    assert_eq!(parsed[3], Position {
        type_of_position: PositionType::Empty,
        x: 1,
        y: 1
    });
    assert_eq!(parsed[4], Position {
        type_of_position: PositionType::Empty,
        x: 0,
        y: 2
    });
    assert_eq!(parsed[5], Position {
        type_of_position: PositionType::Empty,
        x: 1,
        y: 2
    });

}
