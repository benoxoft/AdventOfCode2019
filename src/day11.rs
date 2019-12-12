use std::collections::HashMap;
use crate::intcode::run_intcode;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<i128> {    
    let ret = input.split(",").map(|token| token.parse::<i128>().unwrap()).collect();
    ret
}

#[aoc(day11, part1)]
fn find_solution1(input: &Vec<i128>) -> usize {
    let output = run_program(&mut input.clone(), 0);
    output.len()
}


/*#[aoc(day11, part2)]
fn find_solution2(input: &Vec<i128>) -> usize {
    let output = run_program(&mut input.clone(), 1);
    for i in -100i128..100i128 {
        for j in -100i128..100i128 {
            let color = *output.get(&(i, j)).unwrap_or(&0i128);
            if color == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    0
}*/

fn run_program(input: &mut Vec<i128>, mut prog_input: i128) -> HashMap<(i128, i128), i128> {
    let mut relative_base = 0i128;
    let mut i = 0;
    let mut output:Vec<i128> = Vec::new();
    let mut input_consumed = false;
    let mut reached99 = false;
    let mut positions: HashMap<(i128, i128), i128> = HashMap::new();
    positions.insert((0i128, 0i128), prog_input);
    let mut posx = 0i128;
    let mut posy = 0i128;
    let mut dirx = 0i128;
    let mut diry = 1i128;

    loop {

        let strop = (input[i] + 100_000).to_string();
        let mode3 = strop.chars().nth(1).unwrap(); // 10 000
        let mode2 = strop.chars().nth(2).unwrap(); //  1 000
        let mode1 = strop.chars().nth(3).unwrap(); //    100
        let opcode = strop[4..6].parse::<i128>().unwrap() as i128;

        if opcode == 99 {
            println!("OPCODE 99 REACHED {}", positions.len());
            reached99 = true;
            break;
        }

        let val1;
        let val2;
        let pos1: i128 = input[i + 1];
        let pos2: i128 = input[i + 2];
        let pos3;
        let store;

        // println!("{:?}", strop);

        if mode1 == '0' {
            if pos1 > input.len() as i128 {
                input.resize((pos1 + 2) as usize, 0);
            }    
            val1 = input[pos1 as usize];
        } else if mode1 == '1' {
            val1 = pos1;
        } else if mode1 == '2' {
            let idx = pos1 + relative_base;
            if idx < 0 {
                panic!("Unexpected index lesser than 0");
            }
            if idx > input.len() as i128 {
                input.resize((idx + 2) as usize, 0);
            }    
            val1 = input[idx as usize];
        } else {
            panic!("Cannot comprehend mode");
        }

        if mode2 == '0' {
            if pos2 > input.len() as i128 {
                input.resize((pos2 + 2) as usize, 0);
            }    
            val2 = input[pos2 as usize];
        } else if mode2 == '1' {
            val2 = pos2;
        } else if mode2 == '2' {
            let idx = pos2 + relative_base;
            if idx < 0 {
                panic!("Unexpected index lesser than 0");
            }
            if idx > input.len() as i128 {
                input.resize((idx + 2) as usize, 0);
            }    
            val2 = input[idx as usize];
        } else {
            panic!("Cannot comprehend mode");
        }

        if i + 3 < input.len() {
            pos3 = input[i + 3] as i128;
        } else {
            pos3 = std::i128::MAX            
        }
        if mode3 == '2' {
            store = pos3 + relative_base;
        } else {
            store = pos3;
        }        

        if opcode == 3 {
            println!("Opcode: {} dirx: {} diry: {} posx: {} posy: {} positions: {:?}", 
            input[i], dirx, diry, posx, posy, positions);
            if positions.len() > 6 {
                panic!();
            }
    
            if input_consumed {
                assert_eq!(output.len(), 2);
                *positions.entry((posx, posy)).or_insert(0i128) = output[0];

                if output[1] == 0 { // left
                    if dirx == 1 && diry == 0 {
                        dirx = 0;
                        diry = 1;
                    } else if dirx == 0 && diry == 1 {
                        dirx = -1;
                        diry = 0;
                    } else if dirx == -1 && diry == 0 {
                        dirx = 0;
                        diry = -1;
                    } else if dirx == 0 && diry == -1 {
                        dirx = 1;
                        diry = 0;
                    } else {
                        panic!("Unknown direction!");
                    }
                } else if output[1] == 1 { // right
                    if dirx == 1 && diry == 0 {
                        dirx = 0;
                        diry = -1;
                    } else if dirx == 0 && diry == 1 {
                        dirx = 1;
                        diry = 0;
                    } else if dirx == -1 && diry == 0 {
                        dirx = 0;
                        diry = 1;
                    } else if dirx == 0 && diry == -1 {
                        dirx = -1;
                        diry = 0;
                    } else {
                        panic!("Unknown direction!");
                    }
                } else {
                    panic!("Unknown direction!");
                }
                posx += dirx;
                posy += diry;
                output.clear();                
            }
            input_consumed = true;
            //println!("USES INPUT {} AS INPUT", prog_input);
            if mode1 == '2' {
                input[(pos1 + relative_base) as usize] = *positions.get(&(posx, posy)).unwrap_or(&0i128);
            } else if mode1 == '0' {
                input[pos1 as usize] = *positions.get(&(posx, posy)).unwrap_or(&0i128);
            } else {
                panic!("DON'T KNOW HOW TO HANDLE");
            }
            i += 2;
            continue;
        } 

        if opcode == 4 {
            //println!("OUTPUTS {}", val1);
            output.push(val1);
            //break;
            i += 2;
            continue;
        }

        if opcode == 1 {
            input[store as usize] = val1 + val2;
            i += 4;
        } else if opcode == 2 {
            input[store as usize] = val1 * val2;
            i += 4;
        } else if opcode == 5 {
            if val1 != 0 {
                i = val2 as usize;
            } else {
                i += 3;
            }
            continue;
        } else if opcode == 6 {
            if val1 == 0 {
                i = val2 as usize;
            } else {
                i += 3;
            }
            continue;
        } else if opcode == 7 {
            if val1 < val2 {
                input[store as usize] = 1;
            } else {
                input[store as usize] = 0;
            }
            i += 4;
            continue;
        } else if opcode == 8 {
            if val1 == val2 {
                input[store as usize] = 1
            } else {
                input[store as usize] = 0;
            }
            i += 4;
            continue;
        } else if opcode == 9 {
            relative_base += val1;
            i += 2;
        } else {
            panic!("Unknown opcode {}", opcode);
        }

    }
    positions
}
