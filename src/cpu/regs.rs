#[derive(Debug)]
pub struct RegisterSet {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl RegisterSet {
    fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn f(&self) -> u8 {
        self.f
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn c(&self) -> u8 {
        self.c
    }
    pub fn d(&self) -> u8 {
        self.d
    }
    pub fn e(&self) -> u8 {
        self.e
    }
    pub fn h(&self) -> u8 {
        self.h
    }
    pub fn l(&self) -> u8 {
        self.l
    }
    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }
    pub fn set_f(&mut self, value: u8) {
        self.f = value;
    }
    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }
    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }
    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }
    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }
    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }
    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00FF) as u8;
    }
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }
    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }
    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    pub fn set_reg(&mut self, ix: u8, value: u8) -> Result<(), String> {
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
    pub ix: u16,
    pub iy: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            main: RegisterSet::new(),
            alt: RegisterSet::new(),
            pc: 0,
            sp: 0,
            ix: 0,
            iy: 0,
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