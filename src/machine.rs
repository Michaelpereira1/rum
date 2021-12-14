use std::{env, array, collections::HashMap, process::exit, io::{stdout, Write, stdin, Read}};
use crate::{machine};

type Umi = u32;
pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

fn mask(bits:u32) -> u32 { (1 << bits) - 1}

pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}

pub struct Machine {
    registers: [u32; 8],
    memory: HashMap<u32, Vec<u32>>,
    identifiers: Vec<u32>,
    last_identifier: u32,
    counter: u32,
}

pub fn machine(instructions: Vec<u32>) {
    let mut machine = Machine {
        registers: [0,0,0,0,0,0,0,0],
        memory: HashMap::new(),
        identifiers: vec![],
        last_identifier: 0,
        counter: 0,
    };
    machine.memory.insert(0, instructions);

    loop {
        let inst = machine.memory.get(&0).unwrap()[machine.counter as usize];
        machine.counter += 1;
        match get(&OP, inst) {
            o if o == Opcode::Cmov as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                machine.registers[a as usize] = conditional_move(machine.registers[a as usize], machine.registers[b as usize], machine.registers[c as usize]);
                
            },
            o if o == Opcode::Load as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                machine.registers[a as usize] = machine.memory.get(&machine.registers[b as usize]).unwrap()[machine.registers[c as usize]as usize];

            },
            o if o == Opcode::Store as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                machine.memory.get_mut(&machine.registers[a as usize]).unwrap()[machine.registers[b as usize] as usize] = machine.registers[c as usize];
                
            },
            o if o == Opcode::Add as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                machine.registers[a as usize] = addition(machine.registers[b as usize], machine.registers[c as usize]);
            },
            o if o == Opcode::Mul as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                machine.registers[a as usize] = multiplication(machine.registers[b as usize], machine.registers[c as usize]);
            },
            o if o == Opcode::Div as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                machine.registers[a as usize] = division(machine.registers[b as usize], machine.registers[c as usize]);
                
            },
            // possible enhancement: if RB == RC, complement RC
            o if o == Opcode::Nand as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                machine.registers[a as usize] = nand(machine.registers[b as usize], machine.registers[c as usize]);
            },
            o if o == Opcode::Halt as u32 => {
                exit(0);
            },
            o if o == Opcode::MapSegment as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                let word_count = machine.registers[c as usize];
                if machine.identifiers.is_empty() {
                    machine.last_identifier += 1;
                    machine.registers[b as usize ] = machine.last_identifier;
                    machine.memory.entry(machine.registers[b as usize]).or_insert(vec![0; word_count as usize]);
                } else {
                    machine.registers[b as usize] = machine.identifiers.pop().unwrap();
                    machine.memory.entry(machine.registers[b as usize]).or_insert(vec![0; word_count as usize]);
                    
                }
                
            },
            o if o == Opcode::UnmapSegment as u32 => {
                let c = get(&RC, inst);
                let key = machine.registers[c as usize];
                machine.identifiers.push(key);
                machine.memory.remove(&key);
            },
            o if o == Opcode::Output as u32 => {
                let c = get(&RC, inst);
                output(machine.registers[c as usize]);
                
            },
            o if o == Opcode::Input as u32 => {
                let c = get(&RC, inst);
                match stdin().bytes().next() {
                    Some(value) => {
                        machine.registers[c as usize] = value.unwrap() as u32;
                    }
                    None => machine.registers[c as usize]  = !0 as u32,
                }
                
            },

            o if o == Opcode::LoadProgram as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                let duplicate = machine.memory.get(&machine.registers[b as usize]).unwrap();
                machine.memory.insert(0, duplicate.to_vec());
                machine.counter = machine.registers[c as usize];
            },

            o if o == Opcode::LoadValue as u32 => {
                let a = get(&RL, inst);
                let value = get(&VL, inst);
                machine.registers[a as usize] = value;
            
            },
        
            _ => {
                format!(".data 0x{:x}", inst);
            }
        }
}
#[derive(Debug, PartialEq, Copy, Clone)]
enum Opcode {
    Cmov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
}


pub fn conditional_move(mut a: u32, b: u32, c: u32) -> u32{
    if c != 0 {
        a = b
    }

    return a;
}

pub fn addition(b: u32, c: u32) -> u32{
    let base = 2;
    let base = usize::pow(base, 32);

    let answer = (b as usize + c as usize) % base;
    return answer as u32;    
}

pub fn multiplication(b: u32, c: u32) -> u32{
    let base = 2;
    let base = usize::pow(base, 32);
    let answer = (b as usize * c as usize) % base;
    return answer as u32;
}

pub fn division(b: u32, c: u32) -> u32{
    let answer = b / c;
    return answer;
}

pub fn nand(b: u32, c: u32) -> u32{
    return !(b & c);
}


pub fn output(c: u32){
    if c <= 255{
        print!("{}", c as u8 as char);
    }
}
}