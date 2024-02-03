use crate::cpu::{RefBus, RefClock};

pub struct Screen {
    bus: RefBus,
    clock: RefClock
}

impl Screen {
    pub fn new(bus: RefBus, clock: RefClock) -> Self {
        Self {
            bus,
            clock
        }
    }

    pub fn peek_bus(&self, address: u16) {
        println!("value: {}", self.bus.borrow().peek(address));
        println!("clock: {}", self.clock.borrow().read());
    }
}