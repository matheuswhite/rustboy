use rustboy::io_regs::IoRegs;
use rustboy::virtual_memory::{MemoryMappedPeripheral, VirtualMemory};

struct Memory<const S: usize> {
    mem: [u8; S],
}

impl<const S: usize> MemoryMappedPeripheral<S> for Memory<S> {
    fn write(&mut self, address: u16, data: u8) {
        self.mem[address as usize] = data;
    }

    fn read(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }
}

impl<const S: usize> Default for Memory<S> {
    fn default() -> Self {
        Self { mem: [0xff; S] }
    }
}

fn main() {
    let mut virtual_memory = VirtualMemory::<
        Memory<0x4_000>,
        Memory<0x2_000>,
        Memory<0x1_000>,
        Memory<0x7F>,
        Memory<0xA0>,
        IoRegs,
    >::default();

    let joypad = virtual_memory.io_regs_ref().joypad_ref();

    virtual_memory.write(0, 0xaf);
    let _ = virtual_memory.read(0);

    println!("Hello, World!");
}
