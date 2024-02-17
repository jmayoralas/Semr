mod regs;
mod cu;

use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, clock::Clock};

use self::{cu::CUnit, regs::Registers};

pub type RefBus = Rc<RefCell<Bus>>;
pub type RefClock = Rc<RefCell<Clock>>;

pub struct Cpu {
    bus: RefBus,
    clock: RefClock,
    cu: CUnit,
}

impl Cpu {
    pub fn new(bus: RefBus, clock: RefClock) -> Self {
        Self {
            bus: bus.clone(),
            clock: clock.clone(),
            cu: CUnit::new(Registers::new(), bus.clone(), clock.clone()),
        }
    }

    pub fn reset(&mut self) {
        self.cu.regs.pc = 0;
        self.clock.borrow_mut().reset();
    }
    
    pub fn execute(&mut self) -> Result<(), String> {
        let opcode = self.fetch_op();
        self.cu.regs.pc += 1;

        self.cu.decode(opcode)?;

        Ok(())
    }

    fn fetch_op(&self) -> u8 {
        self.bus.borrow().read(self.cu.regs.pc)
    }
}

#[cfg(test)]
mod test_cpu {
    use std::{cell::RefCell, rc::Rc};

    use crate::{bus::Bus, clock::Clock, device::ram::Ram};

    use super::{Cpu, RefBus, RefClock};

    fn init() -> Cpu {
        let bus: RefBus = Rc::new(RefCell::new(Bus::new()));
        let clock: RefClock = Rc::new(RefCell::new(Clock::new()));
        bus.borrow_mut().add_device(Box::new(Ram::new(0x0000, 0x1000, clock.clone()))).unwrap();
        
        Cpu::new(bus, clock)
    }

    #[test]
    fn test_ld_r_r() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x40, 0x41]);
        cpu.cu.regs.main.set_bc(0x1122);

        cpu.execute(); // b,b
        assert_eq!(cpu.cu.regs.main.b(), 0x11);
        assert_eq!(cpu.clock.borrow().read(), 4);

        cpu.execute(); // b,c
        assert_eq!(cpu.cu.regs.main.b(), 0x22);
        assert_eq!(cpu.clock.borrow().read(), 4 + 4);
    }

    #[test]
    fn test_ld_r_hl() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x46, 0x4E, 0x56, 0x5E]);
        cpu.cu.regs.main.set_hl(0x0100);
        cpu.bus.borrow_mut().write_vec(cpu.cu.regs.main.hl(), vec![0x44]);

        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res); // b,(hl)
        assert_eq!(cpu.cu.regs.main.b(), 0x44);
        assert_eq!(cpu.clock.borrow().read(), 7);
        cpu.clock.borrow_mut().reset();

        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res); // b,(hl)
        assert_eq!(cpu.cu.regs.main.c(), 0x44);
        assert_eq!(cpu.clock.borrow().read(), 7);
        cpu.clock.borrow_mut().reset();

        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res); // b,(hl)
        assert_eq!(cpu.cu.regs.main.d(), 0x44);
        assert_eq!(cpu.clock.borrow().read(), 7);
        cpu.clock.borrow_mut().reset();

        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res); // b,(hl)
        assert_eq!(cpu.cu.regs.main.e(), 0x44);
        assert_eq!(cpu.clock.borrow().read(), 7);
        cpu.clock.borrow_mut().reset();
    }

    #[test]
    fn test_ld_hl_r() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x70]);
        cpu.cu.regs.main.set_hl(0x0100);
        cpu.cu.regs.main.set_b(0x55);
        cpu.bus.borrow_mut().write_vec(cpu.cu.regs.main.hl(), vec![0x00]);

        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res); // b,(hl)
        assert_eq!(cpu.bus.borrow().peek(cpu.cu.regs.main.hl()), 0x55);
        assert_eq!(cpu.clock.borrow().read(), 7);
    }
}