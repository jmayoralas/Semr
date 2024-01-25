
pub trait BusDevice {
    fn read(&self, address: u16) -> u8 { 0xFF }
    fn write(&mut self, address: u16, value: u8) {}
}

struct BusComponent {
    base_address: u16,
    size: u16,
    device: Box<dyn BusDevice>,
}

pub struct Bus {
    components: Vec<BusComponent>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            components: vec![],
        }
    }

    pub fn add_device(&mut self, base_address: u16, size: u16, device: Box<dyn BusDevice>) -> Result<(), String> {
        match self._is_available(base_address, size) {
            true => {
                self.components.push(BusComponent {
                        base_address,
                        size,
                        device,
                    });
                    
                    Ok(())
                }
            false => Err(format!("Component already exists at address {}", base_address)),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match self._find_component(address) {
            Some(component) => component.device.read(address),
            None => 0xFF
        }
    }

    fn _find_component(&self, address: u16) -> Option<&BusComponent> {
        for component in &self.components {
            if component.base_address <= address && address < component.base_address + component.size {
                return Some(component);
            }
        }

        None
    }

    fn _is_available(&self, address: u16, size: u16) -> bool {
        if self.components.is_empty() {
            return true;
        }

        for component in &self.components {
            if
                (component.base_address <= address && address < component.base_address + component.size) ||
                (component.base_address <= address + size && address + size < component.base_address + component.size) ||
                (address <= component.base_address && component.base_address < address + size) ||
                (address <= component.base_address + component.size - 1 && component.base_address + size < address + size) {
                return false;
            }
        }

        true
    }
}


#[cfg(test)]
mod test_bus {
    use super::{Bus, BusDevice};

    
    #[test]
    fn test_add_device() {
        struct TestDevice;
        impl BusDevice for TestDevice {}
        
        let mut bus = Bus::new();
        assert!(bus.add_device(0x0000, 0x100, Box::new(TestDevice)).is_ok());
        assert!(bus.add_device(0x0000, 0x100, Box::new(TestDevice)).is_err());
        assert!(bus.add_device(0x0100, 0x100, Box::new(TestDevice)).is_ok());
        assert!(bus.add_device(0x0150, 0x100, Box::new(TestDevice)).is_err());
        assert!(bus.add_device(0x01FF, 0x100, Box::new(TestDevice)).is_err());
        assert!(bus.add_device(0x0300, 0x100, Box::new(TestDevice)).is_ok());
        assert!(bus.add_device(0x0200, 0x500, Box::new(TestDevice)).is_err());
        assert!(bus.add_device(0x0200, 0x100, Box::new(TestDevice)).is_err());
        assert!(bus.add_device(0x0200, 0x110, Box::new(TestDevice)).is_err());
    }
    
    #[test]
    fn test_read() {
        struct TestDeviceA;
        impl BusDevice for TestDeviceA {
            fn read(&self, _address: u16) -> u8 {
                0x11
            }
        }

        struct TestDeviceB;
        impl BusDevice for TestDeviceB {
            fn read(&self, _address: u16) -> u8 {
                0x22
            }
        }

        let mut bus = Bus::new();
        assert!(bus.add_device(0x0000, 0x100, Box::new(TestDeviceA)).is_ok());
        assert!(bus.add_device(0x0100, 0x100, Box::new(TestDeviceB)).is_ok());
        assert_eq!(bus.read(0x0000), 0x11);
        assert_eq!(bus.read(0x00FF), 0x11);
        assert_eq!(bus.read(0x0100), 0x22);
        assert_eq!(bus.read(0x01FF), 0x22);
        assert_eq!(bus.read(0x0200), 0xFF);
    }
}