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



/*pub fn sign_extension(mut x: u32, num_bits: u32) -> u32{
    if ((x >> (num_bits - 1)) & 1) == 1{
        x |= 0x7FFFFFFF << num_bits;
    }
    return x;
}*/

pub fn conditional_move(mut a: u32, b: u32, c: u32){
    if c != 0 {
        a = b
    }
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




