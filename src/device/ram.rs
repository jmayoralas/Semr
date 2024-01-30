use std::{cell::RefCell, iter::repeat, rc::Rc};

use crate::{bus::BusDevice, clock::Clock};

pub struct Ram {
    base_address: u16,
    size: u16,
    data: Vec<u8>,
    clock: Rc<RefCell<Clock>>
}

impl Ram {
    pub fn new(base_address: u16, size: u16, clock: Rc<RefCell<Clock>>) -> Self {
        Ram {
            base_address,
            size,
            data: Vec::from_iter(repeat(0x00).take(size as usize)),
            clock,
        }
    }
}

impl BusDevice for Ram {
    fn get_base_address(&self) -> u16 {
        self.base_address
    }

    fn get_size(&self) -> u16 {
        self.size
    }

    fn read(&self, address: u16) -> u8 {
        self.clock.borrow_mut().add(3);
        self.data[(address - self.base_address) as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.clock.borrow_mut().add(3);
        self.data[(address - self.base_address) as usize] = value
    }
}