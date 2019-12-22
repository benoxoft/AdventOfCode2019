use crate::intcode::{run_intcode, InputGenerator, OutputHandler};
use std::fs::File;
use std::io::Write as IoWrite;

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Vec<i64> {    
    let ret = input.split(",").map(|token| token.parse::<i64>().unwrap()).collect();
    ret
}

#[aoc(day19, part1)]
fn find_solution1(input: &Vec<i64>) -> usize {

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
        for i in 0..3000 {
            posy = i;
            for j in 0..3000 {
                posx = j;
                //println!("checking position {} {}", posx, posy);
                run_intcode(&mut program.clone(), &ig(), &oh());    
            }
            output.push(Vec::new());
        }

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

        for y in 0..output.len() {
            for x in 0..output[y].len() {
                if theres_a_square(&output, (x, y)) {
                    return x * 10000 + y;
                }
            }
        }
    }
    count
}

fn theres_a_square(data: &Vec<Vec<i64>>, point: (usize, usize)) -> bool {

    let init_x = point.0 as usize;
    let init_y = point.1 as usize;

    for x in init_x..init_x + 100 {
        for y in init_y..init_y + 100 {
            if data[y][x] == 0 {
                return false;
            }
        }
    }
    true
}
