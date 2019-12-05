#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<isize> {    
    let ret = input.split(",").map(|token| token.parse::<isize>().unwrap()).collect();
    ret
}

#[aoc(day5, part1)]
fn find_solution1(input: &Vec<isize>) -> isize {
    let mut data = input.clone();
    let (result, _) = run_program(&mut data, 1);
    result[0]
}

#[aoc(day5, part2)]
fn find_solution2(input: &Vec<isize>) -> isize {
    let mut data = input.clone();
    let (_, result) = run_program(&mut data, 5);
    0
}


fn run_program(input: &mut Vec<isize>, prog_input: isize) -> (Vec<isize>, Vec<isize>) {
    let mut i = 0;
    let mut output:Vec<isize> = Vec::new();
    let mut steps = 0;

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
            input[pos1] = prog_input;
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
        /*if mode3 == '0' {
            store = input[pos3] as usize;
        } else if mode3 == '1' {
            store = pos3 as usize;
        } else {
            panic!("Cannot comprehend mode");
        }*/

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