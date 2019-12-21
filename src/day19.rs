use crate::intcode::{run_intcode, InputGenerator, OutputHandler};

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Vec<i64> {    
    let ret = input.split(",").map(|token| token.parse::<i64>().unwrap()).collect();
    ret
}

#[aoc(day19, part1)]
fn find_solution1(input: &Vec<i64>) -> i64 {

    let mut program = input.clone();
    static mut posx: i64 = 0;
    static mut posy: i64 = 0;
    static mut flip: bool = false;
    static mut output: Vec<Vec<i64>> = Vec::new();
    let mut count = 0;

    unsafe {
        output.push(Vec::new());
        let ig = || -> InputGenerator {
            Box::new(|| {
                flip = !flip;
                if flip {
                    posx
                } else {
                    posy
                }
            })
        };
        
        let oh = || -> OutputHandler {
            Box::new(|o: i64| {
                output[posy as usize].push(o);
            })
        };
        for i in 0..10000 {
            posy = i;
            for j in 0..10000 {
                posx = j;
                //println!("checking position {} {}", posx, posy);
                run_intcode(&mut program.clone(), &ig(), &oh());    
            }
            output.push(Vec::new());
        }

        use std::fs::File;
        use std::io::Write as IoWrite;
        let mut file = File::create("C:\\Users\\Benoit\\foo.txt").unwrap();
    for line in &output {
            for c in line {
                if *c == 1 {
                    count += 1;
                }
            }
            writeln!(file, "{:?}", line);
        }
        file.flush();
    }
    count
}

