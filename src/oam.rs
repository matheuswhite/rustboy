use crate::virtual_memory::MemoryMappedPeripheral;

pub struct Oam {
    _buffer: [u8; 0xA0],
}

impl Default for Oam {
    fn default() -> Self {
        Self {
            _buffer: [0xff; 0xA0],
        }
    }
}

impl MemoryMappedPeripheral for Oam {
    fn write(&mut self, _address: u16, _data: u8) {
        todo!()
    }

    fn read(&self, _address: u16) -> u8 {
        todo!()
    }
}
