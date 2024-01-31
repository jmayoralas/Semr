use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, clock::Clock};

pub struct Screen {
    bus: Rc<RefCell<Bus>>,
    clock: Rc<RefCell<Clock>>
}

impl Screen {
    pub fn new(bus: Rc<RefCell<Bus>>, clock: Rc<RefCell<Clock>>) -> Self {
        Self {
            bus,
            clock
        }
    }

    pub fn peek_bus(&self, address: u16) {
        println!("value: {}", self.bus.borrow().read(address));
        println!("clock: {}", self.clock.borrow().read());
    }
}