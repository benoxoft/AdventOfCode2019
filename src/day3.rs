use std::collections::HashMap;

type Moves = Vec<Position>;
type Operations = Vec<MoveClosure>;

type Position = (isize, isize);
type MoveClosure = Box<dyn Fn(Position) -> Moves>;
type TransformClosure = Box<dyn Fn((isize, isize))-> (isize, isize)>;
type Intersections = HashMap<(isize, isize), isize>;

fn move_cursor(amount: isize, transform: TransformClosure) -> MoveClosure {
    let actual_move = move |pos: Position| -> Moves {
        let mut ret = Vec::new();
        let mut new_pos = pos;
        for _ in 0..amount {
            new_pos = transform(new_pos);
            ret.push(new_pos);
        }
        ret
    };
    Box::new(actual_move)
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Operations> {
    fn generate_operation(token: &&str) -> MoveClosure {
        let chr = token.chars().nth(0).unwrap();
        
        let amount = &token[1..].parse::<isize>().unwrap();
        let operation = match chr {
            'R' => move_cursor(*amount, Box::new(|(x, y)| (x + 1, y))),
            'L' => move_cursor(*amount, Box::new(|(x, y)| (x - 1, y))),
            'U' => move_cursor(*amount, Box::new(|(x, y)| (x, y + 1))),
            'D' => move_cursor(*amount, Box::new(|(x, y)| (x, y - 1))),
            _ => panic!("Unknowm char!")
        };
        operation
    }

    let mut wires_ops: Vec<Operations> = Vec::new();
    let wires: Vec<&str> = input.lines().collect();
    for wire in wires {
        let tokens: Vec<&str> = wire.split(",").collect();
        let ret = Moves::new();
        let operations: Operations = tokens.iter().map(generate_operation).collect();
        wires_ops.push(operations);
    }
    wires_ops
}

#[aoc(day3, part1)]
fn find_solution1(wires: &Vec<Operations>) -> isize {
    let intersections = cross_wires(wires);
    let calc_dist = calculate_distance(intersections);
    calc_dist
}

fn cross_wires(wires: &Vec<Operations>) -> Intersections {
    let mut intersections: Intersections = HashMap::new();
    let mut initial_pos = (0, 0);
    *intersections.entry(initial_pos).or_insert(0) += 1;
    
    let ops = wires.get(0).unwrap();
    for op in ops {
        let positions = op(initial_pos);
        for pos in positions {
            *intersections.entry(pos).or_insert(0) = 1;
            initial_pos = pos;
        }
    }

    initial_pos = (0, 0);
    let ops = wires.get(1).unwrap();
    for op in ops {
        let positions = op(initial_pos);
        for pos in positions {
            *intersections.entry(pos).or_insert(0) += 1;
            initial_pos = pos;
        }
    }
    intersections
}

fn calculate_distance(intersections: Intersections) -> isize {
    let mut smallest = std::isize::MAX;
    
    for (key, value) in intersections {
        if value > 1 {
            let dist;
            dist = key.0.abs() + key.1.abs();
            if dist < smallest {
                smallest = dist;
            }
        }
    }
    smallest
}

#[test]
fn test1() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let distance = 159;
    let parsed = parse_input(input);
    let intersections = cross_wires(&parsed);
    let calc_dist = calculate_distance(intersections);
    assert_eq!(distance, calc_dist);
}

#[test]
fn test2() {
    let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    let distance = 135;
    let parsed = parse_input(input);
    let intersections = cross_wires(&parsed);
    let calc_dist = calculate_distance(intersections);
    assert_eq!(distance, calc_dist);
}

#[test]
fn test_move_down() {
    let m = move_cursor(4, Box::new(|(x, y)| (x - 1, y)));
    let result = m((12,13));
    assert_eq!(result[0], (11,13));
    assert_eq!(result[1], (10,13));
    assert_eq!(result[2], (9,13));
    assert_eq!(result[3], (8,13));
    assert_eq!(result.len(), 4);
}
