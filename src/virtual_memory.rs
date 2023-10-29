use crate::cartridge::{Cartridge, Rom};
use crate::joypad::JoyPad;
use crate::ram::Ram;

pub trait MemoryMappedPeripheral {
    fn write(&mut self, address: u16, data: u8);
    fn read(&self, address: u16) -> u8;
}

pub trait WriteBlock {
    fn write_block(&mut self, base_address: u16, block: &[u8]);
}

pub trait ReadBlock {
    fn read_block<const S: usize>(&self, base_address: u16) -> [u8; S];
}

pub struct VirtualMemory {
    boot_rom: Rom<0x100>,
    rom_bank0: Rom<0x4000>,
    rom_bank1: Rom<0x4000>,
    vram: Ram<0x2000>,
    external_ram: Option<Ram<0x2000>>,
    wram0: Ram<0x1000>,
    wram1: Ram<0x1000>,
    oam: Ram<0xA0>,
    joypad: JoyPad,
    boot_rom_en: u8,
    oam_dma: u8,
    hram: Ram<0x7F>,
    ie: bool,
}

impl VirtualMemory {
    const BOOT_ROM: &'static [u8] = include_bytes!("DMG_ROM.bin");

    pub fn new(mut cartridge: Cartridge) -> Self {
        Self {
            boot_rom: Rom::new(VirtualMemory::BOOT_ROM.to_vec(), 1),
            rom_bank0: cartridge.take_bank0(),
            rom_bank1: cartridge.take_bank1(),
            vram: Ram::default(),
            external_ram: cartridge.take_ram(),
            wram0: Ram::default(),
            wram1: Ram::default(),
            oam: Ram::default(),
            joypad: JoyPad::default(),
            boot_rom_en: 0x01,
            oam_dma: 0x00,
            hram: Ram::default(),
            ie: true,
        }
    }

    pub fn joypad_ref(&self) -> &JoyPad {
        &self.joypad
    }

    fn write_io_regs(&mut self, address: u16, data: u8) {
        match address {
            0x0000 => self.joypad.write(address, data),
            0x0001..=0x0002 => todo!("serial transfer write"),
            0x0004..=0x0007 => todo!("timer and driver write"),
            0x0010..=0x0026 => todo!("audio write"),
            0x0030..=0x003F => todo!("wave pattern write"),
            0x0040..=0x0045 | 0x0047..=0x004B => todo!("graphics write"),
            0x0046 => {
                self.oam_dma = data;

                let base_address = (data as u16) << 8;
                let source = self.read_block::<0xA0>(base_address);
                self.oam.write_block(0x0000, &source);
            }
            0x0050 => self.boot_rom_en = data,
            _ => panic!("IO register address cannot be written"),
        }
    }

    fn read_io_regs(&self, address: u16) -> u8 {
        match address {
            0x0000 => self.joypad.read(address),
            0x0001..=0x0002 => todo!("serial transfer read"),
            0x0004..=0x0007 => todo!("timer and driver read"),
            0x0010..=0x0026 => todo!("audio read"),
            0x0030..=0x003F => todo!("wave pattern read"),
            0x0040..=0x0045 | 0x0047..=0x004B => todo!("graphics read"),
            0x0046 => self.oam_dma,
            0x0050 => self.boot_rom_en,
            _ => panic!("IO register address cannot be read"),
        }
    }
}

impl MemoryMappedPeripheral for VirtualMemory {
    fn write(&mut self, address: u16, data: u8) {
        let boot_rom_en = self.boot_rom_en == 0x00;

        match address {
            0x0000..=0x00ff if boot_rom_en => self.boot_rom.write(address, data),
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
            0xff00..=0xff7f => self.write_io_regs(address - 0xff00, data),
            0xff80..=0xfffe => self.hram.write(address - 0xff80, data),
            0xffff => self.ie = data != 0,
        }
    }

    fn read(&self, address: u16) -> u8 {
        let boot_rom_en = self.boot_rom_en == 0x00;

        match address {
            0x0000..=0x00ff if boot_rom_en => self.boot_rom.read(address),
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
            0xff00..=0xff7f => self.read_io_regs(address - 0xff00),
            0xff80..=0xfffe => self.hram.read(address - 0xff80),
            0xffff => self.ie as u8,
        }
    }
}

impl ReadBlock for VirtualMemory {
    fn read_block<const S: usize>(&self, base_address: u16) -> [u8; S] {
        let mut output = [0xFF; S];

        for i in 0..S {
            output[i] = self.read(base_address + i as u16);
        }

        output
    }
}
