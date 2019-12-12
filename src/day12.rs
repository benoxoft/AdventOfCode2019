
type Pairs = Vec<(Vector, Vector)>;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vector {
    x: isize,
    y: isize,
    z: isize
}

#[aoc(day12, part1)]
fn find_solution(input: &str) -> isize {
    let mut m1 = Vector{x: 1, y: 4, z: 4};
    let mut m2 = Vector{x: -4, y: -1, z: 19};
    let mut m3 = Vector{x: -15, y: -14, z: 12};
    let mut m4 = Vector{x: -17, y: 1, z: 10};

    let mut v1 = Vector{x: 0, y: 0, z: 0};
    let mut v2 = Vector{x: 0, y: 0, z: 0};
    let mut v3 = Vector{x: 0, y: 0, z: 0};
    let mut v4 = Vector{x: 0, y: 0, z: 0};

    println!("{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);
    println!("{:?}", m4);

    for i in 0..1000 {
        let changes = gravity(&expand_moons(m1, m2, m3, m4), &expand_moons(v1, v2, v3, v4));
        let (c1, c2, c3, c4) = shrink_moons(changes);
        v1 = new_vel_vec(v1, c1);
        v2 = new_vel_vec(v2, c2);
        v3 = new_vel_vec(v3, c3);
        v4 = new_vel_vec(v4, c4);

        m1 = new_vel_vec(m1, v1);
        m2 = new_vel_vec(m2, v2);
        m3 = new_vel_vec(m3, v3);
        m4 = new_vel_vec(m4, v4);       
    }
    println!("\n{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);
    println!("{:?}", m4);
    let e = sum(m1) * sum(v1) + sum(m2) * sum(v2) + sum(m3) * sum(v3) + sum(m4) * sum(v4);
    e
}

#[aoc(day12, part2)]
fn find_solution2(input: &str) -> u128 {
    let im1 = Vector{x: 1, y: 4, z: 4};
    let im2 = Vector{x: -4, y: -1, z: 19};
    let im3 = Vector{x: -15, y: -14, z: 12};
    let im4 = Vector{x: -17, y: 1, z: 10};

    let mut m1 = Vector{x: 1, y: 4, z: 4};
    let mut m2 = Vector{x: -4, y: -1, z: 19};
    let mut m3 = Vector{x: -15, y: -14, z: 12};
    let mut m4 = Vector{x: -17, y: 1, z: 10};

    let iv1 = Vector{x: 0, y: 0, z: 0};
    let iv2 = Vector{x: 0, y: 0, z: 0};
    let iv3 = Vector{x: 0, y: 0, z: 0};
    let iv4 = Vector{x: 0, y: 0, z: 0};

    let mut v1 = Vector{x: 0, y: 0, z: 0};
    let mut v2 = Vector{x: 0, y: 0, z: 0};
    let mut v3 = Vector{x: 0, y: 0, z: 0};
    let mut v4 = Vector{x: 0, y: 0, z: 0};

    let mut count = 0u128;

    loop {
        let changes = gravity(&expand_moons(m1, m2, m3, m4), &expand_moons(v1, v2, v3, v4));
        let (c1, c2, c3, c4) = shrink_moons(changes);
        v1 = new_vel_vec(v1, c1);
        v2 = new_vel_vec(v2, c2);
        v3 = new_vel_vec(v3, c3);
        v4 = new_vel_vec(v4, c4);
    
        m1 = new_vel_vec(m1, v1);
        m2 = new_vel_vec(m2, v2);
        m3 = new_vel_vec(m3, v3);
        m4 = new_vel_vec(m4, v4);
        count += 1;
        if count % 1_000_000 == 0 {
            println!("{}", count);
        }
        if m1.z == im1.z && v1.z == iv1.z &&
           m2.z == im2.z && v2.z == iv2.z &&
           m3.z == im3.z && v3.z == iv3.z &&
           m4.z == im4.z && v4.z == iv4.z {
            println!("ANSWER: {}", count);
            break;
        } 
        //&& m2 == im2 && m3 == im3 && m4 == im4 &&  && v2 == iv2 && v3 == iv3 && v4 == iv4 {
            
//            break;
//}
    }
    count
}

fn generate_pull_vector(moon1: Vector, moon2: Vector) -> Vector {
    Vector {
        x: {
            if moon1.x > moon2.x {
                -1
            } else if moon1.x < moon2.x {
                1
            } else {
                0
            }
        },
        y: {
            if moon1.y > moon2.y {
                -1
            } else if moon1.y < moon2.y {
                1
            } else {
                0
            }
        },
        z: {
            if moon1.z > moon2.z {
                -1
            } else if moon1.z < moon2.z {
                1
            } else {
                0
            }
        }
    }
}

fn new_vel_vec(vel: Vector, chg: Vector) -> Vector {
    Vector {
        x: vel.x + chg.x,
        y: vel.y + chg.y,
        z: vel.z + chg.z
    }
}

fn gravity(moon_pairs: &Pairs, vel_pairs: &Pairs) -> Pairs {
    let mut changes = Vec::new();

    for i in 0..moon_pairs.len() {
        let (moon1, moon2) = moon_pairs[i];
        let chg1 = generate_pull_vector(moon1, moon2);
        let chg2 = generate_pull_vector(moon2, moon1);
        changes.push((chg1, chg2));
    };
    changes
}

fn expand_moons(m1: Vector, m2: Vector, m3: Vector, m4: Vector) -> Pairs {
    vec![
        (m1, m2), 
        (m1, m3), 
        (m1, m4), 
        (m2, m3),
        (m2, m4),
        (m3, m4)
    ]
}

fn shrink_moons(pairs: Pairs) -> (Vector, Vector, Vector, Vector) {
    let v1 = new_vel_vec(new_vel_vec(pairs[0].0, pairs[1].0), pairs[2].0);
    let v2 = new_vel_vec(new_vel_vec(pairs[0].1, pairs[3].0), pairs[4].0);
    let v3 = new_vel_vec(new_vel_vec(pairs[1].1, pairs[3].1), pairs[5].0);
    let v4 = new_vel_vec(new_vel_vec(pairs[2].1, pairs[4].1), pairs[5].1);
    (v1, v2, v3, v4)
}

fn sum(v: Vector) -> isize {
    v.x.abs() + v.y.abs() + v.z.abs()
}
