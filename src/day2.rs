#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<usize> {    
    let ret = input.split(",").map(|token| token.parse::<usize>().unwrap()).collect();
    ret
}


#[aoc(day2, part1)]
fn find_solution1(input: &Vec<usize>) -> usize {
    let mut data = input.clone();
    data[1] = 12;
    data[2] = 2;
    let result = run_program(&mut data);
    result[0]
}

#[aoc(day2, part2)]
fn find_solution2(input: &Vec<usize>) -> usize {
    for i in 0..99 {
        for j in 0..99 {
            let mut data = input.clone();
            data[1] = i;
            data[2] = j;
            let result = run_program(&mut data);
            if result[0] == 19690720 {
                return 100 * i + j;
            }
        }
    }
    panic!("Could not find answer")
}

fn run_program(input: &mut Vec<usize>) -> Vec<usize> {
    for i in 0..input.len() / 4 {
        let opcode = input[i * 4];
        let pos1 = input[i * 4 + 1];
        let pos2 = input[i * 4 + 2];
        let val1 = input[pos1];
        let val2 = input[pos2];
        let store: usize = input[i * 4 + 3];

        if opcode == 1 {
            input[store] = val1 + val2
        } else if opcode == 2 {
            input[store] = val1 * val2
        } else if opcode == 99 {
            break;
        }
    }
    input.to_vec()
}

#[test]
fn test_program() {
    let mut program1 = vec![1, 0, 0, 0, 99];
    let result1 = vec![2, 0, 0, 0, 99];
    assert_eq!(result1, run_program(&mut program1));

    let mut program2 = vec![2, 3, 0, 3, 99];
    let result2 = vec![2, 3, 0, 6, 99];
    assert_eq!(result2, run_program(&mut program2));

    let mut program3 = vec![2, 4, 4, 5, 99, 0];
    let result3 = vec![2, 4, 4, 5, 99, 9801];
    assert_eq!(result3, run_program(&mut program3));

    let mut program4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let result4 = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    assert_eq!(result4, run_program(&mut program4));
}