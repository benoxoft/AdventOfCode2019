use std::collections::HashMap;

#[derive(Debug)]
struct Recipe {
    name: String,
    output: usize,
    materials: String
}

//#[aoc_generator(day9)]
fn parse_input(input: &str) -> HashMap<String, Recipe> {       
    let mut ret = HashMap::new();

    let lines = input.lines();
    for line in lines {
        let tokens: Vec<&str> = line.split(" => ").collect();
        let mat_line = tokens[0];

        let output: Vec<&str> = tokens[1].split(" ").collect();
        let out_qty = output[0].parse::<usize>().unwrap();
        let out_name = output[1];
        let recipe = Recipe {
            name: out_name.to_owned(),
            output: out_qty,
            materials: mat_line.to_owned()
        };
        ret.insert(out_name.to_owned(), recipe);
    }
    ret
}

fn parse_mat_line(mat_line: &String) -> Vec<(String, usize)> {
    let materials_line: Vec<&str> = mat_line.split(", ").collect();
    let mut materials = Vec::new();
    for mat in materials_line {
        let splitted_mat: Vec<&str> = mat.split(" ").collect();
        let qty = splitted_mat[0].parse::<usize>().unwrap();
        let name = splitted_mat[1];
        materials.push((name.to_owned(), qty));
    }
    materials
}

#[aoc(day14, part1)]
fn find_solution1(input: &str) -> usize {
    let recipes = parse_input(input);
    let requested = Recipe {
        output: 1, 
        name: "FUEL".to_owned(),
        materials: "".to_owned()
    };
    let mut overflow = HashMap::new();
    let minerals = solve_recipe(&recipes, &requested, &mut overflow);
    println!("{:?}", overflow);
    minerals
}

#[aoc(day14, part2)]
fn find_solution2(input: &str) -> usize {
    let recipes = parse_input(input);
    let requested = Recipe {
        output: 1, 
        name: "FUEL".to_owned(),
        materials: "".to_owned()
    };
    let mut overflow = HashMap::new();
    let mut count = 0;
    let mut remaining_ore = 1000000000000i64;
    loop {
        let minerals = solve_recipe(&recipes, &requested, &mut overflow);
        remaining_ore -= minerals as i64;
        if remaining_ore < 0 {
            return count;
        } else {
            if count % 10000 == 0 {
                println!("REMAINING ORE: {}", remaining_ore);
            }
            count += 1;
        }
    }
}

fn solve_recipe(recipes: &HashMap<String, Recipe>, requested_recipe: &Recipe, overflow: &mut HashMap<String, usize>) -> usize {
    if !recipes.contains_key(&requested_recipe.name) {
        assert_eq!(requested_recipe.name, "ORE");
        // println!("RETURNING {} {}", requested_recipe.output, requested_recipe.name);
        return requested_recipe.output;
    }
    let matching_recipe = recipes.get(&requested_recipe.name).unwrap();
    let needed = if requested_recipe.output > matching_recipe.output {
         (requested_recipe.output + matching_recipe.output - 1) / (matching_recipe.output)
    } else {
        1
    };

    // println!("WE NEED {} RECIPES TO PRODUCE {} {}", needed, requested_recipe.name, requested_recipe.output);
    // println!("{} {} WILL BE PRODUCED IN OVER", (needed * matching_recipe.output) - requested_recipe.output, requested_recipe.name);

    *overflow.entry(requested_recipe.name.to_string()).or_insert(0) += (needed * matching_recipe.output) - requested_recipe.output;

    let mut minerals = 0;
    for (name, qty) in parse_mat_line(&matching_recipe.materials) {
        let needed_qty = if *overflow.get(&name).unwrap_or(&0) > 0 {
            let over = *overflow.get(&name).unwrap();
            if qty * needed < over {
                *overflow.entry(name.clone()).or_insert(0) -= qty * needed;
                0
            } else {
                overflow.remove(&name);
                qty * needed - over    
            }
        } else {
            qty * needed
        };

        if needed_qty == 0 {
            continue;
        }
        let needed_recipe = Recipe {
            output: needed_qty,
            name: name,
            materials: "".to_owned()
        };
        // println!("PROCESSING RECIPE {} {}", needed_recipe.name, needed_recipe.output);

        minerals += solve_recipe(recipes, &needed_recipe, overflow);

    }

    minerals
}

#[test]
fn test_produce_one_fuel_1() {
    let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    let recipes = parse_input(input);
    let requested = Recipe {
        output: 1, 
        name: "FUEL".to_owned(),
        materials: "".to_owned()
    };
    let minerals = solve_recipe(&recipes, &requested, &mut HashMap::new());
    assert_eq!(165, minerals);
}

#[test]
fn test_produce_one_fuel_2() {
    let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    let recipes = parse_input(input);
    let requested = Recipe {
        output: 1, 
        name: "FUEL".to_owned(),
        materials: "".to_owned()
    };
    let minerals = solve_recipe(&recipes, &requested, &mut HashMap::new());
    assert_eq!(13312, minerals);
}

#[test]
fn test_produce_one_fuel_3() {
    let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    let recipes = parse_input(input);
    let requested = Recipe {
        output: 1, 
        name: "FUEL".to_owned(),
        materials: "".to_owned()
    };
    let minerals = solve_recipe(&recipes, &requested, &mut HashMap::new());
    assert_eq!(180697, minerals);
}

#[test]
fn test_produce_one_fuel_4() {
    let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    let recipes = parse_input(input);
    let requested = Recipe {
        output: 1, 
        name: "FUEL".to_owned(),
        materials: "".to_owned()
    };
    let minerals = solve_recipe(&recipes, &requested, &mut HashMap::new());
    assert_eq!(2210736, minerals);
}
