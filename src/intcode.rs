pub type Program = Vec<i64>;
pub type InputGenerator = Box<dyn Fn()-> i64>;
pub type OutputHandler = Box<dyn Fn(i64)>;

// Helper functions to plug in IO when it's not needed
pub fn ZeroInputGenerator() -> InputGenerator {
    Box::new(|| 0)
}

pub fn NullHandler() -> OutputHandler {
    Box::new(|output: i64| ())
}

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

#[derive(PartialEq, Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2
}

#[derive(Copy, Clone)]
enum ParamPos {
    Param1 = 1,
    Param2 = 2,
    Param3 = 3
}

fn check_resize(program: &mut Program, size: usize) {
    if program.len() <= size {
        program.resize(size + 3, 0);
    }
}

fn run_opcode_add(program: &mut Program, index: usize, relbase: i64) -> usize {
    let param1 = get_parameter_1(program, index, relbase);
    let param2 = get_parameter_2(program, index, relbase);
    let param3 = get_address_3(&program, index, relbase);
    check_resize(program, param3);
    program[param3] = param1 + param2;
    index + 4
}

fn run_opcode_mult(program: &mut Program, index: usize, relbase: i64) -> usize {
    let param1 = get_parameter_1(program, index, relbase);
    let param2 = get_parameter_2(program, index, relbase);
    let param3 = get_address_3(&program, index, relbase);
    check_resize(program, param3);
    program[param3] = param1 * param2;
    index + 4
}

fn run_opcode_input(program: &mut Program, index: usize, relbase: i64, input: &InputGenerator) -> usize {
    let param1 = get_address_1(program, index, relbase) as usize;
    let mode = parse_mode(program[index], ParamPos::Param1);

    check_resize(program, param1);
    program[param1] = input();
    index + 2
}

fn run_opcode_output(program: &mut Program, index: usize, relbase: i64, output: &OutputHandler) -> usize {
    let param1 = get_parameter_1(program, index, relbase);
    output(param1);
    index + 2
}

fn run_opcode_jump_if_true(program: &mut Program, index: usize, relbase: i64) -> usize {
    let param1 = get_parameter_1(program, index, relbase);
    let param2 = get_parameter_2(program, index, relbase) as usize;
    if param1 != 0 {
        check_resize(program, param2);
        param2
    } else {
        index + 3
    }
}

fn run_opcode_jump_if_false(program: &mut Program, index: usize, relbase: i64) -> usize {
    let param1 = get_parameter_1(program, index, relbase);
    let param2 = get_parameter_2(program, index, relbase) as usize;
    if param1 == 0 {
        check_resize(program, param2);
        param2
    } else {
        index + 3
    }
}

fn run_opcode_less_than(program: &mut Program, index: usize, relbase: i64) -> usize {
    let param1 = get_parameter_1(program, index, relbase);
    let param2 = get_parameter_2(program, index, relbase);
    let param3 = get_address_3(&program, index, relbase) as usize;
    check_resize(program, param3);
    program[param3] = if param1 < param2 { 1 } else { 0 };
    index + 4
}

fn run_opcode_equals(program: &mut Program, index: usize, relbase: i64) -> usize {
    let param1 = get_parameter_1(program, index, relbase);
    let param2 = get_parameter_2(program, index, relbase);
    let param3 = get_address_3(&program, index, relbase) as usize;
    check_resize(program, param3);
    program[param3] = if param1 == param2 { 1 } else { 0 };
    index + 4
}

fn run_opcode_change_relbase(program: &mut Program, index: usize, relbase: &mut i64) -> usize {
    let param1 = get_parameter_1(program, index, *relbase);
    *relbase += param1;
    index + 2
}

fn get_address_1(program: &Program, index: usize, relbase: i64) -> usize {
    get_address_x(program, index, ParamPos::Param1, relbase)
}

/*
fn get_address_2(program: &Program, index: usize, relbase: i64) -> usize {
    get_address_x(program, index, ParamPos::Param2, relbase)
}
*/

fn get_address_3(program: &Program, index: usize, relbase: i64) -> usize {
    get_address_x(program, index, ParamPos::Param3, relbase)
}

fn get_address_x(program: &Program, index: usize, ppos: ParamPos, relbase: i64) -> usize {
    let mode = parse_mode(program[index], ppos);
    let param_index = ppos as usize;

    match mode {
        ParameterMode::Immediate => index + param_index,
        ParameterMode::Position => program[index + param_index] as usize,
        ParameterMode::Relative => (program[index + param_index] + relbase) as usize
    }
}

fn get_parameter_1(program: &mut Program, index: usize, relbase: i64) -> i64 {
    get_parameter_x(program, index, ParamPos::Param1, relbase)
}

fn get_parameter_2(program: &mut Program, index: usize, relbase: i64) -> i64 {
    get_parameter_x(program, index, ParamPos::Param2, relbase)
}

/*
fn get_parameter_3(program: &Program, index: usize, relbase: i64) -> i64 {
    get_parameter_x(program, index, ParamPos::Param3, relbase)
}
*/

fn get_parameter_x(program: &mut Program, index: usize, ppos: ParamPos, relbase: i64) -> i64 {
    let mode = parse_mode(program[index], ppos);
    let param_index = ppos as usize;

    match mode {
        ParameterMode::Immediate => program[index + param_index],
        ParameterMode::Position => {
            let idx = program[index + param_index] as usize;
            check_resize(program, idx);
            program[idx]
        },
        ParameterMode::Relative => {
            let idx = (program[index + param_index] + relbase) as usize;
            check_resize(program, idx);
            program[idx]
        }
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

fn parse_mode(value: i64, ppos: ParamPos) -> ParameterMode {
    let strop = (value + 100_000).to_string();
    let mode_pos = match ppos {
        ParamPos::Param1 => 3,
        ParamPos::Param2 => 2,
        ParamPos::Param3 => 1
    };

    let mode = match strop.chars().nth(mode_pos).unwrap() {
        '0' => ParameterMode::Position,
        '1' => ParameterMode::Immediate,
        '2' => ParameterMode::Relative,
        _ => panic!("Unknown mode")
    };
    mode
}

pub fn run_intcode(program: &mut Vec<i64>, input_generator: &InputGenerator, output_handler: &OutputHandler) {
    let mut index = 0;
    let mut relbase = 0i64;

    loop {
        let opcode = parse_opcode(program[index]);
        index = match opcode {
            Opcodes::Add => run_opcode_add(program, index, relbase),
            Opcodes::Mult => run_opcode_mult(program, index, relbase),
            Opcodes::Input => run_opcode_input(program, index, relbase, input_generator),
            Opcodes::Output => run_opcode_output(program, index, relbase, output_handler),
            Opcodes::PutIfZero => run_opcode_jump_if_true(program, index, relbase),
            Opcodes::PutIfNotZero => run_opcode_jump_if_false(program, index, relbase),
            Opcodes::Put1IfP1LessThanP2 => run_opcode_less_than(program, index, relbase),
            Opcodes::Put1IfP1EqualsP2 => run_opcode_equals(program, index, relbase),
            Opcodes::ChangeRelBase => run_opcode_change_relbase(program, index, &mut relbase),
            Opcodes::Break => std::usize::MAX
        };

        if index == std::usize::MAX {
            break;
        }
    }
}
