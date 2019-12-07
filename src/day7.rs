extern crate itertools;

use itertools::Itertools;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<isize> {    
    let ret = input.split(",").map(|token| token.parse::<isize>().unwrap()).collect();
    ret
}

#[aoc(day7, part1)]
fn find_solution1(program: &Vec<isize>) -> isize {
    let it = (0..5).permutations(5);

    let mut outputs: Vec<isize> = Vec::new();
    let mut input = 0;

    for seq in it {
        println!("IT");
        input = 0;
        for element in seq {
            println!("{} {}", input, element);
            let (_, output, _, _) = run_program(&mut program.clone(), input, element, false, 0);
            input = *output.get(output.len()-1).unwrap();    
        }
        outputs.push(input);
    }
    let mut max_amp = 0;
    for o in outputs {
        if o > max_amp {
            max_amp = o;
        }
    }
    max_amp
}

#[test]
fn test_2_prog_1() {
    let program_str = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    let program = parse_input(program_str);
    let mut input = 0;

    let out = amp_the_output(&program, &vec![9,8,7,6,5]);
    assert_eq!(out, 139629729);

}

fn amp_the_output(program: &Vec<isize>, seq: &Vec<isize>) -> isize {
    let mut input = 0;
    let mut running_programs: Vec<Vec<isize>> = (0..5).map(|x| program.clone()).collect();
    let mut stopped: bool = false;
    let mut pointers: Vec<usize> = vec![0, 0, 0, 0, 0];
    let mut current_amp = 0;

    for element in seq {
        println!("INPUT: {} AMP PHASE: {}", input, element);
        let (running_program, output, stopped, i) = run_program(&mut running_programs[current_amp], input, *element, false, pointers[current_amp]);
        running_programs[current_amp] = running_program;
        pointers[current_amp] = i;
        input = *output.get(output.len()-1).unwrap();
        current_amp += 1;
        if stopped {
            panic!("Didn't expect to stop there");
        }
    }
    current_amp = 0;
    loop {
        let (running_program, output, stopped, i) = run_program(&mut running_programs[current_amp], input, 0, true, pointers[current_amp]);
        running_programs[current_amp] = running_program;
        pointers[current_amp] = i;
        input = *output.get(output.len()-1).unwrap();
        current_amp += 1;
        if stopped && current_amp == 5 {
            break;
        }
        if current_amp == 5 {
            current_amp = 0;
        }
    }
    input
}

#[aoc(day7, part2)]
fn find_solution2(program: &Vec<isize>) -> isize {
    let it = (5..10).permutations(5);

    let mut outputs: Vec<isize> = Vec::new();

    for seq in it {
        println!("IT");
        outputs.push(amp_the_output(&program, &seq));
    }
    let mut max_amp = 0;
    for o in outputs {
        if o > max_amp {
            max_amp = o;
        }
    }
    max_amp
}

fn run_program(input: &mut Vec<isize>, 
    prog_input: isize, 
    amplifier: isize, 
    mut amped: bool,
    mut i: usize) -> (Vec<isize>, Vec<isize>, bool, usize) {

    let mut output:Vec<isize> = Vec::new();
    let mut steps = 0;
    let mut stopped = false;
    let mut inputted = false;

    loop {
        steps += 1;
        if steps == 500 {
            break;
        }
        let strop = (input[i] + 100_000).to_string();
        let mode3 = strop.chars().nth(1).unwrap(); // 10 000
        let mode2 = strop.chars().nth(2).unwrap(); //  1 000
        let mode1 = strop.chars().nth(3).unwrap(); //    100
        let opcode = strop[4..6].parse::<usize>().unwrap() as usize;

        if opcode == 99 {
            println!("OPCODE 99 REACHED");
            stopped = true;
            break;
        }

        let val1;
        let val2;
        let pos1 = input[i + 1] as usize;
        let pos2 = input[i + 2] as usize;
        let pos3;
        let store;
        if i + 3 < input.len() {
            pos3 = input[i + 3] as usize;
        } else {
            pos3 = std::usize::MAX            
        }
        store = pos3;

        if opcode == 3 {
            if !amped {
                println!("USES AMP {} AS INPUT", amplifier);
                input[pos1] = amplifier;
                amped = true;
            } else if !inputted {
                println!("USES INPUT {} AS INPUT", prog_input);
                input[pos1] = prog_input;
                inputted = true;
            } else {
                println!("AMPLIFIER NEEDS INPUT");
                break;
            }
            i += 2;
            continue;
        } 
        if opcode == 4 {
            println!("OUTPUTS {}", input[pos1]);
            output.push(input[pos1]);
            //break;
            i += 2;
            continue;
        }

        if mode1 == '0' {
            val1 = input[pos1];
        } else if mode1 == '1' {
            val1 = pos1 as isize;
        } else {
            panic!("Cannot comprehend mode");
        }

        if mode2 == '0' {
            val2 = input[pos2];
        } else if mode2 == '1' {
            val2 = pos2 as isize;
        } else {
            panic!("Cannot comprehend mode");
        }

        if opcode == 1 {
            input[store] = val1 + val2;
            i += 4;
        } else if opcode == 2 {
            input[store] = val1 * val2;
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
                input[store] = 1;
            } else {
                input[store] = 0;
            }
            i += 4;
            continue;
        } else if opcode == 8 {
            if val1 == val2 {
                input[store] = 1
            } else {
                input[store] = 0;
            }
            i += 4;
            continue;    
        } else {
            panic!("Unknown opcode {}", opcode);
        }

    }
    (input.to_vec(), output.to_vec(), stopped, i)
}


#[test]
fn test_prog1() {
    let program_str = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let program = parse_input(program_str);
    let mut input = 0;

    for i in vec![4,3,2,1,0] {
        let (_, output, _, _) = run_program(&mut program.clone(), input, i, false, 0);
        input = *output.get(output.len()-1).unwrap();
    }
    assert_eq!(input, 43210);
}

#[test]
fn test_prog2() {
    let program_str = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    let program = parse_input(program_str);
    let mut input = 0;

    for i in vec![0,1,2,3,4] {
        let (_, output, _, _) = run_program(&mut program.clone(), input, i, false, 0);
        input = *output.get(output.len()-1).unwrap();
    }
    assert_eq!(input, 54321);
}

#[test]
fn test_prog3() {
    let program_str = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    let program = parse_input(program_str);
    let mut input = 0;

    for i in vec![1,0,4,3,2] {
        let (_, output, _, _) = run_program(&mut program.clone(), input, i, false, 0);
        input = *output.get(output.len()-1).unwrap();
    }
    assert_eq!(input, 65210);
}
