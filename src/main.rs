mod stack;
use stack::STACK;

pub enum InstructionType {
    PUSH,
    ADD,
    SUB,
    MUL,
    DIV,
    PRINTLN,
}


pub struct Instruction {
    inst_type: InstructionType,
    operand: Option<i32>,
}


const PROGRAM: [Instruction; 4] = [
    Instruction { inst_type: InstructionType::PUSH, operand: Some(5) },
    Instruction { inst_type: InstructionType::PUSH, operand: Some(4) },
    Instruction { inst_type: InstructionType::MUL, operand: None },
    Instruction { inst_type: InstructionType::PRINTLN, operand: None },
];


fn main() {
    let mut stack: STACK = STACK { ..Default::default() };

    for inst in PROGRAM {
        match inst.inst_type {
            InstructionType::PUSH => stack.push(inst.operand.unwrap()),
            InstructionType::ADD => {
                let a: i32 = stack.pop();
                let b: i32 = stack.pop();
                stack.push(a + b);
            },
            InstructionType::SUB => {
                let a: i32 = stack.pop();
                let b: i32 = stack.pop();
                stack.push(a - b);
            }
            InstructionType::MUL => {
                let a: i32 = stack.pop();
                let b: i32 = stack.pop();
                stack.push(a * b);
            },
            InstructionType::DIV => {
                let a: i32 = stack.pop();
                let b: i32 = stack.pop();
                stack.push(a / b);
            },
            InstructionType::PRINTLN => {
                let a: i32 = stack.pop();
                println!("{a}");
            }
        }
    }
}
