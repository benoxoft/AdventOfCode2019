use crate::intcode::{run_intcode, InputGenerator, OutputHandler};

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Vec<i64> {    
    let ret = input.split(",").map(|token| token.parse::<i64>().unwrap()).collect();
    ret
}

#[aoc(day21, part1)]
fn find_solution1(input: &Vec<i64>) -> i64 {

    let mut program = input.clone();

    static input_script: &str = "NOT A J
NOT C T
AND D T
AND H T
OR T J
NOT B T
AND E T
AND D T
OR T J
NOT B T
AND C T
AND D T
OR T J
RUN    
";
    static mut idx: usize = 0;

    unsafe {
        let ig = || -> InputGenerator {
            Box::new(|| {
                let z = input_script.chars().nth(idx).unwrap() as i64;
                idx += 1;
                z
            })
        };
        
        let oh = || -> OutputHandler {
            Box::new(|o: i64| {
                //print!("{}", o as u8 as char);
                print!(" {} ", o);
            })
        };
        run_intcode(&mut program.clone(), &ig(), &oh());    
        0
    }
}
