#[derive(Debug)]
pub struct RegisterSet {
    pub a: Box<u8>,
    pub f: Box<u8>,
    pub b: Box<u8>,
    pub c: Box<u8>,
    pub d: Box<u8>,
    pub e: Box<u8>,
    pub h: Box<u8>,
    pub l: Box<u8>,
}

impl RegisterSet {
    fn new() -> Self {
        Self {
            a: Box::new(0),
            f: Box::new(0),
            b: Box::new(0),
            c: Box::new(0),
            d: Box::new(0),
            e: Box::new(0),
            h: Box::new(0),
            l: Box::new(0),
        }
    }

    pub fn af(&self) -> u16 {
        (*self.a as u16) << 8 | *self.f as u16
    }
    pub fn set_af(&mut self, value: u16) {
        *self.a = (value >> 8) as u8;
        *self.f = (value & 0x00FF) as u8;
    }
    pub fn bc(&self) -> u16 {
        (*self.b as u16) << 8 | *self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        *self.b = (value >> 8) as u8;
        *self.c = (value & 0x00FF) as u8;
    }
    pub fn de(&self) -> u16 {
        (*self.d as u16) << 8 | *self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        *self.d = (value >> 8) as u8;
        *self.e = (value & 0x00FF) as u8;
    }
    pub fn hl(&self) -> u16 {
        (*self.h as u16) << 8 | *self.l as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        *self.h = (value >> 8) as u8;
        *self.l = (value & 0x00FF) as u8;
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
        *bank.main.a = 0xdd;
        bank.alt.set_hl(0xccbb);
        bank.pc = 0x1234;
        assert_eq!(*bank.main.a, 0xdd);
        assert_eq!(*bank.main.h, 0x00);
        assert_eq!(*bank.main.l, 0x00);
        assert_eq!(*bank.alt.h, 0xcc);
        assert_eq!(*bank.alt.l, 0xbb);
        assert_eq!(bank.pc, 0x1234);
    }
}