type Map = Vec<Vec<char>>;

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Map {    
    let mut map = Vec::new();
    for line in input.lines() {
        let mut line_vec = Vec::new();
        for c in line.chars() {
            line_vec.push(c);
        }
        map.push(line_vec);
    }
    map

}

#[aoc(day24, part1)]
fn find_solution1(input: &Map) -> usize {
    let mut maps: Vec<Map> = Vec::new();
    let mut map = input.clone();
    maps.push(map.clone());
    let mut iterations = 0;
    loop {
        iterations += 1;
        let new_map = find_next_state(&map);
        for m in &maps {
            if compare_maps(m, &new_map) {
                println!("ITERATIONS: {}", iterations);
                return calculate_biodiversity(m);
            }
        }
        maps.push(new_map.clone());
        map = new_map;
    }
    0
}

fn calculate_biodiversity(map: &Map) -> usize {
    println!("HERE'S THE MAP: {:?}", map);
    let mut value = 1;
    let mut count = 0;
    for line in map {
        for c in line {
            if *c == '#' {
                count += value;
            }
            value *= 2;
        }
    }
    count
}
fn find_next_state(state: &Map) -> Map {
    let mut new_map: Map = state.clone();
    
    for line_idx in 0..state.len() {
        let line = &state[line_idx];
        for c_idx in 0..line.len() {
            new_map[line_idx][c_idx] = find_next_state_of_char(line_idx, c_idx, &state);
        }
    }
    new_map.clone()
}

fn compare_maps(map1: &Map, map2: &Map) -> bool {
    for line_idx in 0..map1.len() {
        for c_idx in 0..map1[line_idx].len() {
            if map1[line_idx][c_idx] != map2[line_idx][c_idx] {
                return false;
            }
        }
    }
    true
}

fn find_next_state_of_char(line_idx: usize, c_idx: usize, state: &Map) -> char {
    let c_up = if line_idx > 0 {
        state[line_idx - 1][c_idx]
    } else {
        ' '
    };
    let c_down = if line_idx < state.len() - 1 {
        state[line_idx + 1][c_idx]
    } else {
        ' '
    };  
    let c_left = if c_idx > 0 {
        state[line_idx][c_idx - 1]
    } else {
        ' '
    };
    let c_right = if c_idx < state[line_idx].len() - 1 {
        state[line_idx][c_idx + 1]
    } else {
        ' '
    };
    let c = state[line_idx][c_idx];
    let mut count_bugs = 0;
    if c_up == '#' {
        count_bugs +=1;
    }
    if c_down == '#' {
        count_bugs += 1;
    }
    if c_left == '#' {
        count_bugs += 1;
    }
    if c_right == '#' {
        count_bugs += 1;
    }

    if c == '#' {
        if count_bugs == 1 {
            return '#';
        } else {
            return '.'
        }
    } else if c == '.' {
        if count_bugs == 1 || count_bugs == 2 {
            return '#';
        } else {
            return '.'
        }
    } else {
        panic!("Unknown char");
    }
}

#[test]
fn test_example() {
    let input = "....#
#..#.
#..##
..#..
#....";

    let map = parse_input(input);
    let new_map1 = find_next_state(&map);
    let after_1_minute = "#..#.
####.
###.#
##.##
.##..";
    let check1 = parse_input(after_1_minute);

    assert!(compare_maps(&check1, &new_map1));

    let new_map2 = find_next_state(&new_map1);
    let after_2_minutes = "#####
....#
....#
...#.
#.###";
    let check2 = parse_input(after_2_minutes);

    assert!(compare_maps(&check2, &new_map2));
}