use std::collections::HashMap;

struct Orbits {
    planets: HashMap<String, String>,
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Orbits {    
    let pairs = input.lines();
    let mut orbits = Orbits{
        planets: HashMap::new(),
    };

    for pair in pairs {
        let planets: Vec<&str> = pair.split(")").collect();
        orbits.planets.insert(planets[1].to_owned(), planets[0].to_owned());
    }
    orbits
}

#[aoc(day6, part1)]
fn find_solution1(orbits: &Orbits) -> usize {
    let x = count_orbits(&orbits);
    x
}

#[aoc(day6, part2)]
fn find_solution2(orbits: &Orbits) -> usize {
    let x = calc_santa(&orbits);
    x
}

fn count_orbits(orbits: &Orbits) -> usize {
    let mut count = 0;

    for (k, v) in &orbits.planets {
        // println!("1 {} ORBITS {} COUNT: {}", k, v, count);

        count += 1;
        let mut current_orbit = v;
        loop {
            if orbits.planets.contains_key(&current_orbit.to_owned()) {
                // println!("2 {} ORBITS {} COUNT: {}", current_orbit, orbits.planets.get(&current_orbit.to_owned()).unwrap(), count);
                count += 1;
                current_orbit = orbits.planets.get(current_orbit).unwrap();
            } else {
                break;
            }
        }
    }
    count
}

fn calc_santa(orbits: &Orbits) -> usize {
    let mut current_orbit = orbits.planets.get("YOU").unwrap();
    let mut count = 0;
    let mut steps: HashMap<String, usize> = HashMap::new();

    loop {
        // println!("2 {} ORBITS {} COUNT: {}", current_orbit, orbits.planets.get(&current_orbit.to_owned()).unwrap(), count);
        if current_orbit == "COM" {
            break;
        } else if orbits.planets.contains_key(&current_orbit.to_owned()) {
            count += 1;
            current_orbit = orbits.planets.get(current_orbit).unwrap();
            steps.insert(current_orbit.to_owned(), count);
        } else {
            panic!("COULD NOT FIND");
        }
    }

    current_orbit = orbits.planets.get("SAN").unwrap();
    count = 0;

    loop {
        if steps.contains_key(&current_orbit.to_owned()) {
            count += steps.get(&current_orbit.to_owned()).unwrap();
            break;
        } else if orbits.planets.contains_key(&current_orbit.to_owned()) {
            count += 1;
            current_orbit = orbits.planets.get(current_orbit).unwrap();
        } else {
            panic!("COULD NOT FIND");
        }
    }
    count
}

#[test]
fn test_orbit() {
    let input_data = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    let parsed = parse_input(input_data);
    assert_eq!(42, count_orbits(&parsed));
}

#[test]
fn test_santa() {
    let input_data = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
    let parsed = parse_input(input_data);
    assert_eq!(4, calc_santa(&parsed));
}
