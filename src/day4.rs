
// 264793-803935

#[aoc_generator(day4)]
fn parse_input(input: &str) -> (usize, usize) {
    (264793, 803935)
}

#[aoc(day4, part1)]
fn find_solution1(input: &(usize, usize)) -> usize {
    let mut counter = 0;
    for i in input.0..input.1 {
        if validator(i) {
            counter += 1;
        }
    }
    counter
}

#[aoc(day4, part2)]
fn find_solution2(input: &(usize, usize)) -> usize {
    let mut counter = 0;
    for i in input.0..input.1 {
        if validator2(i) {
            counter += 1;
        }
    }
    counter
}

fn validator(number: usize) -> bool {
    let string_input = number.to_string();
    at_least_a_double(&string_input) && never_decrease(&string_input)
}

fn at_least_a_double(number: &str) -> bool {
    for i in 0..number.len() {
        if number.chars().nth(i) == number.chars().nth(i + 1) {
            return true;
        }
    }
    false
}

fn never_decrease(number: &str) -> bool {
    let mut indice = '0';
    for i in 0..number.len() {
        if number.chars().nth(i).unwrap() < indice {
            return false;
        }
        indice = number.chars().nth(i).unwrap();
    }
    true
}

fn validator2(number: usize) -> bool {
    let string_input = number.to_string();
    at_least_a_double2(&string_input) && never_decrease2(&string_input)
}

fn at_least_a_double2(number: &str) -> bool {
    for i in 0..number.len() {
        if i > 0 && 
           i < number.len() && 
           number.chars().nth(i) == number.chars().nth(i + 1) &&
           number.chars().nth(i) != number.chars().nth(i - 1) &&
           number.chars().nth(i) != number.chars().nth(i + 2) {
            return true;
        } else if i == 0 && number.chars().nth(i) == number.chars().nth(i + 1) &&
                            number.chars().nth(i) != number.chars().nth(i + 2) {
            return true;
        } else if i == number.len() &&
                  number.chars().nth(i) == number.chars().nth(i - 1) &&
                  number.chars().nth(i) != number.chars().nth(i - 2) {
            return true;

        }
    }
    false
}

fn never_decrease2(number: &str) -> bool {
    let mut indice = '0';
    for i in 0..number.len() {
        if number.chars().nth(i).unwrap() < indice {
            return false;
        }
        indice = number.chars().nth(i).unwrap();
    }
    true
}

#[test]
fn test_part2() {
    assert_eq!(true, validator2(112233));
    assert_eq!(false, validator2(123444));
    assert_eq!(true, validator2(111122));
}

#[test]
fn test_never_decrease() {
    assert_eq!(true, never_decrease("234567"));
    assert_eq!(false, never_decrease("3456478"));
}

#[test]
fn test_at_least_a_double() {
    assert_eq!(true, at_least_a_double("123343"));
    assert_eq!(false, at_least_a_double("13546"));
}

#[test]
fn test_validator() {
    assert_eq!(true, validator(111111));
    assert_eq!(false, validator(223450));
    assert_eq!(false, validator(123789));
}