use crate::ram::Ram;
use crate::virtual_memory::MemoryMappedPeripheral;

pub struct Rom<const S: usize> {
    buffer: [u8; S],
}

impl<const S: usize> MemoryMappedPeripheral for Rom<S> {
    fn write(&mut self, _address: u16, _data: u8) {}

    fn read(&self, address: u16) -> u8 {
        self.buffer[address as usize]
    }
}

pub struct Cartridge {
    bank0: Option<Rom<0x4000>>,
    bank1: Option<Rom<0x4000>>,
    ram: Option<Ram<0x2000>>,
}

impl Cartridge {
    pub fn load<const S: usize>(_content: [u8; S]) -> Self {
        todo!()
    }

    pub fn take_bank0(&mut self) -> Rom<0x4000> {
        self.bank0.take().unwrap()
    }

    pub fn take_bank1(&mut self) -> Rom<0x4000> {
        self.bank1.take().unwrap()
    }

    pub fn take_ram(&mut self) -> Option<Ram<0x2000>> {
        self.ram.take()
    }
}
