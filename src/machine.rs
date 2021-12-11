use std::{env, array, collections::HashMap};
use crate::rumdis;

pub fn machine(instructions: Vec<u32>) {
    let mut registers: [u32; 8] = [0,0,0,0,0,0,0,0];
    let mut memory: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut identifiers: Vec<u32> = vec![];
    let mut counter:u32 = 0;
    memory.insert(0, instructions);

    loop {
        rumdis::disassemble(memory.get(&0).unwrap()[counter as usize]);
    }


}

