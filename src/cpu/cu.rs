use crate::cpu::regs::Registers;

use super::{RefBus, RefClock};

pub struct CUnit {
    pub regs: Registers,
    bus: RefBus,
    clock: RefClock,
}

impl CUnit {
    pub fn new(regs: Registers, bus: RefBus, clock: RefClock) -> Self {
        Self {
            regs,
            bus,
            clock
        }
    }

    pub fn decode(&mut self, opcode: u8) -> Result<(), String> {
        // ld r,r' block
        match opcode {
            0x40..=0x7F =>{
                if self.ld_r_r(opcode).is_ok() { return Ok(()) }
                if self.ld_r_hl(opcode).is_ok() { return Ok(()) }
                if self.ld_hl_r(opcode).is_ok() { return Ok(()) }
                
                Err(format!("Opcode {:#04X} not implemented", opcode))
            },
            _ => Err(format!("Opcode {:#04X} not implemented", opcode))
        }
    }

    fn ld_r_r(&self, opcode: u8) -> Result<(), String> {
        let dst = (opcode & 0b00111000) >> 3;
        let src = opcode & 0b00000111;

        self.regs.main.set_reg(dst, self.regs.main.get_reg(src)?)?;
        self.clock.borrow_mut().add(1);

        Ok(())
    }

    fn ld_r_hl(&self, opcode: u8) -> Result<(), String> {
        if opcode & 0b00000111 != 0b110 {
            return Err(format!("Invalid opcode for ld_r_hl {:#04X}", opcode));
        }
        let dst = (opcode & 0b00111000) >> 3;

        self.regs.main.set_reg(dst, self.bus.borrow().read(self.regs.main.hl()))?;
        self.clock.borrow_mut().add(1);

        Ok(())
    }

    fn ld_hl_r(&self, opcode: u8) -> Result<(), String> {
        if (opcode & 0b00111000) >> 3 != 0b110 {
            return Err(format!("Invalid opcode for ld_r_hl {:#04X}", opcode));
        }
        let src = opcode & 0b00000111;

        self.bus.borrow_mut().write(self.regs.main.hl(), self.regs.main.get_reg(src)?);
        self.clock.borrow_mut().add(1);

        Ok(())
    }
}