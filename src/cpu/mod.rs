mod regs;

use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, clock::Clock};

use self::regs::Registers;

pub struct Cpu {
    bus: Rc<RefCell<Bus>>,
    clock: Rc<RefCell<Clock>>,
    regs: Registers
}

impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>, clock: Rc<RefCell<Clock>>) -> Self {
        Self {
            bus,
            clock,
            regs: Registers::new(),
        }
    }

    pub fn reset(&mut self) {
        self.cu.regs.pc = 0;
        self.clock.borrow_mut().reset();
    }
    
    pub fn execute(&mut self) {
        println!("{:#?}", self.regs);
    }
}