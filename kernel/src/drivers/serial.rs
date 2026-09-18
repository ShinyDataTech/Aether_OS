//! UART 16550 Serial Driver for Debug Log & Intent Stream Ingestion

pub struct SerialDriver {
    pub io_port: u16,
}

impl SerialDriver {
    pub fn new(io_port: u16) -> Self {
        Self { io_port }
    }

    pub fn write_str(&mut self, s: &str) {
        // Output via serial port
        for _b in s.bytes() {
            // Write byte to port
        }
    }
}
