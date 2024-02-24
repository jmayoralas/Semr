use crate::cpu::regs::Registers;

use super::{RefBus, RefClock};

pub enum Status {
    Running,
    Halted
}

pub enum IndexedAddressMode {
    IX,
    IY
}

pub struct CUnit {
    pub regs: Registers,
    bus: RefBus,
    clock: RefClock,
    pub status: Status,
    pub address_mode: Option<IndexedAddressMode>,
}

impl CUnit {
    pub fn new(regs: Registers, bus: RefBus, clock: RefClock) -> Self {
        Self {
            regs,
            bus,
            clock,
            status: Status::Running,
            address_mode: None,
        }
    }

    fn read_little_endian_u16(&self, address: u16) -> u16 {
        (self.bus.borrow().read(address + 1) as u16) << 8 + self.bus.borrow().read(address) as u16
    }

    fn get_address_by_address_mode(&mut self) -> u16 {
        match &self.address_mode {
            None => {
                self.regs.main.hl()
            }
            Some(mode) => {
                let base_address = match mode {
                    IndexedAddressMode::IX => self.regs.ix,
                    IndexedAddressMode::IY => self.regs.iy
                };

                let d = self.bus.borrow().read(self.regs.pc) as i8;
                self.regs.pc += 1;

                self.clock.borrow_mut().add(2);
                base_address.wrapping_add(d as u16)
            }
        }
    }

    pub fn decode(&mut self, opcode: u8) -> Result<(), String> {
        if opcode & 0b11000111 == 0b110 {
            if self.ld_r_n(opcode).is_ok() { return Ok(()) }
        }

        match opcode {
            0x00 => { self.nop(); Ok(()) }
            0x02 => { self.ld_bc_a(); Ok(()) }
            0x0A => { self.ld_a_bc(); Ok(()) }
            0x12 => { self.ld_de_a(); Ok(()) }
            0x1A => { self.ld_a_de(); Ok(()) }
            0x32 => { self.ld_nn_a(); Ok(()) }
            0x36 => { self.ld_hl_n(); Ok(()) }
            0x3A => { self.ld_a_nn(); Ok(()) }
            0x40..=0x7F =>{
                if self.halt(opcode).is_ok() { return Ok(()) }
                if self.ld_r_r(opcode).is_ok() { return Ok(()) }
                if self.ld_r_hl(opcode).is_ok() { return Ok(()) }
                if self.ld_hl_r(opcode).is_ok() { return Ok(()) }

                Err(format!("Opcode {:#04X} not implemented", opcode))
            }
            0xDD => {
                self.address_mode = Some(IndexedAddressMode::IX);
                self.clock.borrow_mut().add(1);
                Ok(())
            }
            0xFD => {
                self.address_mode = Some(IndexedAddressMode::IY);
                self.clock.borrow_mut().add(1);
                Ok(())
            }
            _ => Err(format!("Opcode {:#04X} not implemented", opcode))
        }
    }

    fn nop(&mut self) {
        self.clock.borrow_mut().add(1);
        self.address_mode = None;
    }

    fn ld_r_r(&mut self, opcode: u8) -> Result<(), String> {
        let dst = (opcode & 0b00111000) >> 3;
        let src = opcode & 0b00000111;

        self.regs.main.set_reg(dst, self.regs.main.get_reg(src)?)?;
        self.clock.borrow_mut().add(1);
        self.address_mode = None;

        Ok(())
    }

    fn ld_r_hl(&mut self, opcode: u8) -> Result<(), String> {
        if opcode & 0b00000111 != 0b110 {
            return Err(format!("Invalid opcode for ld_r_hl {:#04X}", opcode));
        }
        
        let dst = (opcode & 0b00111000) >> 3;
        let address = self.get_address_by_address_mode();
        
        self.regs.main.set_reg(dst, self.bus.borrow().read(address))?;
        self.clock.borrow_mut().add(if self.address_mode.is_some() { 4 } else { 1 });
        self.address_mode = None;

        Ok(())
    }
    
    fn ld_hl_r(&mut self, opcode: u8) -> Result<(), String> {
        if (opcode & 0b00111000) >> 3 != 0b110 {
            return Err(format!("Invalid opcode for ld_r_hl {:#04X}", opcode));
        }
        
        let src = opcode & 0b00000111;
        let address = self.get_address_by_address_mode();
        
        self.bus.borrow_mut().write(address, self.regs.main.get_reg(src)?);
        self.clock.borrow_mut().add(if self.address_mode.is_some() { 4 } else { 1 });
        self.address_mode = None;

        Ok(())
    }
    
    fn ld_r_n(&mut self, opcode: u8) -> Result<(), String> {
        let dst = (opcode & 0b00111000) >> 3;

        if opcode & 0b00000111 != 0b110 || dst == 0b110 {
            return Err(format!("Invalid opcode for ld_r_n {:#04X}", opcode));
        }
        
        self.regs.main.set_reg(dst, self.bus.borrow().read(self.regs.pc))?;
        self.regs.pc += 1;

        self.clock.borrow_mut().add(1);
        self.address_mode = None;
        Ok(())
    }

    fn ld_hl_n(&mut self) {
        let address = self.get_address_by_address_mode();
        let value = self.bus.borrow().read(self.regs.pc);
        self.regs.pc += 1;
        self.bus.borrow_mut().write(address, value);
        self.clock.borrow_mut().add(1);
        self.address_mode = None;
    }
    
    fn halt(&mut self, opcode: u8) -> Result<(), String>{
        if opcode != 0x76 {
            return Err(format!("Invalid opcode for halt {:#04X}", opcode));
        }
        self.status = Status::Halted;
        self.clock.borrow_mut().add(1);
        self.address_mode = None;
        Ok(())
    }
    
    fn ld_bc_a(&mut self) {
        self.bus.borrow_mut().write(self.regs.main.bc(), self.regs.main.a());
        self.clock.borrow_mut().add(1);
        self.address_mode = None;
    }

    fn ld_de_a(&mut self) {
        self.bus.borrow_mut().write(self.regs.main.de(), self.regs.main.a());
        self.clock.borrow_mut().add(1);
        self.address_mode = None;
    }

    fn ld_nn_a(&mut self) {
        let address = self.read_little_endian_u16(self.regs.pc);
        self.regs.pc += 2;
        self.bus.borrow_mut().write(address, self.regs.main.a());
        self.clock.borrow_mut().add(1);
        self.address_mode = None;
    }

    fn ld_a_bc(&mut self) {
        self.regs.main.set_a(self.bus.borrow().read(self.regs.main.bc()));
        self.clock.borrow_mut().add(1);
        self.address_mode = None;
    }

    fn ld_a_de(&mut self) {
        self.regs.main.set_a(self.bus.borrow().read(self.regs.main.de()));
        self.clock.borrow_mut().add(1);
        self.address_mode = None;
    }

    fn ld_a_nn(&mut self) {
        self.regs.main.set_a(self.bus.borrow().read(self.read_little_endian_u16(self.regs.pc)));
        self.regs.pc += 2;
        self.clock.borrow_mut().add(1);
        self.address_mode = None;
    }
}