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
        self.peek(address)
    }
    
    fn write(&mut self, address: u16, value: u8) {
        self.clock.borrow_mut().add(3);
        self.data[(address - self.base_address) as usize] = value
    }
    
    fn peek(&self, address: u16) -> u8 {
        self.data[(address - self.base_address) as usize]
    }

    fn poke(&mut self, address: u16, value: u8) {
        self.data[(address - self.base_address) as usize] = value;
    }

    fn write_vec(&mut self, address: u16, data: Vec<u8>) {
        for (i, value) in data.iter().enumerate() {
            self.poke(address + i as u16, *value)
        }
    }
}

#[cfg(test)]
mod test_ram {
    use std::{cell::RefCell, rc::Rc};

    use crate::{bus::BusDevice, clock::Clock, device::ram::Ram};

    fn init() -> (Ram, Rc<RefCell<Clock>>) {
        let clock = Rc::new(RefCell::new(Clock::new()));
        (Ram::new(0x0000, 0x100, clock.clone()), clock)
    }

    #[test]
    fn test_peek_poke() {
        let (mut ram, clock) = init();
        
        ram.poke(0x0000, 0x11);
        assert_eq!(clock.borrow().read(), 0);
        assert_eq!(ram.peek(0x0000), 0x11);
    }
    
    #[test]
    fn test_read_write() {
        let (mut ram, clock) = init();
        
        ram.write(0x0001, 0xDD); // 3 tics
        assert_eq!(ram.read(0x0000), 0); // 3 + 3 tics
        assert_eq!(ram.read(0x0001), 0xDD); // 3 + 3 + 3 tics
        assert_eq!(clock.borrow().read(), 9);
    }
    
    #[test]
    fn test_write_vec() {
        let (mut ram, clock) = init();
        
        ram.write_vec(0x0000, vec![0x01, 0x02, 0x03, 0xFF]);
        assert_eq!(clock.borrow().read(), 0);
        assert_eq!(ram.peek(0x0000), 0x01);
        assert_eq!(ram.peek(0x0001), 0x02);
        assert_eq!(ram.peek(0x0002), 0x03);
        assert_eq!(ram.peek(0x0003), 0xFF);
        assert_eq!(ram.peek(0x0004), 0);
    }
}