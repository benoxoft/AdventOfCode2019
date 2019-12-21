use std::collections::HashMap;
use leemaze::{AllowedMoves2D, boolify_2d_maze, maze_directions2d};

type Map = Vec<Vec<bool>>;
type WarpZones = HashMap<String, ((usize, usize), bool, (usize, usize), bool)>;
type Position = (usize, usize);

fn parse_input(input: &str) -> (Map, WarpZones, Position, Position) {
    let lines: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut warps = HashMap::new();
    let mut init_pos = (0, 0);
    let mut destination = (0, 0);

    for line_idx in 0..lines.len() {
        let mut map_line = Vec::new();
        let line = lines[line_idx];
        for c_idx in 0..line.len() {
            let c = line.chars().nth(c_idx).unwrap();
            match c {
                '#' => {
                    map_line.push(1);
                },
                '.' => {
                    map_line.push(0);
                },
                ' ' => {
                    map_line.push(1);
                },
                _ => {
                    if c == 'A' && line.chars().nth(c_idx - 1).unwrap() == 'A' && line.chars().nth(c_idx - 2).unwrap() == '.' {
                        init_pos = (c_idx - 2, line_idx);
                    }
                    if line_idx + 2 < lines.len() &&  c == 'A' && lines[line_idx + 1].chars().nth(c_idx).unwrap() == 'A' && lines[line_idx + 2].chars().nth(c_idx).unwrap() == '.' {
                        init_pos = (c_idx, line_idx + 2);
                    }
                    if line_idx > 2 &&  c == 'A' && lines[line_idx - 1].chars().nth(c_idx).unwrap() == 'A' && lines[line_idx - 2].chars().nth(c_idx).unwrap() == '.' {
                        init_pos = (c_idx, line_idx - 2);
                    }
                    if line_idx > 2 && c == 'Z' && lines[line_idx - 1].chars().nth(c_idx).unwrap() == 'Z' && lines[line_idx - 2].chars().nth(c_idx).unwrap() == '.' {
                        destination = (c_idx, line_idx - 2);
                    }
                    if c_idx + 2 < line.len() && line.chars().nth(c_idx + 1).unwrap() == 'Z' && line.chars().nth(c_idx + 2).unwrap() == '.' {
                        destination = (c_idx + 2, line_idx);
                    }
                    if line_idx + 2 < lines.len() &&  c == 'Z' && lines[line_idx + 1].chars().nth(c_idx).unwrap() == 'Z' && lines[line_idx + 2].chars().nth(c_idx).unwrap() == '.' {
                        destination = (c_idx, line_idx + 2);
                    }

                    map_line.push(1);
                    let warp_info = if c_idx + 2 < line.len() && line.chars().nth(c_idx + 1).unwrap().is_alphabetic() && line.chars().nth(c_idx + 2).unwrap() == '.' {
                        let mut warp_name = String::new();
                        warp_name.push(c);
                        warp_name.push(line.chars().nth(c_idx + 1).unwrap());
                        let outer = c_idx < line.len() / 2;
                        Some((warp_name, (c_idx + 2, line_idx), outer))
                    } else if c_idx > 2 && line.chars().nth(c_idx -1).unwrap().is_alphabetic() && line.chars().nth(c_idx - 2).unwrap() == '.' {
                        let mut warp_name = String::new();
                        warp_name.push(line.chars().nth(c_idx - 1).unwrap());
                        warp_name.push(c);
                        let outer = c_idx > line.len() / 2;
                        Some((warp_name, (c_idx - 2, line_idx), outer))
                    } else if line_idx > 2 && lines[line_idx - 1].chars().nth(c_idx).unwrap().is_alphabetic() && lines[line_idx - 2].chars().nth(c_idx).unwrap() == '.' {
                        let mut warp_name = String::new();
                        warp_name.push(lines[line_idx - 1].chars().nth(c_idx).unwrap());
                        warp_name.push(c);
                        let outer = line_idx > lines.len() / 2;
                        Some((warp_name, (c_idx, line_idx - 2), outer))
                    } else if line_idx + 2 < lines.len() && lines[line_idx + 1].chars().nth(c_idx).unwrap().is_alphabetic() && lines[line_idx + 2].chars().nth(c_idx).unwrap() == '.' {
                        let mut warp_name = String::new();
                        warp_name.push(c);
                        warp_name.push(lines[line_idx + 1].chars().nth(c_idx).unwrap());
                        let outer = line_idx < lines.len() / 2;
                        Some((warp_name, (c_idx, line_idx + 2), outer))
                    } else {
                        None
                    };

                    match warp_info {
                        Some((warp, pos, outer)) => {
                            if warp == "AA" || warp == "ZZ" {
                                continue;
                            }
                            if !warps.contains_key(&warp) {
                                warps.insert(warp, (pos, outer, (0, 0), false));
                            } else {
                                let other_pos = warps.get(&warp).unwrap().0;
                                let other_outer = warps.get(&warp).unwrap().1;
                                *warps.get_mut(&warp).unwrap() = (other_pos, other_outer, pos, outer);
                            }
                        },
                        None => ()
                    }
                }
            }
        }
        map.push(map_line);
    }

    assert_ne!(init_pos, (0, 0));
    assert_ne!(destination, (0, 0));

    let bool_maze = boolify_2d_maze(&0, &map);
    (bool_maze, warps, init_pos, destination)
}

#[aoc(day20, part1)]
fn find_solution1(input: &str) -> usize {
    let (map, warps, init_pos, destination) = parse_input(input);
    let mut final_size = std::u32::MAX as usize;

    explore_road(&map, &warps, init_pos, destination, 0, &mut final_size, Vec::new(), 0);
    println!("{}", final_size);

    0
}
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
    let input = "         A           \n         A           \n  #######.#########  \n  #######.........#  \n  #######.#######.#  \n  #######.#######.#  \n  #######.#######.#  \n  #####  B    ###.#  \nBC...##  C    ###.#  \n  ##.##       ###.#  \n  ##...DE  F  ###.#  \n  #####    G  ###.#  \n  #########.#####.#  \nDE..#######...###.#  \n  #.#########.###.#  \nFG..#########.....#  \n  ###########.#####  \n             Z       \n             Z       ";
    let (map, warps, init_pos, destination) = parse_input(input);
    let mut final_size = std::u32::MAX as usize;

    println!("{:?} AA {:?} ZZ {:?}", warps, init_pos, destination);
    explore_road(&map, &warps, init_pos, destination, 0, &mut final_size, Vec::new(), 0);
    assert_eq!(26, final_size);
}

#[test]
fn test_example2() {
let input = "#             Z L X W       C                 
#             Z P Q B       K                 
#  ###########.#.#.#.#######.###############  
#  #...#.......#.#.......#.#.......#.#.#...#  
#  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
#  #.#...#.#.#...#.#.#...#...#...#.#.......#  
#  #.###.#######.###.###.#.###.###.#.#######  
#  #...#.......#.#...#...#.............#...#  
#  #.#########.#######.#.#######.#######.###  
#  #...#.#    F       R I       Z    #.#.#.#  
#  #.###.#    D       E C       H    #.#.#.#  
#  #.#...#                           #...#.#  
#  #.###.#                           #.###.#  
#  #.#....OA                       WB..#.#..ZH
#  #.###.#                           #.#.#.#  
#CJ......#                           #.....#  
#  #######                           #######  
#  #.#....CK                         #......IC
#  #.###.#                           #.###.#  
#  #.....#                           #...#.#  
#  ###.###                           #.#.#.#  
#XF....#.#                         RF..#.#.#  
#  #####.#                           #######  
#  #......CJ                       NM..#...#  
#  ###.#.#                           #.###.#  
#RE....#.#                           #......RF
#  ###.###        X   X       L      #.#.#.#  
#  #.....#        F   Q       P      #.#.#.#  
#  ###.###########.###.#######.#########.###  
#  #.....#...#.....#.......#...#.....#.#...#  
#  #####.#.###.#######.#######.###.###.#.#.#  
#  #.......#.......#.#.#.#.#...#...#...#.#.#  
#  #####.###.#####.#.#.#.#.###.###.#.###.###  
#  #.......#.....#.#...#...............#...#  
#  #############.#.#.###.###################  
#               A O F   N                     
#               A A D   M                     ";


    let (map, warps, init_pos, destination) = parse_input(input);
    let mut final_size = std::u32::MAX as usize;

    explore_road(&map, &warps, init_pos, destination, 0, &mut final_size, Vec::new(), 0);
    assert_eq!(396, final_size);
    
}

fn explore_road(map: &Map, 
                warps: &WarpZones, 
                current_pos: Position, 
                final_destination: Position, 
                score: usize, 
                final_score: &mut usize,
                explored_warps: Vec<(usize, usize)>,
                current_level: usize) {

    // println!("explored roads: {:?} current score: {}", explored_warps, score);

    let allowed_moves = get_allowed_moves();

    for (name, (pos1, outer1, pos2, outer2)) in warps {
        //println!("LOOPING {} {:?} {} {:?} {}", name, pos1, outer1, pos2, outer2);
        if !explored_warps.contains(pos1) && !(current_level == 0 && *outer1) {
            let road1 = match maze_directions2d(&map, &allowed_moves, &current_pos, &pos1) {
                Some(road) => Some(road.len()),
                None => None
            };
            match road1 {
                Some(road_len) => {
                    //println!("found a road to {:?}", pos1);
                    let new_init_pos = pos2.clone();
                    let mut new_explored_warps = explored_warps.clone();
                    //new_explored_warps.push(pos1.clone());
                    let new_level = if *outer1 {
                        current_level - 1
                    } else {
                        current_level + 1
                    };
                    explore_road(map, warps, new_init_pos, final_destination, score + road_len + 1, final_score, new_explored_warps, new_level);
                },
                None => {
                    //println!("could not find a road to {:?}", pos1);
                }
            }    
        } else {
            //println!("Will not explore {:?}", pos1);
        }
        
        if !explored_warps.contains(pos2) && !(current_level == 0 && *outer2) {
            let road2 = match maze_directions2d(&map, &allowed_moves, &current_pos, &pos2) {
                Some(road) => Some(road.len()),
                None => None
            };
            match road2 {
                Some(road_len) => {
                    //println!("found a road to {:?}", pos2);
                    let new_init_pos = pos1.clone();
                    let mut new_explored_warps = explored_warps.clone();
                    //new_explored_warps.push(pos2.clone());
                    let new_level = if *outer2 {
                        current_level - 1
                    } else {
                        current_level + 1
                    };
                    explore_road(map, warps, new_init_pos, final_destination, score + road_len + 1, final_score, new_explored_warps, new_level);
                },
                None => {
                    //println!("could not find a road to {:?}", pos2);
                }
            }
        } else {
            //println!("Will not explore {:?}", pos2);
        }

        if current_level == 0 {
            let final_road = match maze_directions2d(&map, &allowed_moves, &current_pos, &final_destination) {
                Some(road) => Some(road.len()),
                None => None
            };
            match final_road {
                Some(road_len) => {
                    if score + road_len < *final_score {
                        *final_score = score + road_len;
                        println!("Found new shorter way: {}", final_score);
                    } else if score + road_len > *final_score {
                        //println!("Busting final score, going back");
                        return;
                    }
                },
                None => ()
            }    
        }
    
    }    

}
