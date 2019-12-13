use std::collections::HashMap;
use crate::intcode::{run_intcode, InputGenerator, OutputHandler};
use std::sync::Mutex;
use std::{thread, time};

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<i64> {    
    let ret = input.split(",").map(|token| token.parse::<i64>().unwrap()).collect();
    ret
}

#[aoc(day13, part1)]
fn find_solution1(input: &Vec<i64>) -> i64 {

    lazy_static! { 
        static ref positions: Mutex<HashMap<(i64, i64), i64>> = Mutex::new(HashMap::new());
        static ref current_time: Mutex<std::cell::Cell<time::Instant>> = Mutex::new(std::cell::Cell::new(time::Instant::now()));
    }

    static mut pos: (i64, i64) = (0, 0);
    static mut output_type: u8 = 0;
    static mut count_id: usize = 0;
    static mut score: i64 = 0;
    static mut paddle_pos: (i64, i64) = (0, 0);
    static mut ball_pos: (i64, i64) = (0, 0);

    static mut frame: time::Duration = time::Duration::from_millis(1000 / 30);

    let mut program = input.clone();
    program[0] = 2;

    unsafe {
        let ig = || -> InputGenerator {
            Box::new(|| {
                let ten_millis = time::Duration::from_millis(10);
                thread::sleep(ten_millis);

                let mut out = String::with_capacity(1600);
                for y in 0..30 {
                    for x in 0..50 {
                        let tile = *positions.lock().unwrap().get(&(x, y)).unwrap_or(&0i64);
                        let s = match tile {
                            0 => " ",
                            1 => "W",
                            2 => "#",
                            3 => {
                                paddle_pos = (x, y);
                                "_"
                            },
                            4 => {
                                ball_pos = (x, y);
                                "O"
                            },
                            _ => ""
                        };
                        out.push_str(s);
                    }
                    out.push_str("\n");
                }
                let mut line = String::new();

                if current_time.lock().unwrap().get().elapsed() >= frame {
                    println!("{}[2J{} Score: {}" , 27 as char, out, score);
                    current_time.lock().unwrap().set(time::Instant::now());
                }
                if paddle_pos.0 < ball_pos.0 {
                    1
                } else if paddle_pos.0 > ball_pos.0 {
                    -1
                } else {
                    0
                }
            })
        };
        
        let oh = || -> OutputHandler {
            Box::new(|o: i64| {
                match output_type {
                    0 => {
                        pos.0 = o;
                        output_type += 1;
                    },
                    1 => {
                        pos.1 = o;
                        output_type += 1;
                    },
                    2 => {
                        *positions.lock().unwrap().entry(pos).or_insert(0i64) = o;
                        output_type = 0;
                        if pos == (-1, 0) {
                            score = o;
                        } else if o == 2 {
                            count_id += 1;
                        }
                    },
                    _ => ()
                }
            })
        };
        run_intcode(&mut program.clone(), &ig(), &oh());        
        score
    }

}
