pub struct Clock {
    tics: u32
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            tics: 0,
        }
    }

    pub fn add(&mut self, tics:u32) {
        self.tics += tics;
    }

    pub fn read(&self) -> u32 {
        self.tics
    }

    pub fn reset(&mut self) {
        self.tics = 0;
    }
}

#[cfg(test)]
mod test_clock {
    use super::Clock;

    #[test]
    fn add_and_read_tics() {
        let mut clk = Clock::new();
        assert_eq!(clk.read(), 0);
        clk.add(10);
        assert_eq!(clk.read(), 10);
    }

    #[test]
    fn reset() {
        let mut clk = Clock::new();
        clk.add(10);
        assert_eq!(clk.read(), 10);
        clk.reset();
        assert_eq!(clk.read(), 0);
    }
}