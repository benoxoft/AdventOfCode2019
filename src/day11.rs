use std::collections::HashMap;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<i128> {    
    let ret = input.split(",").map(|token| token.parse::<i128>().unwrap()).collect();
    ret
}

#[aoc(day11, part1)]
fn find_solution1(input: &Vec<i128>) -> usize {
    let mut program = input.clone();
    let mut positions: HashMap<(i128, i128), i128> = HashMap::new();
    positions.insert((0i128, 0i128), 0i128);
    let mut posx = 0i128;
    let mut posy = 0i128;
    let mut dirx = 0i128;
    let mut diry = 1i128;
    let mut color = 0i128;
    let mut prog_index = 0;
    let mut relative_base = 0i128;

    loop {
        color = *positions.get(&(posx, posy)).unwrap_or(&0i128);
        let (_, output, reached99, prog_index, relative_base) = run_program(&mut program, color, prog_index, relative_base);
        
        //if positions.len() % 1000 == 0 {
        //    println!("output: {:?} len: {} color: {} idx: {} dirx: {} diry: {} posx: {} posy: {} relbase: {} positions: {:?}", output, positions.len(), color, prog_index, dirx, diry, posx, posy, relative_base, positions);
        //}

        //if positions.len() > 15 {
        //    panic!("egnsoi");
        //}

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

        if reached99 {
            break;
        }    
    }
    positions.len()
}

/*
#[aoc(day11, part2)]
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

*/

#[test]
fn test_run_program() {
    let input = "1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1101,3,0,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1102,1,37,1000,1101,856,0,1029,1101,286,0,1025,1101,39,0,1004,1101,861,0,1028,1101,845,0,1026,1102,28,1,1002,1102,1,0,1020,1101,0,892,1023,1101,0,291,1024,1101,35,0,1018,1101,0,27,1006,1102,1,26,1011,1101,33,0,1019,1102,31,1,1014,1102,1,36,1010,1102,23,1,1007,1101,0,32,1016,1101,29,0,1008,1101,20,0,1001,1102,1,25,1015,1101,38,0,1017,1101,0,24,1012,1102,1,22,1005,1101,1,0,1021,1101,0,21,1003,1102,1,838,1027,1102,1,30,1013,1101,895,0,1022,1101,0,34,1009,109,7,1208,0,22,63,1005,63,201,1001,64,1,64,1105,1,203,4,187,1002,64,2,64,109,-6,2102,1,5,63,1008,63,24,63,1005,63,223,1105,1,229,4,209,1001,64,1,64,1002,64,2,64,109,17,21102,40,1,-6,1008,1012,40,63,1005,63,255,4,235,1001,64,1,64,1106,0,255,1002,64,2,64,109,-15,21108,41,41,9,1005,1012,277,4,261,1001,64,1,64,1106,0,277,1002,64,2,64,109,11,2105,1,10,4,283,1105,1,295,1001,64,1,64,1002,64,2,64,109,-9,21101,42,0,8,1008,1013,44,63,1005,63,315,1105,1,321,4,301,1001,64,1,64,1002,64,2,64,109,13,1206,3,337,1001,64,1,64,1106,0,339,4,327,1002,64,2,64,109,-10,1208,0,29,63,1005,63,361,4,345,1001,64,1,64,1106,0,361,1002,64,2,64,109,2,2108,27,-4,63,1005,63,383,4,367,1001,64,1,64,1105,1,383,1002,64,2,64,109,-4,1207,2,30,63,1005,63,405,4,389,1001,64,1,64,1105,1,405,1002,64,2,64,109,22,1205,-8,417,1106,0,423,4,411,1001,64,1,64,1002,64,2,64,109,-27,2108,19,0,63,1005,63,443,1001,64,1,64,1106,0,445,4,429,1002,64,2,64,109,13,21108,43,45,-1,1005,1013,461,1106,0,467,4,451,1001,64,1,64,1002,64,2,64,109,1,21107,44,45,4,1005,1019,485,4,473,1105,1,489,1001,64,1,64,1002,64,2,64,109,-8,2102,1,-7,63,1008,63,37,63,1005,63,515,4,495,1001,64,1,64,1106,0,515,1002,64,2,64,109,1,2107,38,-4,63,1005,63,533,4,521,1105,1,537,1001,64,1,64,1002,64,2,64,109,4,21107,45,44,1,1005,1013,553,1106,0,559,4,543,1001,64,1,64,1002,64,2,64,109,-7,2107,21,-4,63,1005,63,575,1106,0,581,4,565,1001,64,1,64,1002,64,2,64,109,9,1205,7,599,4,587,1001,64,1,64,1105,1,599,1002,64,2,64,109,-11,2101,0,-3,63,1008,63,40,63,1005,63,619,1105,1,625,4,605,1001,64,1,64,1002,64,2,64,109,1,2101,0,-2,63,1008,63,28,63,1005,63,651,4,631,1001,64,1,64,1106,0,651,1002,64,2,64,109,1,21102,46,1,7,1008,1012,44,63,1005,63,671,1106,0,677,4,657,1001,64,1,64,1002,64,2,64,109,4,1201,-7,0,63,1008,63,28,63,1005,63,699,4,683,1105,1,703,1001,64,1,64,1002,64,2,64,109,-6,1207,-3,36,63,1005,63,719,1105,1,725,4,709,1001,64,1,64,1002,64,2,64,109,-4,1201,6,0,63,1008,63,23,63,1005,63,745,1106,0,751,4,731,1001,64,1,64,1002,64,2,64,109,8,1202,-6,1,63,1008,63,20,63,1005,63,777,4,757,1001,64,1,64,1105,1,777,1002,64,2,64,109,5,1202,-5,1,63,1008,63,25,63,1005,63,801,1001,64,1,64,1105,1,803,4,783,1002,64,2,64,109,8,21101,47,0,-6,1008,1014,47,63,1005,63,829,4,809,1001,64,1,64,1106,0,829,1002,64,2,64,109,1,2106,0,6,1001,64,1,64,1106,0,847,4,835,1002,64,2,64,109,11,2106,0,-4,4,853,1105,1,865,1001,64,1,64,1002,64,2,64,109,-15,1206,3,883,4,871,1001,64,1,64,1106,0,883,1002,64,2,64,109,14,2105,1,-8,1105,1,901,4,889,1001,64,1,64,4,64,99,21102,1,27,1,21102,1,915,0,1106,0,922,21201,1,57564,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,1,942,0,1105,1,922,22101,0,1,-1,21201,-2,-3,1,21101,957,0,0,1105,1,922,22201,1,-1,-2,1106,0,968,21202,-2,1,-2,109,-3,2106,0,0";
    let mut parsed = parse_input(input);
    let output = run_program(&mut parsed, 1, 0, 0);
    println!("{:?}", output.1);
    panic!("blak");
}

fn run_program(input: &mut Vec<i128>, prog_input: i128, mut i: usize, mut relative_base: i128) -> (Vec<i128>, Vec<i128>, bool, usize, i128) {
    let mut output:Vec<i128> = Vec::new();
    let mut input_consumed = false;
    let mut reached99 = false;

    loop {

        let strop = (input[i] + 100_000).to_string();
        let mode3 = strop.chars().nth(1).unwrap(); // 10 000
        let mode2 = strop.chars().nth(2).unwrap(); //  1 000
        let mode1 = strop.chars().nth(3).unwrap(); //    100
        let opcode = strop[4..6].parse::<i128>().unwrap() as i128;

        if opcode == 99 {
            println!("OPCODE 99 REACHED");
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

        if opcode == 3 {
            if input_consumed {
                assert_eq!(output.len(), 2);
                break;
            } else {
                input_consumed = true;
            }
            //println!("USES INPUT {} AS INPUT", prog_input);
            if mode1 == '2' {
                input[(pos1 + relative_base) as usize] = prog_input;
            } else if mode1 == '0' {
                input[pos1 as usize] = prog_input;
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
    (input.to_vec(), output.to_vec(), reached99, i, relative_base)
}
