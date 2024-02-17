use std::{borrow::{Borrow, BorrowMut}, cell::RefCell};

#[derive(Debug)]
pub struct RegisterSet {
    a: RefCell<u8>,
    f: RefCell<u8>,
    b: RefCell<u8>,
    c: RefCell<u8>,
    d: RefCell<u8>,
    e: RefCell<u8>,
    h: RefCell<u8>,
    l: RefCell<u8>,
}

impl RegisterSet {
    fn new() -> Self {
        Self {
            a: RefCell::new(0),
            f: RefCell::new(0),
            b: RefCell::new(0),
            c: RefCell::new(0),
            d: RefCell::new(0),
            e: RefCell::new(0),
            h: RefCell::new(0),
            l: RefCell::new(0),
        }
    }

    pub fn a(&self) -> u8 {
        *self.a.borrow()
    }
    pub fn f(&self) -> u8 {
        *self.f.borrow()
    }
    pub fn b(&self) -> u8 {
        *self.b.borrow()
    }
    pub fn c(&self) -> u8 {
        *self.c.borrow()
    }
    pub fn d(&self) -> u8 {
        *self.d.borrow()
    }
    pub fn e(&self) -> u8 {
        *self.e.borrow()
    }
    pub fn h(&self) -> u8 {
        *self.h.borrow()
    }
    pub fn l(&self) -> u8 {
        *self.l.borrow()
    }
    pub fn set_a(&self, value: u8) {
        *self.a.borrow_mut() = value;
    }
    pub fn set_f(&self, value: u8) {
        *self.f.borrow_mut() = value;
    }
    pub fn set_b(&self, value: u8) {
        *self.b.borrow_mut() = value;
    }
    pub fn set_c(&self, value: u8) {
        *self.c.borrow_mut() = value;
    }
    pub fn set_d(&self, value: u8) {
        *self.d.borrow_mut() = value;
    }
    pub fn set_e(&self, value: u8) {
        *self.e.borrow_mut() = value;
    }
    pub fn set_h(&self, value: u8) {
        *self.h.borrow_mut() = value;
    }
    pub fn set_l(&self, value: u8) {
        *self.l.borrow_mut() = value;
    }

    pub fn af(&self) -> u16 {
        (*self.a.borrow() as u16) << 8 | *self.f.borrow() as u16
    }
    pub fn set_af(&self, value: u16) {
        *self.a.borrow_mut() = (value >> 8) as u8;
        *self.f.borrow_mut() = (value & 0x00FF) as u8;
    }
    pub fn bc(&self) -> u16 {
        (*self.b.borrow() as u16) << 8 | *self.c.borrow() as u16
    }
    pub fn set_bc(&self, value: u16) {
        *self.b.borrow_mut() = (value >> 8) as u8;
        *self.c.borrow_mut() = (value & 0x00FF) as u8;
    }
    pub fn de(&self) -> u16 {
        (*self.d.borrow() as u16) << 8 | *self.e.borrow() as u16
    }
    pub fn set_de(&self, value: u16) {
        *self.d.borrow_mut() = (value >> 8) as u8;
        *self.e.borrow_mut() = (value & 0x00FF) as u8;
    }
    pub fn hl(&self) -> u16 {
        (*self.h.borrow() as u16) << 8 | *self.l.borrow() as u16
    }
    pub fn set_hl(&self, value: u16) {
        *self.h.borrow_mut() = (value >> 8) as u8;
        *self.l.borrow_mut() = (value & 0x00FF) as u8;
    }

    pub fn set_reg(&self, ix: u8, value: u8) -> Result<(), String> {
        match ix {
            0b000 => {self.set_b(value); Ok(())},
            0b001 => {self.set_c(value); Ok(())},
            0b010 => {self.set_d(value); Ok(())},
            0b011 => {self.set_e(value); Ok(())},
            0b100 => {self.set_h(value); Ok(())},
            0b101 => {self.set_l(value); Ok(())},
            0b111 => {self.set_a(value); Ok(())},
            _ => Err("Bad register index".to_string())
        }
    }

    pub fn get_reg(&self, ix: u8) -> Result<u8, String> {
        match ix {
            0b000 => Ok(self.b()),
            0b001 => Ok(self.c()),
            0b010 => Ok(self.d()),
            0b011 => Ok(self.e()),
            0b100 => Ok(self.h()),
            0b101 => Ok(self.l()),
            0b111 => Ok(self.a()),
            _ => Err("Bad register index".to_string())
        }
    }

}

#[derive(Debug)]
pub struct Registers {
    pub main: RegisterSet,
    pub alt: RegisterSet,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            main: RegisterSet::new(),
            alt: RegisterSet::new(),
            pc: 0,
            sp: 0,
        }
    }
}

#[cfg(test)]
mod test_register {
    use super::Registers;

    #[test]
    fn test_regbank() {
        let mut bank = Registers::new();
        bank.main.set_a(0xdd);
        bank.alt.set_hl(0xccbb);
        bank.pc = 0x1234;
        assert_eq!(bank.main.a(), 0xdd);
        assert_eq!(bank.main.h(), 0x00);
        assert_eq!(bank.main.l(), 0x00);
        assert_eq!(bank.alt.h(), 0xcc);
        assert_eq!(bank.alt.l(), 0xbb);
        assert_eq!(bank.pc, 0x1234);
    }
}