use crate::cartridge::{Cartridge, Rom};
use crate::io_regs::IoRegs;
use crate::oam::Oam;
use crate::ram::Ram;

pub trait MemoryMappedPeripheral {
    fn write(&mut self, address: u16, data: u8);
    fn read(&self, address: u16) -> u8;
}

pub struct VirtualMemory {
    rom_bank0: Rom<0x4000>,
    rom_bank1: Rom<0x4000>,
    vram: Ram<0x2000>,
    external_ram: Option<Ram<0x2000>>,
    wram0: Ram<0x1000>,
    wram1: Ram<0x1000>,
    oam: Oam,
    io_regs: IoRegs,
    hram: Ram<0x7F>,
    ie: bool,
}

impl VirtualMemory {
    pub fn new(mut cartridge: Cartridge) -> Self {
        Self {
            rom_bank0: cartridge.take_bank0(),
            rom_bank1: cartridge.take_bank1(),
            vram: Ram::default(),
            external_ram: cartridge.take_ram(),
            wram0: Ram::default(),
            wram1: Ram::default(),
            oam: Oam::default(),
            io_regs: IoRegs::default(),
            hram: Ram::default(),
            ie: true,
        }
    }

    pub fn io_regs_ref(&self) -> &IoRegs {
        &self.io_regs
    }
}

impl MemoryMappedPeripheral for VirtualMemory {
    fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x3fff => self.rom_bank0.write(address, data),
            0x4000..=0x7fff => self.rom_bank1.write(address - 0x4000, data),
            0x8000..=0x9fff => self.vram.write(address - 0x8000, data),
            0xa000..=0xbfff => {
                if let Some(external_ram) = self.external_ram.as_mut() {
                    external_ram.write(address - 0xa000, data)
                }
            }
            0xc000..=0xcfff => self.wram0.write(address - 0xc000, data),
            0xd000..=0xdfff => self.wram1.write(address - 0xd000, data),
            0xe000..=0xfdff => self.wram0.write(address - 0xe000, data),
            0xfe00..=0xfe9f => self.oam.write(address - 0xfe00, data),
            0xfea0..=0xfeff => panic!("Write at prohibited memory area [{address}:{data}]"),
            0xff00..=0xff7f => self.io_regs.write(address - 0xff00, data),
            0xff80..=0xfffe => self.hram.write(address - 0xff80, data),
            0xffff => self.ie = data != 0,
        }
    }

    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => self.rom_bank0.read(address),
            0x4000..=0x7fff => self.rom_bank1.read(address - 0x4000),
            0x8000..=0x9fff => self.vram.read(address - 0x8000),
            0xa000..=0xbfff => {
                if let Some(external_ram) = self.external_ram.as_ref() {
                    external_ram.read(address - 0xa000)
                } else {
                    0xff
                }
            }
            0xc000..=0xcfff => self.wram0.read(address - 0xc000),
            0xd000..=0xdfff => self.wram1.read(address - 0xd000),
            0xe000..=0xfdff => self.wram0.read(address - 0xe000),
            0xfe00..=0xfe9f => self.oam.read(address - 0xfe00),
            0xfea0..=0xfeff => panic!("Read at prohibited memory area [{address}]"),
            0xff00..=0xff7f => self.io_regs.read(address - 0xff00),
            0xff80..=0xfffe => self.hram.read(address - 0xff80),
            0xffff => self.ie as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    pub struct Dummy;

    impl MemoryMappedPeripheral for Dummy {
        fn write(&mut self, _address: u16, _data: u8) {}

        fn read(&self, _address: u16) -> u8 {
            0xff
        }
    }

    #[test]
    fn new_virtual_memory() {
        let cartridge = Cartridge::load([]);
        let mut vm = VirtualMemory::new(cartridge);

        vm.write(0, 0xff);
        let _data = vm.read(0);
    }
}
