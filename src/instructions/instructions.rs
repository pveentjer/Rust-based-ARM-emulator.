

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq)]
pub enum Opcode {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    INC,
    DEC,
    LOAD,
    STORE,
    NOP,
    PRINTR,
    MOV,
    JNZ,
    JZ,
    PUSH,
    POP,
    NEG,
    AND,
    OR,
    XOR,
    NOT,
}

pub(crate) fn is_control(opcode: Opcode) -> bool {
    return match opcode {
        Opcode::JNZ => true,
        _ => false,
    };
}

pub(crate) fn mnemonic(opcode: Opcode) -> &'static str {
    match opcode {
        Opcode::ADD => "ADD",
        Opcode::SUB => "SUB",
        Opcode::MUL => "MUL",
        Opcode::DIV => "DIV",
        Opcode::MOD => "MOD",
        Opcode::NEG => "NEG",
        Opcode::LOAD => "LOAD",
        Opcode::STORE => "STORE",
        Opcode::NOP => "NOP",
        Opcode::INC => "INC",
        Opcode::DEC => "DEC",
        Opcode::PRINTR => "PRINTR",
        Opcode::MOV => "PRINTR",
        Opcode::JNZ => "JNZ",
        Opcode::JZ => "JZ",
        Opcode::PUSH => "PUSH",
        Opcode::POP => "POP",
        Opcode::AND => "AND",
        Opcode::OR => "OR",
        Opcode::XOR => "XOR",
        Opcode::NOT => "NOT",
    }
}

pub(crate) fn get_opcode(name: &str) -> Option<Opcode> {
    match name {
        "ADD" => Some(Opcode::ADD),
        "SUB" => Some(Opcode::SUB),
        "MUL" => Some(Opcode::MUL),
        "DIV" => Some(Opcode::DIV),
        "MOD" => Some(Opcode::MOD),
        "NEG" => Some(Opcode::NEG),
        "LOAD" => Some(Opcode::LOAD),
        "STORE" => Some(Opcode::STORE),
        "NOP" => Some(Opcode::NOP),
        "INC" => Some(Opcode::INC),
        "DEC" => Some(Opcode::DEC),
        "PRINTR" => Some(Opcode::PRINTR),
        "MOV" => Some(Opcode::MOV),
        "JNZ" => Some(Opcode::JNZ),
        "JZ" => Some(Opcode::JZ),
        "PUSH" => Some(Opcode::PUSH),
        "POP" => Some(Opcode::POP),
        "AND" => Some(Opcode::AND),
        "OR" => Some(Opcode::OR),
        "XOR" => Some(Opcode::XOR),
        "NOT" => Some(Opcode::NOT),
        _ => None,
    }
}

pub(crate) const NOP: Instr = create_NOP(-1);

pub(crate) type RegisterType = u16;
pub(crate) type MemoryAddressType = u64;
pub(crate) type CodeAddressType = u64;
pub(crate) type WordType = i32;

// The InstrQueue sits between frontend and backend
// The 'a lifetime specifier tells that the instructions need to live as least as long
// as the instruction queue.
pub(crate) struct InstrQueue {
    capacity: u16,
    head: u64,
    tail: u64,
    instructions: Vec<Rc<Instr>>,
}

impl InstrQueue {
    pub fn new(capacity: u16) -> Self {
        let mut instructions = Vec::with_capacity(capacity as usize);
        for _ in 0..capacity {
            instructions.push(Rc::new(NOP));
        }

        InstrQueue {
            capacity,
            head: 0,
            tail: 0,
            instructions,
        }
    }

    pub fn size(&self) -> u16 {
        (self.tail - self.head) as u16
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        self.size() == self.capacity
    }

    pub fn enqueue(&mut self, instr: Rc<Instr>) {
        assert!(!self.is_full(), "Can't enqueue when InstrQueue is empty.");

        let index = (self.tail % self.capacity as u64) as usize;
        self.instructions[index] = instr;
        self.tail += 1;
    }

    pub fn dequeue(&mut self) {
        assert!(!self.is_empty(), "Can't dequeue when InstrQueue is empty.");
        self.head += 1;
    }

    pub fn peek(&self) -> Rc<Instr> {
        assert!(!self.is_empty(), "Can't peek when InstrQueue is empty.");

        let index = (self.head % self.capacity as u64) as usize;
        return Rc::clone(&self.instructions[index]);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum OpType {
    REGISTER,
    MEMORY,
    CONSTANT,
    UNUSED,
    CODE,
}

// The maximum number of source (input) operands for an instruction.
pub(crate) const MAX_SOURCE_COUNT: u8 = 2;

pub(crate) struct Instr {
    pub(crate) cycles: u8,
    pub(crate) opcode: Opcode,
    pub(crate) sink: Operand,
    pub(crate) source_cnt: u8,
    pub(crate) source: [Operand; MAX_SOURCE_COUNT as usize],
    pub(crate) line: i32,
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", mnemonic(self.opcode))?;

        for k in 0..self.source_cnt {
            write!(f, " {}", self.source[k as usize])?;
        }

        if self.sink.op_type != OpType::UNUSED {
            write!(f, " {}", self.sink)?;
        }

        if self.line > 0 {
            write!(f, " line={}", self.line)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Operand {
    pub(crate) op_type: OpType,
    pub(crate) union: OpUnion,
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.union {
            OpUnion::Register(reg) => write!(f, "R{}", reg),
            OpUnion::Memory(mem) => write!(f, "[{}]", mem),
            OpUnion::Code(code) => write!(f, "[{}]", code),
            OpUnion::Constant(val) => write!(f, "{}", val),
            OpUnion::Unused => write!(f, "unused"),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum OpUnion {
    Register(RegisterType),
    Memory(MemoryAddressType),
    Code(CodeAddressType),
    Constant(WordType),
    Unused,
}

impl OpUnion {
    pub(crate) fn get_register(&self) -> RegisterType {
        match self {
            OpUnion::Register(reg) => *reg,
            _ => panic!(),
        }
    }

    pub(crate) fn get_constant(&self) -> WordType {
        match self {
            OpUnion::Constant(constant) => *constant,
            _ => panic!(),
        }
    }

    pub(crate) fn get_code_address(&self) -> CodeAddressType {
        match self {
            OpUnion::Code(constant) => *constant,
            _ => panic!(),
        }
    }

    pub(crate) fn get_memory_addr(&self) -> MemoryAddressType {
        match self {
            OpUnion::Memory(addr) => *addr,
            _ => panic!(),
        }
    }

    // Implement similar functions for other variants as needed
}

pub(crate) struct Data {
    pub(crate) value: WordType,
    pub(crate) offset: u64,
}

pub(crate) struct Program {
    pub(crate) data_items: HashMap::<String, Rc<Data>>,
    pub(crate) code: Vec<Rc<Instr>>,
}

impl Program {
    pub fn new(code: Vec<Rc<Instr>>, data_items: HashMap::<String, Rc<Data>>) -> Self {
        Self { code, data_items }
    }

    pub fn get_instr(&self, pos: usize) -> Rc<Instr> {
        Rc::clone(&self.code[pos])
    }
}

pub(crate) fn create_reg_bi_Instr(opcode: Opcode, src_1: RegisterType, src_2: RegisterType, sink: RegisterType, line: i32) -> Instr {
    Instr {
        cycles: 1,
        opcode: opcode,
        source_cnt: 2,
        source: [
            Operand { op_type: OpType::REGISTER, union: OpUnion::Register(src_1) },
            Operand { op_type: OpType::REGISTER, union: OpUnion::Register(src_2) }],
        sink: Operand { op_type: OpType::REGISTER, union: OpUnion::Register(sink) },
        line,
    }
}

pub(crate) fn create_reg_mono_Instr(opcode: Opcode, src: RegisterType, dst: RegisterType, line: i32) -> Instr {
    Instr {
        cycles: 1,
        opcode,
        source_cnt: 1,
        source: [
            Operand { op_type: OpType::REGISTER, union: OpUnion::Register(src) },
            Operand { op_type: OpType::UNUSED, union: OpUnion::Unused }
        ],
        sink: Operand { op_type: OpType::REGISTER, union: OpUnion::Register(src) },
        line,
    }
}

pub(crate) fn create_LOAD(addr: MemoryAddressType, sink: RegisterType, line: i32) -> Instr {
    Instr {
        cycles: 1,
        opcode: Opcode::LOAD,
        source_cnt: 1,
        source: [
            Operand { op_type: OpType::MEMORY, union: OpUnion::Memory(addr) },
            Operand { op_type: OpType::UNUSED, union: OpUnion::Unused }
        ],
        sink: Operand { op_type: OpType::REGISTER, union: OpUnion::Register(sink) },
        line,
    }
}


pub(crate) fn create_MOV(src_reg: RegisterType, dst_reg: RegisterType, line: i32) -> Instr {
    Instr {
        cycles: 1,
        opcode: Opcode::MOV,
        source_cnt: 1,
        source: [
            Operand { op_type: OpType::REGISTER, union: OpUnion::Register(src_reg) },
            Operand { op_type: OpType::UNUSED, union: OpUnion::Unused }
        ],
        sink: Operand { op_type: OpType::REGISTER, union: OpUnion::Register(dst_reg) },
        line,
    }
}

pub(crate) fn create_STORE(src: RegisterType, addr: MemoryAddressType, line: i32) -> Instr {
    Instr {
        cycles: 1,
        opcode: Opcode::STORE,
        source_cnt: 1,
        source: [
            Operand { op_type: OpType::REGISTER, union: OpUnion::Register(src) },
            Operand { op_type: OpType::UNUSED, union: OpUnion::Unused }
        ],
        sink: Operand { op_type: OpType::MEMORY, union: OpUnion::Memory(addr) },
        line,
    }
}

pub(crate) const fn create_NOP(line: i32) -> Instr {
    Instr {
        cycles: 1,
        opcode: Opcode::NOP,
        source_cnt: 0,
        source: [
            Operand { op_type: OpType::UNUSED, union: OpUnion::Unused },
            Operand { op_type: OpType::UNUSED, union: OpUnion::Unused }
        ],
        sink: Operand { op_type: OpType::UNUSED, union: OpUnion::Unused },
        line,
    }
}

pub(crate) const fn create_PRINTR(reg: RegisterType, line: i32) -> Instr {
    Instr {
        cycles: 1,
        opcode: Opcode::PRINTR,
        source_cnt: 1,
        source: [
            Operand { op_type: OpType::REGISTER, union: OpUnion::Register(reg) },
            Operand { op_type: OpType::UNUSED, union: OpUnion::Unused }
        ],
        sink: Operand { op_type: OpType::UNUSED, union: OpUnion::Unused },
        line,
    }
}

pub(crate) const fn create_JNZ(reg: RegisterType, address: CodeAddressType, line: i32) -> Instr {
    Instr {
        cycles: 1,
        opcode: Opcode::JNZ,
        source_cnt: 2,
        source: [
            Operand { op_type: OpType::REGISTER, union: OpUnion::Register(reg) },
            Operand { op_type: OpType::CODE, union: OpUnion::Code(address) }
        ],
        sink: Operand { op_type: OpType::UNUSED, union: OpUnion::Unused },
        line,
    }
}

pub(crate) const fn create_JZ(reg: RegisterType, address: CodeAddressType, line: i32) -> Instr {
    Instr {
        cycles: 1,
        opcode: Opcode::JZ,
        source_cnt: 2,
        source: [
            Operand { op_type: OpType::REGISTER, union: OpUnion::Register(reg) },
            Operand { op_type: OpType::CODE, union: OpUnion::Code(address) }
        ],
        sink: Operand { op_type: OpType::UNUSED, union: OpUnion::Unused },
        line,
    }
}
