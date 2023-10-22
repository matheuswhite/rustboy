use rustboy::cartridge::Cartridge;
use rustboy::virtual_memory::{MemoryMappedPeripheral, VirtualMemory};

fn load_cartridge_from_file(path: &str) -> std::io::Result<Vec<u8>> {
    Ok(std::fs::read_to_string(path)?.as_bytes().to_vec())
}

fn main() -> std::io::Result<()> {
    let cartridge = load_cartridge_from_file("test.gb")?;
    let cartridge = Cartridge::load(&cartridge);
    let mut virtual_memory = VirtualMemory::new(cartridge);
    let _joypad = virtual_memory.io_regs_ref().joypad_ref();

    virtual_memory.write(0, 0xaf);
    let _ = virtual_memory.read(0);

    Ok(())
}
