mod regs;

use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, clock::Clock};


pub type RefBus = Rc<RefCell<Bus>>;
pub type RefClock = Rc<RefCell<Clock>>;

pub struct Cpu {
    bus: RefBus,
    clock: RefClock,
    regs: Registers
}

impl Cpu {
    pub fn new(bus: RefBus, clock: RefClock) -> Self {
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