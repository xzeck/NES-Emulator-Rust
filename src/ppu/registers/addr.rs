/*

    Address Register (0x2006)

    Process -   
    
    If the CPU wants to access memory at cell 0x1000 in PPU memory
    1. load requesting address into addr register
        - write twice to load 2 bytes into 1 byte register
    2. CPU can request data from data register 0x2007
    3. CPU has to read from 0x2007  twice  to get all the data from PPU's internal buffer

    Note:

    Since CHR ROM and RAM are treated as external devices by the PPU, 
    it is unable to provide an immediate value return. 
    Instead, the PPU must fetch the data and store it in its internal buffer. 
    When the CPU performs its first read from the address 0x2007, 
    it retrieves the content stored in this internal buffer, 
    which was filled during the previous load operation. 
    This read is considered a dummy read from the perspective of the CPU.

 */

pub struct AddrRegister {
    value: (u8, u8),
    hi_ptr: bool,
}

impl AddrRegister {
    pub fn new() -> Self {
        AddrRegister {
            value: (0, 0),
            hi_ptr: true,
        }
    }

    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    pub fn update(&mut self, data: u8) {

        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        if self.get() > 0x3fff {

            self.set(self.get() & 0b11111111111111);
        }

        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);

        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }

        // mirror down address above 0x3fff
        if self.get() > 0x3fff {
            self.set(self.get() & 0b11111111111111);
        }
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }

    pub fn get(&self) -> u16 { 
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }
}