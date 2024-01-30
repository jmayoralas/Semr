use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, clock::Clock};

pub struct Cpu<'a> {
    bus: &'a RefCell<Bus>,
    clock: Rc<RefCell<Clock>>
}

impl<'a> Cpu<'a> {
    pub fn new(bus: &'a RefCell<Bus>, clock: Rc<RefCell<Clock>>) -> Self {
        Self {
            bus,
            clock
        }
    }

    pub fn execute(&mut self) {
        self.bus.borrow_mut().write(0x0000, 0x22);
        self.clock.borrow_mut().add(4);
    }
}