mod puzzle;

fn parse_input(input: &str) -> Vec<i32> {    
    let ret = input.split("\n").map(|token| token.parse::<i32>().unwrap()).collect();
    ret
}

fn calculate_fuel(mass: &i32) -> i32 {
    let step1: i32 = mass / 3;
    let step2 = step1 - 2;
    if step2 < 0 {
        0
    } else {
        step2
    }
}

fn calculate_fuel_2(mass: &i32) -> i32 {
    let mut calc_fuel = calculate_fuel(mass);
    let mut ret = calc_fuel;
    while calc_fuel > 0 {
        calc_fuel = calculate_fuel(&calc_fuel);
        ret += calc_fuel;
    }
    ret
}

fn main() {
    let input = parse_input(puzzle::INPUT);

    let answer: i32 = input.iter().map(calculate_fuel).sum();
    println!("Answer 1: {}", answer);

    let answer2: i32 = input.iter().map(calculate_fuel_2).sum();
    println!("Answer 2: {}", answer2);

}

#[test]
fn test_calculate_fuel() {
    assert_eq!(2, calculate_fuel(&12));
    assert_eq!(2, calculate_fuel(&14));
    assert_eq!(654, calculate_fuel(&1969));
    assert_eq!(33583, calculate_fuel(&100756));
}

#[test]
fn test_calculate_fuel_2() {
    assert_eq!(2, calculate_fuel_2(&14));
    assert_eq!(966, calculate_fuel_2(&1969));
    assert_eq!(50346, calculate_fuel_2(&100756));
}

#[test]
fn test_parse_input() {
    let input = "12\n34\n56";
    let result = parse_input(input);
    let expected = vec![12, 34, 56];
    assert_eq!(expected, result);
}
