#![no_std]

// Re-export the LED module
pub mod led;
pub use led::*;

/// Helper function to format MAC address as colon-separated hex
pub fn mac(mac: [u8; 6]) -> heapless::String<18> {
    use heapless::String;
    use core::fmt::Write;
    
    let mut formatted = String::new();
    for (i, byte) in mac.iter().enumerate() {
        if i > 0 {
            let _ = write!(formatted, ":");
        }
        let _ = write!(formatted, "{:02x}", byte);
    }
    formatted
}

/// Wrapper type for MAC addresses that implements Debug and defmt::Format with hex formatting
#[derive(Clone, Copy)]
pub struct MacAddress(pub [u8; 6]);

impl core::fmt::Debug for MacAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
               self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
    }
}

impl defmt::Format for MacAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                      self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]);
    }
}

impl From<[u8; 6]> for MacAddress {
    fn from(mac: [u8; 6]) -> Self {
        MacAddress(mac)
    }
}