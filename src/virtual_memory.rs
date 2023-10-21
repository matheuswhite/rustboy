pub trait MemoryMappedPeripheral<const S: usize> {
    fn write(&mut self, address: u16, data: u8);
    fn read(&self, address: u16) -> u8;
}

#[derive(Default)]
pub struct VirtualMemory<ROM16, RAM8, RAM4, RAM128, OAM, IO>
where
    ROM16: MemoryMappedPeripheral<0x4000> + Default,
    RAM8: MemoryMappedPeripheral<0x2000> + Default,
    RAM4: MemoryMappedPeripheral<0x1000> + Default,
    RAM128: MemoryMappedPeripheral<0x7F> + Default,
    OAM: MemoryMappedPeripheral<0xA0> + Default,
    IO: MemoryMappedPeripheral<0x80> + Default,
{
    rom_bank0: ROM16,
    rom_bank1: ROM16,
    vram: RAM8,
    external_ram: RAM8,
    wram0: RAM4,
    wram1: RAM4,
    oam: OAM,
    io_regs: IO,
    hram: RAM128,
    ie: bool,
}

impl<ROM16, RAM8, RAM4, RAM128, OAM, IO> VirtualMemory<ROM16, RAM8, RAM4, RAM128, OAM, IO>
where
    ROM16: MemoryMappedPeripheral<0x4_000> + Default,
    RAM8: MemoryMappedPeripheral<0x2_000> + Default,
    RAM4: MemoryMappedPeripheral<0x1_000> + Default,
    RAM128: MemoryMappedPeripheral<0x7F> + Default,
    OAM: MemoryMappedPeripheral<0xA0> + Default,
    IO: MemoryMappedPeripheral<0x80> + Default,
{
    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x3fff => self.rom_bank0.write(address, data),
            0x4000..=0x7fff => self.rom_bank1.write(address - 0x4000, data),
            0x8000..=0x9fff => self.vram.write(address - 0x8000, data),
            0xa000..=0xbfff => self.external_ram.write(address - 0xa000, data),
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

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => self.rom_bank0.read(address),
            0x4000..=0x7fff => self.rom_bank1.read(address - 0x4000),
            0x8000..=0x9fff => self.vram.read(address - 0x8000),
            0xa000..=0xbfff => self.external_ram.read(address - 0xa000),
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

    impl<const S: usize> MemoryMappedPeripheral<S> for Dummy {
        fn write(&mut self, _address: u16, _data: u8) {}

        fn read(&self, _address: u16) -> u8 {
            0xff
        }
    }

    #[test]
    fn new_virtual_memory() {
        let mut vm = VirtualMemory::<Dummy, Dummy, Dummy, Dummy, Dummy, Dummy>::default();

        vm.write(0, 0xff);
        let _data = vm.read(0);
    }
}
