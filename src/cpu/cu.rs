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
        if opcode >> 6 == 1 {
            self.ld_r_r(opcode);
            return Ok(());
        }

        Err(format!("Opcode {:#04X} not implemented", opcode))
    }

    fn ld_r_r(&mut self, opcode: u8) {
        let dst = (opcode & 0b00111000) >> 3;
        let src = opcode & 0b00000111;

        let ref_src_reg = self.get_reg(src).unwrap().clone();
        let ref_dst_reg = self.get_reg(dst).unwrap();

        *ref_dst_reg = ref_src_reg;

        self.clock.borrow_mut().add(1);
    }
    
    fn get_reg(&mut self, ix: u8) -> Option<&mut Box<u8>> {
        match ix {
            0b000 => Some(&mut self.regs.main.b),
            0b001 => Some(&mut self.regs.main.c),
            0b010 => Some(&mut self.regs.main.d),
            0b011 => Some(&mut self.regs.main.e),
            0b100 => Some(&mut self.regs.main.h),
            0b101 => Some(&mut self.regs.main.l),
            0b111 => Some(&mut self.regs.main.a),
            _ => None            
        }
        
    }
}