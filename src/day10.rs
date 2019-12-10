use gcd::Gcd;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
enum PositionType {
    Asteroid = 0,
    Empty = 1
}

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
struct Position {
    type_of_position: PositionType,
    x: isize, 
    y: isize
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
                    x: x as isize,
                    y: y as isize
                });
            } else if c == '.' {
                ret.push(Position {
                    type_of_position: PositionType::Empty,
                    x: x as isize,
                    y: y as isize
                });
            } else {
                panic!("Unknown character");
            }
        }
    }
    ret
}

fn find_best_asteroid(asteroids: &Vec<Position>) {
    for ast in asteroids {
        if ast.type_of_position == PositionType::Empty {
            continue;
        }
        find_all_visible(*ast, asteroids);
    }
}

fn find_all_visible(ast: Position, asteroids: &Vec<Position>) -> usize {
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
        for obfuscated_ast in find_all_obfuscated(ast, compare_ast, &asteroids) {
            match find_current_position(obfuscated_ast.x, obfuscated_ast.y, &obfs_asts) {
                None => obfs_asts.push(obfuscated_ast),
                Some(_) => ()
            }
        }
    }
    println!("{:?}", obfs_asts);
    let mut count_asts = 0;
    for comp_ast in asteroids {
        if comp_ast.type_of_position == PositionType::Empty {
            continue;
        }
        if *comp_ast == ast {
            continue;
        }
        if find_current_position(comp_ast.x, comp_ast.y, &obfs_asts) == None {
            count_asts += 1;
        }
    }
    count_asts
}

#[test]
fn test_find_all_visible() {
    let input = ".#..#\n.....\n#####\n....#\n...##";
    let asteroids = parse_input(input);
    assert_eq!(asteroids[4].type_of_position, PositionType::Asteroid);
    println!("{:?}", asteroids);
    let result = find_all_visible(*find_current_position(3, 4, &asteroids).unwrap(), &asteroids);
    assert_eq!(result, 8);
}

fn find_all_obfuscated(base_ast: Position, tested_ast: Position, asteroids: &Vec<Position>) -> Vec<Position> {
    let steps_x = (tested_ast.x - base_ast.x) as isize;
    let steps_y = (tested_ast.y - base_ast.y) as isize;
    let gcd = (isize::abs(steps_x) as usize).gcd(isize::abs(steps_y) as usize) as isize;
    let steps_x = steps_x / gcd;
    let steps_y = steps_y / gcd;
    let mut obfuscated = Vec::new();

    let mut current_x = tested_ast.x;
    let mut current_y = tested_ast.y;
    loop {
        current_x += steps_x;
        current_y += steps_y;
        println!("TESTING {} {} WITH {:?}", current_x, current_y, tested_ast);
        let tested_position = find_current_position(current_x, current_y, asteroids);
        if tested_position == None {
            return obfuscated;
        } else {
            obfuscated.push(*tested_position.unwrap());
        }
    }
}

#[test]
fn test_find_all_obfuscated() {
    let input = ".#..#\n.....\n#####\n....#\n...##";
    let asteroids = parse_input(input);
    let result = find_all_obfuscated(*find_current_position(3, 4, &asteroids).unwrap(), 
                                     *find_current_position(2, 2, &asteroids).unwrap(), 
                                     &asteroids);
    assert_eq!(result.len(), 1);
    assert_eq!(&result[0], find_current_position(1, 0, &asteroids).unwrap());
}

fn find_current_position(x: isize, y: isize, asteroids: &Vec<Position>) -> Option<&Position> {
    for ast in asteroids {
        if ast.x == x && ast.y == y {
            return Some(ast);
        }
    }
    None
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

#[aoc(day10, part1)]
fn find_solution1(input: &Vec<Position>) -> usize {
    0
}

/*
#[aoc(day10, part2)]
fn find_solution2(input: &Vec<usize>) -> usize {
    0
}
*/