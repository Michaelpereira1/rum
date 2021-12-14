use std::{env, array, collections::HashMap, process::exit};
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
        if machine.counter as usize > machine.memory.get(&0).unwrap().len() {
            exit(0);
        }
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
            },
            o if o == Opcode::Store as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                
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
                    machine.registers[b as usize ] = machine.last_identifier + 1;
                    machine.last_identifier += 1;
                    machine.memory.entry(machine.registers[b as usize]).or_insert(vec![word_count; 0]);
                } else {
                    machine.registers[b as usize] = machine.identifiers.pop().unwrap();
                    machine.memory.entry(machine.registers[b as usize]).or_insert(vec![word_count; 0]);
                }
                
            },
            o if o == Opcode::UnmapSegment as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                let key = machine.registers[c as usize];
                machine.identifiers.push(key);
                machine.memory.remove(&key);
            },
            o if o == Opcode::Output as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                output(machine.registers[c as usize]);
                
            },
            o if o == Opcode::Input as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                
            },
            o if o == Opcode::LoadProgram as u32 => {
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                
            },
            o if o == Opcode::LoadValue as u32 => {
            format!("r{} := {};", get(&RL, inst), get(&VL, inst));
            let a = get(&RL, inst);
            let value = get(&VL, inst);
            
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



/*pub fn sign_extension(mut x: u32, num_bits: u32) -> u32{
    if ((x >> (num_bits - 1)) & 1) == 1{
        x |= 0x7FFFFFFF << num_bits;
    }
    return x;
}*/

pub fn conditional_move(mut a: u32, b: u32, c: u32) -> u32{
    if c != 0 {
        a = b
    }

    return a;

    
}

pub fn segmented_load(seg_id: u32, offset: usize, memory:HashMap<u32, Vec<u32>>) -> u32{
    return memory.get(&seg_id).unwrap()[offset];
}

pub fn segmented_store(a: u32, seg_id: u32, offset: usize, memory:HashMap<u32, Vec<u32>>){
    let mut word = memory.get(&seg_id).unwrap()[offset];
    word = a;
}

pub fn addition(b: u32, c: u32) -> u32{
    let base = 2;
    let base = u32::pow(base, 32);
    return (b + c) % base;    
}

pub fn multiplication(b: u32, c: u32) -> u32{
    let base = 2;
    //not sure if pow function fucks this up 
    let base = u32::pow(base, 32);
    return (b * c) % base;
}

pub fn division(b: u32, c: u32) -> u32{
    //integer division
    return b / c;
}

pub fn nand(b: u32, c: u32) -> u32{
    return !(b ^ c);
}

pub fn map_segment(){

}

pub fn output(c: u32){
    if c <= 255{
        println!("{}",c);
    }
}

pub fn input(){
    todo!();    
}

pub fn load_program(seg_id: u32, mut memory:HashMap<u32, &Vec<u32>>){
    if seg_id == 0{
        return;
    }
    let copied_segment = memory.get(&seg_id).unwrap();
    let new_segment = memory.insert(0, copied_segment);    
}

pub fn load_value(value: u32) -> u32{
    return value;
}

}