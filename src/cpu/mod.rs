mod regs;
mod cu;

use std::{cell::RefCell, rc::Rc};

use crate::{bus::Bus, clock::Clock};

use self::{cu::{CUnit, Status}, regs::Registers};

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
        self.cu.regs.iff1 = false;
        self.cu.regs.iff2 = false;
        self.cu.regs.pc = 0;
        self.cu.prefix = None;
        self.cu.status = Status::Running;
        self.clock.borrow_mut().reset();
    }
    
    pub fn execute(&mut self) -> Result<(), String> {
        loop {
            let mut opcode = self.fetch_op();
            
            if let Status::Halted = self.cu.status {
                opcode = 0x00;
            } else {
                self.cu.regs.pc += 1;
            }
    
            self.cu.decode(opcode)?;
            
            if self.cu.prefix.is_none() {
                break;
            }
        }

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
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x47, 0x4A, 0x4B]);
        cpu.cu.regs.main.set_bc(0x1122);

        cpu.clock.borrow_mut().reset();
        let res = cpu.execute(); // b,b
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.b(), 0x11);
        assert_eq!(cpu.clock.borrow().read(), 4);
        
        cpu.clock.borrow_mut().reset();
        let res = cpu.execute(); // b,c
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.b(), 0x22);
        assert_eq!(cpu.clock.borrow().read(), 4);
        
        cpu.cu.regs.main.set_de(0x3344);
        
        cpu.clock.borrow_mut().reset();
        let res = cpu.execute(); // b,c
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.b(), 0x33);
        assert_eq!(cpu.clock.borrow().read(), 4);
        
        cpu.clock.borrow_mut().reset();
        let res = cpu.execute(); // b,c
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.b(), 0x44);
        assert_eq!(cpu.clock.borrow().read(), 4);

        cpu.cu.regs.main.set_hl(0x5566);

        cpu.clock.borrow_mut().reset();
        let res = cpu.execute(); // b,c
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.b(), 0x55);
        assert_eq!(cpu.clock.borrow().read(), 4);
        
        cpu.clock.borrow_mut().reset();
        let res = cpu.execute(); // b,c
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.b(), 0x66);
        assert_eq!(cpu.clock.borrow().read(), 4);

        cpu.cu.regs.main.set_a(0x77);
    
        cpu.clock.borrow_mut().reset();
        let res = cpu.execute(); // b,c
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.b(), 0x77);
        assert_eq!(cpu.clock.borrow().read(), 4);

        cpu.cu.regs.main.set_de(0x8899);

        cpu.clock.borrow_mut().reset();
        let res = cpu.execute(); // b,c
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.c(), 0x88);
        assert_eq!(cpu.clock.borrow().read(), 4);
        
        cpu.clock.borrow_mut().reset();
        let res = cpu.execute(); // b,c
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.c(), 0x99);
        assert_eq!(cpu.clock.borrow().read(), 4);
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
        assert!(res.is_ok(), "{:?}", res); // c,(hl)
        assert_eq!(cpu.cu.regs.main.c(), 0x44);
        assert_eq!(cpu.clock.borrow().read(), 7);
        cpu.clock.borrow_mut().reset();
        
        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res); // d,(hl)
        assert_eq!(cpu.cu.regs.main.d(), 0x44);
        assert_eq!(cpu.clock.borrow().read(), 7);
        cpu.clock.borrow_mut().reset();
        
        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res); // e,(hl)
        assert_eq!(cpu.cu.regs.main.e(), 0x44);
        assert_eq!(cpu.clock.borrow().read(), 7);
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

    #[test]
    fn test_halt() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x76]);

        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res); // halt
        assert_eq!(cpu.clock.borrow().read(), 4);
        assert_eq!(cpu.cu.regs.pc, 0x0001);

        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 8);
        assert_eq!(cpu.cu.regs.pc, 0x0001);

        cpu.reset();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0xDD, 0x76]);

        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res); // b,(hl)
        assert_eq!(cpu.clock.borrow().read(), 8);
        assert_eq!(cpu.cu.regs.pc, 0x0002);

        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res); // b,(hl)
        assert_eq!(cpu.clock.borrow().read(), 12);
        assert_eq!(cpu.cu.regs.pc, 0x0002);

    }

    #[test]
    fn test_nop() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x00, 0x00]);

        let res = cpu.execute(); // b,c
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 4);
        assert_eq!(cpu.cu.regs.pc, 0x0001);
        
        let res = cpu.execute(); // b,c
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 8);
        assert_eq!(cpu.cu.regs.pc, 0x0002);
    }
    
    #[test]
    fn test_ld_r_n() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x06, 0x44, 0x0E, 0x55, 0x16, 0x66, 0x1E, 0x77, 0x26, 0x88, 0x2E, 0x99, 0x3E, 0xAA]);

        let res = cpu.execute(); // b,n
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.cu.regs.main.b(), 0x44);
        cpu.clock.borrow_mut().reset();

        let res = cpu.execute(); // c,n
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.cu.regs.main.c(), 0x55);
        cpu.clock.borrow_mut().reset();

        let res = cpu.execute(); // d,n
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.cu.regs.main.d(), 0x66);
        cpu.clock.borrow_mut().reset();

        let res = cpu.execute(); // e,n
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.cu.regs.main.e(), 0x77);
        cpu.clock.borrow_mut().reset();

        let res = cpu.execute(); // h,n
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.cu.regs.main.h(), 0x88);
        cpu.clock.borrow_mut().reset();

        let res = cpu.execute(); // l,n
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.cu.regs.main.l(), 0x99);
        cpu.clock.borrow_mut().reset();

        let res = cpu.execute(); // a,n
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.cu.regs.main.a(), 0xAA);
        cpu.clock.borrow_mut().reset();
    }
    
    #[test]
    fn test_ld_hl_n() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x36, 0x55]);
        cpu.cu.regs.main.set_hl(0x0100);
        
        let res = cpu.execute(); // (hl),n
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 10);
        assert_eq!(cpu.bus.borrow().peek(cpu.cu.regs.main.hl()), 0x55);
    }
    
    #[test]
    fn test_ld_r_ixd() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0xDD, 0x46, 0x01, 0xFD, 0x46, 0xFF, 0xDD, 0x4E, 0x01]);
        cpu.bus.borrow_mut().write_vec(0x0100, vec![0xBB, 0xAA]);
        cpu.cu.regs.ix = 0x0100;
        cpu.cu.regs.iy = 0x0101;
        
        let res = cpu.execute(); // b,(IX+d)
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 19);
        assert_eq!(cpu.cu.regs.main.b(), 0xAA);
        cpu.clock.borrow_mut().reset();
        
        let res = cpu.execute(); // b,(IY+d)
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 19);
        assert_eq!(cpu.cu.regs.main.b(), 0xBB);
        cpu.clock.borrow_mut().reset();

        let res = cpu.execute(); // b,(IX+d)
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 19);
        assert_eq!(cpu.cu.regs.main.c(), 0xAA);
    }

    #[test]
    fn test_ld_ixd_n() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0xDD, 0x36, 0x01, 0x55]);
        cpu.cu.regs.ix = 0x0100;
        
        let res = cpu.execute(); // (hl),n
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 19);
        assert_eq!(cpu.bus.borrow().peek(0x101), 0x55);
    }
    
    #[test]
    fn test_ld_ixd_r() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0xDD, 0x70, 0x01]);
        cpu.cu.regs.ix = 0x0100;
        cpu.cu.regs.main.set_b(0xAA);
        
        let res = cpu.execute(); // (ix+d),b
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 19);
        assert_eq!(cpu.bus.borrow().peek(0x101), 0xAA);
    }
    
    #[test]
    fn test_ld_a_bc() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x0A]);
        cpu.bus.borrow_mut().write_vec(0x0100, vec![0xBB]);
        cpu.cu.regs.main.set_bc(0x0100);

        let res = cpu.execute(); // ld a,(bc)
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.cu.regs.main.a(), 0xBB);
    }

    #[test]
    fn test_ld_a_de() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x1A]);
        cpu.bus.borrow_mut().write_vec(0x0100, vec![0xBB]);
        cpu.cu.regs.main.set_de(0x0100);

        let res = cpu.execute(); // ld a,(bc)
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.cu.regs.main.a(), 0xBB);
    }

    #[test]
    fn test_ld_a_nn() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x3A, 0x00, 0x01]);
        cpu.bus.borrow_mut().write_vec(0x0100, vec![0xBB]);

        let res = cpu.execute(); // ld a,(bc)
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 13);
        assert_eq!(cpu.cu.regs.main.a(), 0xBB);
        assert_eq!(cpu.cu.regs.pc, 0x0003);
    }

    #[test]
    fn test_ld_bc_a() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x02]);
        cpu.cu.regs.main.set_bc(0x0100);
        cpu.cu.regs.main.set_a(0xBB);

        let res = cpu.execute(); // ld a,(bc)
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.bus.borrow().peek(cpu.cu.regs.main.bc()), 0xBB);
    }

    #[test]
    fn test_ld_de_a() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x12]);
        cpu.cu.regs.main.set_de(0x0100);
        cpu.cu.regs.main.set_a(0xBB);

        let res = cpu.execute(); // ld a,(bc)
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 7);
        assert_eq!(cpu.bus.borrow().peek(cpu.cu.regs.main.de()), 0xBB);
    }

    #[test]
    fn test_ld_nn_a() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0x32, 0x00, 0x01]);
        cpu.cu.regs.main.set_a(0xBB);

        let res = cpu.execute(); // ld a,(bc)
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.clock.borrow().read(), 13);
        assert_eq!(cpu.bus.borrow().peek(0x100), 0xBB);
        assert_eq!(cpu.cu.regs.pc, 0x0003);
    }
    
    #[test]
    fn test_ld_a_i() {
        let mut cpu = init();
        cpu.bus.borrow_mut().write_vec(0x0000, vec![0xED, 0x57, 0xED, 0x57, 0xED, 0x57]);
        cpu.cu.regs.main.set_f(0b11111111);
        
        cpu.cu.regs.i = 0x55;
        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.a(), 0x55);
        assert_eq!(cpu.clock.borrow().read(), 9);
        assert_eq!(cpu.cu.regs.pc, 0x0002);
        assert_eq!(cpu.cu.regs.main.f(), 0b00101001);
        
        cpu.cu.regs.i = 0x00;
        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.a(), 0x00);
        assert_eq!(cpu.clock.borrow().read(), 18);
        assert_eq!(cpu.cu.regs.pc, 0x0004);
        assert_eq!(cpu.cu.regs.main.f(), 0b01101001);

        cpu.cu.regs.i = 0xFE;
        let res = cpu.execute();
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(cpu.cu.regs.main.a(), 0xFE);
        assert_eq!(cpu.clock.borrow().read(), 27);
        assert_eq!(cpu.cu.regs.pc, 0x0006);
        assert_eq!(cpu.cu.regs.main.f(), 0b10101001);
    }
}