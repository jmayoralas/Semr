mod regs;

use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, clock::Clock};

use self::regs::Registers;

pub struct Cpu {
    bus: Rc<RefCell<Bus>>,
    clock: Rc<RefCell<Clock>>,
}

impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>, clock: Rc<RefCell<Clock>>) -> Self {
        Self {
            bus,
            clock,
        }
    }

    pub fn execute(&mut self) {
    }
}