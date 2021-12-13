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

pub fn conditional_move(mut a: u32, mut b: u32, mut c: u32) -> u32{
    if c != 0 {
        return b;
    }else{
        return a;
    }
}

pub fn segmented_load(id: u32, offset: u32){
    todo!();     

}

pub fn addition(mut b: u32, mut c: u32) -> u32{
    let base = 2;
    //not sure if pow function fucks this up 
    let base = u32::pow(base, 32);
    return (b + c) % base;    
}

pub fn multiplication(mut b: u32, mut c: u32) -> u32{
    let base = 2;
    //not sure if pow function fucks this up 
    let base = u32::pow(base, 32);
    return (b * c) % base;
}

pub fn division(mut b: u32, mut c: u32) -> u32{
    //integer division
    return b / c;
}

pub fn nand(mut b: u32, mut c: u32) -> u32{
    return !(b ^ c);
}


pub fn output(mut c: u32){
    if c <= 255{
        println!("{}",c);
    }
}

pub fn input(){
    todo!();    
}

pub fn load_value(value: u32){
    return value;
}
