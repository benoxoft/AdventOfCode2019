#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<i128> {    
    let ret = input.split(",").map(|token| token.parse::<i128>().unwrap()).collect();
    ret
}


#[aoc(day9, part1)]
fn find_solution1(input: &Vec<i128>) -> i128 {
    let ret = run_program(&mut input.clone(), 1);
    let output = ret.1;
    output[0]
}


#[aoc(day9, part2)]
fn find_solution2(input: &Vec<i128>) -> i128 {
    let ret = run_program(&mut input.clone(), 2);
    let output = ret.1;
    output[0]
}

#[test]
fn test_program1() {
    let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    assert_eq!(run_program(&mut program.clone(), 0).1, program);
}

#[test]
fn test_program2() {
    let program = vec![104,1125899906842624,99];
    assert_eq!(run_program(&mut program.clone(), 0).1[0], 1125899906842624);
}

#[test]
fn test_program3() {
    let program = vec![1102,34915192,34915192,7,4,7,99,0];
    assert_eq!(run_program(&mut program.clone(), 0).1.len(), 1);
    assert_eq!(run_program(&mut program.clone(), 0).1[0].to_string().len(), 16);
}

fn run_program(input: &mut Vec<i128>, prog_input: i128) -> (Vec<i128>, Vec<i128>) {
    let mut i = 0;
    let mut output:Vec<i128> = Vec::new();
    let mut steps = 0;
    let mut relative_base: i128 = 0;

    loop {
        steps += 1;

        let strop = (input[i] + 100_000).to_string();
        let mode3 = strop.chars().nth(1).unwrap(); // 10 000
        let mode2 = strop.chars().nth(2).unwrap(); //  1 000
        let mode1 = strop.chars().nth(3).unwrap(); //    100
        let opcode = strop[4..6].parse::<i128>().unwrap() as i128;

        if opcode == 99 {
            println!("OPCODE 99 REACHED");
            break;
        }

        let val1;
        let val2;
        let pos1: i128 = input[i + 1];
        let pos2: i128 = input[i + 2];
        let pos3;
        let store;

        /* println!("{:?}", input);*/

        if pos2 > input.len() as i128 {
            input.resize((pos2 + 2) as usize, 0);
        }

        if mode1 == '0' {
            val1 = input[pos1 as usize];
        } else if mode1 == '1' {
            val1 = pos1;
        } else if mode1 == '2' {
            let idx = pos1 + relative_base;
            if idx < 0 {
                panic!("Unexpected index lesser than 0");
            }
            val1 = input[idx as usize];
        } else {
            panic!("Cannot comprehend mode");
        }

        if mode2 == '0' {
            val2 = input[pos2 as usize];
        } else if mode2 == '1' {
            val2 = pos2;
        } else if mode2 == '2' {
            let idx = pos2 + relative_base;
            if idx < 0 {
                panic!("Unexpected index lesser than 0");
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

        // println!("strop{} i {} pos1 {} pos2 {} pos3 {} store {} rb {}", strop, i, pos1, pos2, pos3, store, relative_base);

        if opcode == 3 {
            println!("USES INPUT {} AS INPUT", prog_input);
            if mode1 == '2' {
                input[(pos1 + relative_base) as usize] = prog_input;
            } else {
                input[pos1 as usize] = prog_input;
            }
            i += 2;
            continue;
        } 

        if opcode == 4 {
            println!("OUTPUTS {}", val1);
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
    (input.to_vec(), output.to_vec())
}


#[test]
fn test_prog() {
    let mut program1 = vec![3,0,4,0,99];
    run_program(&mut program1, 32534);

    let mut program2 = vec![1101,100,-1,4,0];
    run_program(&mut program2, 32534);
}

#[test]
fn test_program() {
    let mut program1 = vec![1, 0, 0, 0, 99];
    let expected1 = vec![2, 0, 0, 0, 99];
    let (result1, _) = run_program(&mut program1, 0);
    assert_eq!(result1, expected1);

    let mut program2 = vec![2, 3, 0, 3, 99];
    let expected2 = vec![2, 3, 0, 6, 99];
    let (result2, _) = run_program(&mut program2, 0);
    assert_eq!(result2, expected2);

    let mut program3 = vec![2, 4, 4, 5, 99, 0];
    let expected3 = vec![2, 4, 4, 5, 99, 9801];
    let (result3, _) = run_program(&mut program3, 0);
    assert_eq!(result3, expected3);

    let mut program4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let expected4 = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    let (result4, _) = run_program(&mut program4, 0);
    assert_eq!(result4, expected4);
}
/*
#[test]
fn test_part2() {
    let program1 = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
    1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    let (_, result1) = run_program(&mut program1.clone(), 5);
    assert_eq!(result1[0], 999);
    let (_, result1) = run_program(&mut program1.clone(), 8);
    assert_eq!(result1[0], 1000);
    let (_, result1) = run_program(&mut program1.clone(), 9);
    assert_eq!(result1[0], 1001);
}
*/

#[test]
fn test_part2_1() {
    let program1 = vec![3,9,8,9,10,9,4,9,99,-1,8];
    let (_, result1) = run_program(&mut program1.clone(), 8);
    assert_eq!(result1[result1.len() - 1], 1);
    let (_, result1) = run_program(&mut program1.clone(), 18);
    assert_eq!(result1[result1.len() - 1], 0);

    let program2 = vec![3,9,7,9,10,9,4,9,99,-1,8];
    let (_, result2) = run_program(&mut program2.clone(), 5);
    assert_eq!(result2[result2.len() - 1], 1);
    let (_, result2) = run_program(&mut program2.clone(), 15);
    assert_eq!(result2[result2.len() - 1], 0);

    let program3 = vec![3,3,1108,-1,8,3,4,3,99];
    let (_, result3) = run_program(&mut program3.clone(), 8);
    assert_eq!(result3[result3.len() - 1], 1);
    let (_, result3) = run_program(&mut program3.clone(), 18);
    assert_eq!(result3[result3.len() - 1], 0);

    let program4 = vec![3,3,1107,-1,8,3,4,3,99];
    let (_, result4) = run_program(&mut program4.clone(), 5);
    assert_eq!(result4[result4.len() - 1], 1);
    let (_, result4) = run_program(&mut program4.clone(), 8);
    assert_eq!(result4[result4.len() - 1], 0);

}

#[test]
fn test_jumps() {
    let program1 = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    let (_, result1) = run_program(&mut program1.clone(), 12);
    assert_eq!(result1[0], 1);
}