use std::{collections::HashMap, process::exit, io::{stdout, Write, stdin, Read}};

type Umi = u32;
//Used for the bit shifting operations to extract the opcode and register numbers from words
pub struct Field {
    width: u32,
    lsb: u32,
}

//Constants used to mark where in the 32 bit word certain segments lay.
static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};
//2^32 base to be modded by in certain calculations.
static BASE: usize = usize::pow(2, 32);

fn mask(bits:u32) -> u32 { (1 << bits) - 1}

//Gets the data located in the 32 bit instruction word designated by field.
pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}

//struct for the machine and all its parts.
pub struct Machine {
    registers: [u32; 8],
    memory: HashMap<u32, Vec<u32>>,
    identifiers: Vec<u32>,
    last_identifier: u32,
    counter: u32,
}

/*The initilization of the machine consists of:
 8 32 bit registers stored in a vector, all initilized to 0.
 Segmented memory that consists of a Hashmap with 32 bit words as keys or memory segments, and vectors of 32 bit words as values or memory.
 A vector of already-used 32 bit segment identifiers.
 A program counter that is used to point to the current instruction.
*/
pub fn machine(instructions: Vec<u32>) {
    let mut machine = Machine {
        registers: [0,0,0,0,0,0,0,0],
        memory: HashMap::new(),
        identifiers: vec![],
        last_identifier: 0,
        counter: 0,
    };
    //Load the instruction set into the 0 segment within memory.
    machine.memory.insert(0, instructions);

    
    //Machine execution, continues until Fail state, or program halt.
    loop {
        //Retrieve the current memory from memory segment 0 using the program counter.
        let inst = machine.memory.get(&0).unwrap()[machine.counter as usize];
        //Incriment the program counter.
        machine.counter += 1;

        //Match on the opcode to determine which instruction should be.
        match get(&OP, inst) {
            o if o == Opcode::Cmov as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                //Conditional Move, if the contents of register c are not 0, than the contents of register b are placed in register a.
                if machine.registers[c as usize] != 0 {
                    machine.registers[a as usize] = machine.registers[b as usize];
                }
                
            },
            
            o if o == Opcode::Load as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                //Segment Load, retrieves the data stored in the segment with the identifier located in register b, and the index of register c, and places it in register a.
                machine.registers[a as usize] = machine.memory.get(&machine.registers[b as usize]).unwrap()[machine.registers[c as usize]as usize];

            },
            o if o == Opcode::Store as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                //Segment store, places the data in register c into segment with the identifier located in register a, and index in register b.
                machine.memory.get_mut(&machine.registers[a as usize]).unwrap()[machine.registers[b as usize] as usize] = machine.registers[c as usize];
                
            },
            o if o == Opcode::Add as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                //Add, adds the contents of register b and register c and places them in register a.
                machine.registers[a as usize] = ((machine.registers[b as usize] as usize + machine.registers[c as usize] as usize) % BASE) as u32;

            },
            o if o == Opcode::Mul as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                //Multiplication, multiplies the contents of register b and c and places them in register a.
                machine.registers[a as usize] = ((machine.registers[b as usize] as usize * machine.registers[c as usize] as usize) % BASE) as u32;
            },
            o if o == Opcode::Div as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst);
                //Division, does intiger division dividing the contents of register b by c and places the outcome in register a.
                machine.registers[a as usize] = (machine.registers[b as usize] / machine.registers[c as usize]) as u32;
                
            },
            o if o == Opcode::Nand as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                let a = get(&RA, inst);
                let b = get(&RB, inst); 
                //Bitwise nand, places the outcome of b nand c into register a.
                machine.registers[a as usize] = !(machine.registers[b as usize] & machine.registers[c as usize]);
            },
            //Halt, exits the program.
            o if o == Opcode::Halt as u32 => {
                exit(0);
            },
            o if o == Opcode::MapSegment as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                let b = get(&RB, inst);
                //Retrieves the size of the memory segment from register c.
                let word_count = machine.registers[c as usize];
                //If there are no unusued previous segment identifiers create a unique identifer and store it in register b,
                //then create a new segment using that identifier with a memory sized to word_count.
                if machine.identifiers.is_empty() {
                    machine.last_identifier += 1;
                    machine.registers[b as usize ] = machine.last_identifier;
                    machine.memory.entry(machine.registers[b as usize]).or_insert(vec![0; word_count as usize]);
                //Else, pop one of the previous segment identifiers and do the same.
                } else {
                    machine.registers[b as usize] = machine.identifiers.pop().unwrap();
                    machine.memory.entry(machine.registers[b as usize]).or_insert(vec![0; word_count as usize]);
                    
                }
                
            },
            o if o == Opcode::UnmapSegment as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                //Get the segment identifier located in register C, push it to the previously used identifier vector, and then remove the key value pair from the hashmap/memory.
                let key = machine.registers[c as usize];
                machine.identifiers.push(key);
                machine.memory.remove(&key);
            },
            o if o == Opcode::Output as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                //Outputs the contents of register c as a u8.
                stdout().write(&[machine.registers[c as usize] as u8]).unwrap();
                
            },
            o if o == Opcode::Input as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                //Recieves input from stdin.
                match stdin().bytes().next() {
                    Some(value) => {
                        machine.registers[c as usize] = value.unwrap() as u32;
                    }
                    None => machine.registers[c as usize]  = !0 as u32,
                }
                
            },

            o if o == Opcode::LoadProgram as u32 => {
                //retrieves the correct registers using the get helper function.
                let c = get(&RC, inst);
                let b = get(&RB, inst);
                //Load program, if the contents of register b are not 0, than duplicate the memory segment at the identifier in register b, and replace memory segment 0 with that memory segment.
                if machine.registers[b as usize] != 0 {
                    let duplicate = machine.memory.get(&machine.registers[b as usize]).unwrap();
                    machine.memory.insert(0, duplicate.to_vec());
                }
                //Update the program counter with the contents of register c.
                machine.counter = machine.registers[c as usize];
                
            },

            o if o == Opcode::LoadValue as u32 => {
                //retrieves the correct registers using the get helper function. Also retrieves the value located in the instruction word.
                let a = get(&RL, inst);
                let value = get(&VL, inst);
                //Stores this value in register a.
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
}