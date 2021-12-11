use std::process::exit;
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

pub fn disassemble(inst: Umi) {
    match get(&OP, inst) {
    o if o == Opcode::Cmov as u32 => {
        let c = get(&RC, inst);
        let a = get(&RA, inst);
        let b = get(&RB, inst);
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
    },
    o if o == Opcode::Mul as u32 => {
        let c = get(&RC, inst);
        let a = get(&RA, inst);
        let b = get(&RB, inst);
    },
    o if o == Opcode::Div as u32 => {
        let c = get(&RC, inst);
        let a = get(&RA, inst);
        let b = get(&RB, inst);
    },
    // possible enhancement: if RB == RC, complement RC
    o if o == Opcode::Nand as u32 => {
        let c = get(&RC, inst);
        let a = get(&RA, inst);
        let b = get(&RB, inst);
    },
    o if o == Opcode::Halt as u32 => {
       exit(0);
    },
    o if o == Opcode::MapSegment as u32 => {
        let c = get(&RC, inst);
        let a = get(&RA, inst);
        let b = get(&RB, inst);
    },
    o if o == Opcode::UnmapSegment as u32 => {
        let c = get(&RC, inst);
        let a = get(&RA, inst);
        let b = get(&RB, inst);
    },
    o if o == Opcode::Output as u32 => {
        let c = get(&RC, inst);
        let a = get(&RA, inst);
        let b = get(&RB, inst);
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
       //format!("r{} := {};", get(&RL, inst), get(&VL, inst))
    },

    _ => {
      // format!(".data 0x{:x}", inst)
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