use std::{cell::RefCell, rc::Rc};

use semr::{bus::Bus, clock::Clock, cpu::Cpu, device::ram::Ram, screen::Screen};

extern crate semr;

fn main() {
    let clock = Rc::new(RefCell::new(Clock::new()));
    let bus = Rc::new(RefCell::new(Bus::new()));

    bus.borrow_mut().add_device(Box::new(Ram::new(0x0000, 0x100, Rc::clone(&clock)))).unwrap();

    let mut cpu = Cpu::new(Rc::clone(&bus), Rc::clone(&clock));
    let screen = Screen::new(Rc::clone(&bus), Rc::clone(&clock));

    cpu.reset();
    cpu.execute();
    screen.peek_bus(0x0000);
}
