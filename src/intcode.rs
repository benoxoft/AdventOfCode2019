type Program = Vec<i64>;

enum Opcodes {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    PutIfZero = 5,
    PutIfNotZero = 6,
    Put1IfP1LessThanP2 = 7,
    Put1IfP1EqualsP2 = 8,
    ChangeRelBase = 9,
    Break = 99
}

enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2
}

fn check_resize(program: &mut Program, size: usize) {
    if program.len() < size {
        program.resize(size+2, 0);
    }
}

fn run_opcode_add(program: &mut Program, index: usize, relbase: i64) -> usize {
    let param1 = get_parameter_X(&program, index, 1, relbase);
    let param2 = get_parameter_X(&program, index, 2, relbase);
    let param3 = get_address_X(&program, index, 3, relbase);
    check_resize(program, param3);
    println!("ADD {} {} {}", param3, param1, param2);
    program[param3] = param1 + param2;
    index + 4
}

fn run_opcode_mult(program: &mut Program, index: usize, relbase: i64) -> usize {
    let param1 = get_parameter_X(&program, index, 1, relbase);
    let param2 = get_parameter_X(&program, index, 2, relbase);
    let param3 = get_address_X(&program, index, 3, relbase);
    check_resize(program, param3);
    program[param3] = param1 * param2;
    index + 4
}

fn run_opcode_input(program: &mut Program, index: usize, relbase: i64, input: i64) -> usize {
    let param1 = get_parameter_X(&program, index, 1, relbase) as usize;
    check_resize(program, param1);
    program[param1] = input;
    index + 2
}

fn get_address_X(program: &Program, index: usize, x: usize, relbase: i64) -> usize {
    let mode = parse_mode(program[index], x);
    match mode {
        ParameterMode::Immediate => index + x,
        ParameterMode::Position => program[index + x] as usize,
        ParameterMode::Relative => index + x + relbase as usize
    }
}

fn get_parameter_X(program: &Program, index: usize, x: usize, relbase: i64) -> i64 {
    let mode = parse_mode(program[index], x);
    match mode {
        ParameterMode::Immediate => program[index + x],
        ParameterMode::Position => program[program[index + x] as usize],
        ParameterMode::Relative => program[index + x + relbase as usize]
    }
}

fn parse_opcode(value: i64) -> Opcodes {
    let strop = (value + 100_000).to_string();
    let opcode = match strop[4..6].parse::<i64>().unwrap() {
        1 => Opcodes::Add,
        2 => Opcodes::Mult,
        3 => Opcodes::Input,
        4 => Opcodes::Output,
        5 => Opcodes::PutIfZero,
        6 => Opcodes::PutIfNotZero,
        7 => Opcodes::Put1IfP1LessThanP2,
        8 => Opcodes::Put1IfP1EqualsP2,
        9 => Opcodes::ChangeRelBase,
        99 => Opcodes::Break,
        _ => panic!("Unknown opcode {}", strop)
    };
    opcode
}

fn parse_mode(value: i64, pos: usize) -> ParameterMode {
    let strop = (value + 100_000).to_string();
    let mode = match strop.chars().nth(pos).unwrap() {
        '0' => ParameterMode::Position,
        '1' => ParameterMode::Immediate,
        '2' => ParameterMode::Relative,
        _ => panic!("Unknown mode")
    };
    mode
}

pub fn run_intcode(program: &mut Vec<i64>, input_generator: usize, output_handler: usize) {
    let mut index = 0;
    let mut relbase = 0i64;

    loop {
        let opcode = parse_opcode(program[index]);

        index = match opcode {
            Opcodes::Add => run_opcode_add(program, index, relbase),
            Opcodes::Mult => run_opcode_mult(program, index, relbase),
            Opcodes::Input => 0,
            Opcodes::Output => 0,
            Opcodes::PutIfZero => 0,
            Opcodes::PutIfNotZero => 0,
            Opcodes::Put1IfP1LessThanP2 => 0,
            Opcodes::Put1IfP1EqualsP2 => 0,
            Opcodes::ChangeRelBase => 0,
            Opcodes::Break => std::usize::MAX
        };

        if index == std::usize::MAX {
            break;
        }
    }
}
