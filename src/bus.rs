pub trait BusDevice {
    fn read(&self, address: u16) -> u8 { self.peek(address) }
    fn write(&mut self, _address: u16, _value: u8) {}
    fn get_base_address(&self) -> u16;
    fn get_size(&self) -> u16;
    fn peek(&self, _address: u16) -> u8 { 0xFF }
    fn poke(&mut self, _address: u16, _value: u8) {}
    fn write_vec(&mut self, _address: u16, _data: Vec<u8>) {}
    fn read_word(&self, _address: u16) -> u16 { 0xFFFF }
}

pub struct Bus {
    devices: Vec<Box<dyn BusDevice>>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            devices: vec![],
        }
    }

    pub fn add_device(&mut self, device: Box<dyn BusDevice>) -> Result<(), String> {
        match self.is_available(device.get_base_address(), device.get_size()) {
            true => {
                self.devices.push(device);        
                Ok(())
            }
            false => Err(format!("Component already exists at address {}", device.get_base_address())),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match self.find_device(address) {
            Some(device) => device.read(address),
            None => 0xFF
        }
    }
    
    pub fn write(&mut self, address: u16, value: u8) {
        match self.find_mut_device(address) {
            Some(device) => device.write(address, value),
            None => (),
        }
    }

    pub fn peek(&self, address: u16) -> u8 {
        match self.find_device(address) {
            Some(device) => device.peek(address),
            None => 0xFF
        }
    }

    pub fn write_vec(&mut self, address: u16, data: Vec<u8>) {
        match self.find_mut_device(address) {
            Some(device) => device.write_vec(address, data),
            None => ()
        }
    }
    
    pub fn read_word(&self, address: u16) -> u16 {
        match self.find_device(address) {
            Some(device) => device.read_word(address),
            None => 0xFFFF
        }   
    }

    fn find_device(&self, address: u16) -> Option<&Box<dyn BusDevice>> {
        for device in &self.devices {
            if device.get_base_address() <= address && address < device.get_base_address() + device.get_size() {
                return Some(device);
            }
        }

        None
    }

    fn find_mut_device(&mut self, address: u16) -> Option<&mut Box<dyn BusDevice>> {
        for device in &mut self.devices {
            if device.get_base_address() <= address && address < device.get_base_address() + device.get_size() {
                return Some(device);
            }
        }

        None
    }

    fn is_available(&self, address: u16, size: u16) -> bool {
        if self.devices.is_empty() {
            return true;
        }

        for device in &self.devices {
            if
                (device.get_base_address() <= address && address < device.get_base_address() + device.get_size()) ||
                (device.get_base_address() <= address + size && address + size < device.get_base_address() + device.get_size()) ||
                (address <= device.get_base_address() && device.get_base_address() < address + size) ||
                (address <= device.get_base_address() + device.get_size() - 1 && device.get_base_address() + size < address + size) {
                return false;
            }
        }

        true
    }
}


#[cfg(test)]
mod test_bus {
    use std::{cell::RefCell, rc::Rc};

    use crate::{clock::Clock, device::ram::Ram};

    use super::{Bus, BusDevice};

    struct TestDevice {
        base_address: u16,
        size: u16
    }
    impl TestDevice {
        fn new(base_address: u16, size: u16) -> Self {
            Self {
                base_address,
                size
            }
        }
    }
    impl BusDevice for TestDevice {
        fn get_base_address(&self) -> u16 {
            self.base_address
        }

        fn get_size(&self) -> u16 {
            self.size
        }
    }
    
    #[test]
    fn test_add_device() {
        let mut bus = Bus::new();
        assert!(bus.add_device(Box::new(TestDevice::new(0x0000, 0x100))).is_ok());
        assert!(bus.add_device(Box::new(TestDevice::new(0x0000, 0x100))).is_err());
        assert!(bus.add_device(Box::new(TestDevice::new(0x0100, 0x100))).is_ok());
        assert!(bus.add_device(Box::new(TestDevice::new(0x0150, 0x100))).is_err());
        assert!(bus.add_device(Box::new(TestDevice::new(0x01FF, 0x100))).is_err());
        assert!(bus.add_device(Box::new(TestDevice::new(0x0300, 0x100))).is_ok());
        assert!(bus.add_device(Box::new(TestDevice::new(0x0200, 0x500))).is_err());
        assert!(bus.add_device(Box::new(TestDevice::new(0x0200, 0x100))).is_err());
        assert!(bus.add_device(Box::new(TestDevice::new(0x0200, 0x110))).is_err());
    }
    
    #[test]
    fn test_routing() {
        let clock = Rc::new(RefCell::new(Clock::new()));
        let mut bus = Bus::new();
        assert!(bus.add_device(Box::new(Ram::new(0x0000, 0x100, Rc::clone(&clock)))).is_ok());
        assert!(bus.add_device(Box::new(Ram::new(0x0100, 0x100, Rc::clone(&clock)))).is_ok());
        bus.write(0x0000, 0x11);
        bus.write(0x0100, 0x22);
        assert_eq!(bus.read(0x0000), 0x11);
        assert_eq!(bus.read(0x00FF), 0x00);
        assert_eq!(bus.read(0x0100), 0x22);
        assert_eq!(bus.read(0x0101), 0x00);
        assert_eq!(bus.read(0x1000), 0xFF);
    }
}