
use num::integer::lcm;

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Vec<isize> {
    let mut parsed = Vec::new();
    for c in input.chars() {
        parsed.push(c.to_digit(10).unwrap() as isize);
    }
    parsed
}

#[aoc(day16, part1)]
fn find_solution1(input: &Vec<isize>) -> String {
    let mut signal = input.clone();

    for _ in 0..100 {
        signal = calc_phase(&mut signal, 0, input.len());
    }
    let mut s = String::with_capacity(8);
    for i in 0..8 {
        s.push_str(&signal[i].to_string());
    }
    s
}

#[aoc(day16, part2)]
fn find_solution2(input: &Vec<isize>) -> String {
    test_example2();

    let mut signal = Vec::new();
    for _ in 0..10000 {
        signal.append(&mut input.clone());
    }    

    for i in 0..100 {
        signal = calc_phase(&mut signal, 0, input.len());
        println!("DONE PROCESSING PHASE {}", i);
    }
    //println!("SIGNAL: \n\n{:?}\n\n", signal);
    let mut s = String::with_capacity(8);
    for i in 0..7 {
        s.push_str(&input[i].to_string());
    }
    let offset = s.parse::<usize>().unwrap();

    let mut s = String::with_capacity(8);
    for i in offset..offset+8 {
        s.push_str(&signal[i].to_string());
    }
    s
}

fn calc_phase(signal: &mut Vec<isize>, mut cycle: usize, repeat_size: usize) -> Vec<isize> {
    signal.insert(0, 0);

    let mut final_output: Vec<isize> = Vec::new();
    let base_pattern = vec![0, 1, 0, -1];
    let signal_len = signal.len();
    let mut output: isize = 0;

    for i in 0..signal_len / 2 {
        final_output.push(0 as isize);
        continue;
        output = 0;
        //let pattern = &cached_patterns[i];
        //println!();
        let mut short = lcm(base_pattern.len() * i + 1, repeat_size);
        if short > signal_len {
            short = signal_len;
        }
        if i % 1000 == 0 {
            println!("CURRENT: {}, SHORT: {}", i, short);
        }

        for j in (0..signal_len).step_by(i + 1) {
            let pattern = base_pattern[cycle % base_pattern.len()];
            //print!("{}", pattern);
            cycle += 1;

            if pattern == 0 {
                continue;
            }

            if pattern == 1 {
                for k in 0..i + 1 {
                    if j+k >= signal_len {
                        break;
                    }
                    output += signal[j+k];
                }    
            } else {
                for k in 0..i + 1 {
                    if j+k >= signal_len {
                        break;
                    }
                    output -= signal[j+k];
                }    
            }
        }
        output *= (signal_len / short) as isize;

        let s_output = output.to_string();
        let new_char = s_output.chars().nth(s_output.len() - 1).unwrap().to_digit(10).unwrap();
        final_output.push(new_char as isize);
        cycle = 0;
    }

    let mut output = 0;
    let mut final_output2: Vec<isize> = Vec::new();
    for i in 0..signal_len / 2 {
        output += signal[signal_len - i - 1];
        let s_output = output.to_string();
        let new_char = s_output.chars().nth(s_output.len() - 1).unwrap().to_digit(10).unwrap();
        final_output2.push(new_char as isize);
    }
    
    for i in 0..final_output2.len() {
        final_output.push(final_output2[final_output2.len() - i - 1]);
    }
    final_output
}

#[test]
fn test_example1() {

    let mut signal = parse_input("12345678");
    let mut final_output = calc_phase(&mut signal, 0, 8);
    assert_eq!(parse_input("48226158"), final_output);

    let mut final_output = calc_phase(&mut final_output, 0, 8);
    assert_eq!(parse_input("34040438"), final_output);

    let mut final_output = calc_phase(&mut final_output, 0, 8);
    assert_eq!(parse_input("03415518"), final_output);

    let final_output = calc_phase(&mut final_output, 0, 8);
    assert_eq!(parse_input("01029498"), final_output);   

}

// #[test]
fn test_example2() {
    let input = parse_input("03036732577212944063491565474664");
    let mut signal = Vec::new();
    for _ in 0..10000 {
        signal.append(&mut input.clone());
    }    

    for i in 0..100 {
        signal = calc_phase(&mut signal, 0, input.len());
        println!("DONE PROCESSING PHASE {}", i);
    }
    //println!("SIGNAL: \n\n{:?}\n\n", signal);
    let mut s = String::with_capacity(8);
    for i in 0..7 {
        s.push_str(&input[i].to_string());
    }
    let offset = s.parse::<usize>().unwrap();

    let mut s = String::with_capacity(8);
    for i in offset..offset+8 {
        s.push_str(&signal[i].to_string());
    }
    assert_eq!(s, "84462026");
}