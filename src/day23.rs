use crate::intcode::{run_intcode, InputGenerator, OutputHandler};
use threadpool::ThreadPool;
use std::{thread, time};
use std::sync::Mutex;

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<i64> {    
    let ret = input.split(",").map(|token| token.parse::<i64>().unwrap()).collect();
    ret
}

#[aoc(day23, part1)]
fn find_solution1(input: &Vec<i64>) -> i64 {
    lazy_static! {
        static ref msgs: Mutex<Vec<(usize, usize, usize)>> = Mutex::new(Vec::new());        
    }
    
    static mut ip_sent: Vec<bool> = Vec::new();
    static mut temp_y: Vec<usize> = Vec::new();
    static mut send_y_next: Vec<bool> = Vec::new();
    static mut msg_type: Vec<usize> = Vec::new();
    static mut msg_ip: Vec<usize> = Vec::new();
    static mut out_x: Vec<usize> = Vec::new();
    static mut out_y: Vec<usize> = Vec::new();

    static mut nat_packet: (usize, usize) = (0, 0);

    println!("STARTING PROGRAM");

    unsafe {
        let ig = |ip: usize| -> InputGenerator {
            Box::new(move || {
                if !ip_sent[ip] {
                    ip_sent[ip] = true;
                    return ip as i64;
                }
                if !send_y_next[ip] {
                    let mut lock = msgs.lock().unwrap();
                    let mut retval = -1;
                    for i in 0..lock.len() {
                        let current_msg_ip = lock[i].0;
                        if current_msg_ip == ip {
                            let (_, x, y) = lock[i];
                            temp_y[ip] = y;
                            send_y_next[ip] = true;
                            lock.remove(i);
                            retval = x as i64;
                            break;
                        }
                    }
                    drop(lock);
                    let ten_millis = time::Duration::from_millis(1);
                    thread::sleep(ten_millis);        
                    return retval;
                } else {
                    send_y_next[ip] = false;
                    return temp_y[ip] as i64;
                }
                -1
            })
        };
        
        let oh = |ip: usize| -> OutputHandler {
            Box::new(move |o: i64| {
                match msg_type[ip] {
                    0 => msg_ip[ip] = o as usize,
                    1 => out_x[ip] = o as usize,
                    2 => out_y[ip] = o as usize,
                    _ => panic!("Unknown state")
                }
                msg_type[ip] += 1;
                if msg_type[ip] > 2 {
                    if msg_ip[ip] == 255 {
                        //panic!("FOUND Y: {}", out_y[ip]);
                        //println!("GOT NAT PACKET!");
                        nat_packet = (out_x[ip], out_y[ip]);
                        msg_type[ip] = 0;
                    } else {
                        //println!("RECEIVING MESSAGE {} {} {}", msg_ip[ip], out_x[ip], out_y[ip]);
                        msgs.lock().unwrap().push((msg_ip[ip], out_x[ip], out_y[ip]));
                        msg_type[ip] = 0;
                    }
                }
            })
        };

        println!("GENERATING COMPUTERS");
        let pool = ThreadPool::new(50);
        for i in 0..50 {
            ip_sent.push(false);
            temp_y.push(0);
            send_y_next.push(false);
            msg_type.push(0);
            msg_ip.push(0);
            out_x.push(0);
            out_y.push(0);

            let mut program = input.clone();
            pool.execute(move || run_intcode(&mut program, &ig(i), &oh(i)));
        }
        let mut heartbeat = 0;
        loop {
            let ten_millis = time::Duration::from_millis(200);
            thread::sleep(ten_millis);
            let mut lock = msgs.lock().unwrap();
            heartbeat += 1;
            //println!("CURRENT STATE: {:?}, {}", lock, heartbeat);
            if lock.len() == 0 && nat_packet != (0, 0) {
                println!("SENDING DATA {:?} to NAT", nat_packet);
                lock.push((0, nat_packet.0, nat_packet.1));
            }
            drop(lock);
        }
        pool.join();
    }
    0
}

