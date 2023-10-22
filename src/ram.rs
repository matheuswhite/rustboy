use crate::virtual_memory::MemoryMappedPeripheral;

pub struct Ram<const S: usize> {
    buffer: [u8; S],
}

impl<const S: usize> Default for Ram<S> {
    fn default() -> Self {
        Self { buffer: [0xff; S] }
    }
}

impl<const S: usize> MemoryMappedPeripheral for Ram<S> {
    fn write(&mut self, address: u16, data: u8) {
        if address >= S as u16 {
            return;
        }

        self.buffer[address as usize] = data;
    }

    fn read(&self, address: u16) -> u8 {
        if address >= S as u16 {
            return 0xff;
        }

        self.buffer[address as usize]
    }
}
