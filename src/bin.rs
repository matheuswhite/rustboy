use rustboy::cartridge::Cartridge;
use rustboy::virtual_memory::{MemoryMappedPeripheral, VirtualMemory};

fn main() {
    let cartridge = Cartridge::load([]);
    let mut virtual_memory = VirtualMemory::new(cartridge);
    let _joypad = virtual_memory.io_regs_ref().joypad_ref();

    virtual_memory.write(0, 0xaf);
    let _ = virtual_memory.read(0);

    println!("Hello, World!");
}
