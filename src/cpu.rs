use std::cell::RefCell;
use crate::instructions::{InstrQueue, Program, RegisterType, WordType};
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use crate::backend::Backend;
use crate::frontend::Frontend;
use crate::memory_subsystem::MemorySubsystem;

pub(crate) struct CPUConfig {
    pub(crate) arch_reg_count: u16,
    pub(crate) phys_reg_count: u16,
    pub(crate) frontend_n_wide: u8,
    pub(crate) instr_queue_capacity: u16,
    pub(crate) frequency_hz: u64,
    pub(crate) rs_count: u16,
    pub(crate) memory_size: u32,
    pub(crate) sb_capacity: u16,
    pub(crate) lfb_count: u8,
    pub(crate) rob_capacity: u16,
    pub(crate) eu_count: u8,
    pub(crate) trace: bool,
}

pub(crate) struct CPU {
    backend: Backend,
    frontend: Frontend,
    memory_subsystem: Rc<RefCell<MemorySubsystem>>,
    arch_reg_file: Rc<RefCell<ArgRegFile>>,
    cycle_cnt: u64,
    cycle_period: Duration,
}

impl CPU {
    pub(crate) fn new(cpu_config: & CPUConfig) -> CPU {
        let instr_queue = Rc::new(RefCell::new(InstrQueue::new(cpu_config.instr_queue_capacity)));

        let memory_subsystem = Rc::new(RefCell::new(MemorySubsystem::new(cpu_config)));

        let arch_reg_file = Rc::new(RefCell::new(ArgRegFile::new(cpu_config.arch_reg_count)));

        let backend = Backend::new(cpu_config,
                                   Rc::clone(&instr_queue),
                                   Rc::clone(&memory_subsystem),
                                   Rc::clone(&arch_reg_file),
        );

        let frontend = Frontend::new(cpu_config,
                                     Rc::clone(&instr_queue));

        CPU {
            backend,
            frontend,
            memory_subsystem,
            arch_reg_file,
            cycle_cnt: 0,
            cycle_period: Duration::from_micros(1_000_000 / cpu_config.frequency_hz),
        }
    }

    pub(crate) fn run(&mut self, program: Program) {
        self.frontend.init(program);

        loop {
            self.cycle_cnt += 1;
            println!("=======================================================================");
            println!("Cycle {}", self.cycle_cnt);
            self.memory_subsystem.borrow_mut().do_cycle();
            self.backend.do_cycle();
            self.frontend.do_cycle();
            thread::sleep(self.cycle_period);
        }
    }
}

struct ArgReg {
    pub(crate) value: WordType,
}

pub struct ArgRegFile {
    registers: Vec<ArgReg>,
}

impl ArgRegFile {
    fn new(rs_count: u16) -> ArgRegFile {
        let mut array = Vec::with_capacity(rs_count as usize);
        for _ in 0..rs_count {
            array.push(ArgReg { value: 0 });
        }

        ArgRegFile { registers: array }
    }

    pub fn get_value(&self, reg: RegisterType) -> WordType {
        return self.registers.get(reg as usize).unwrap().value;
    }

    pub fn set_value(&mut self, reg: RegisterType, value: WordType) {
        let arch_reg = self.registers.get_mut(reg as usize).unwrap();
        arch_reg.value = value;
    }
}
