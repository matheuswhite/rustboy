use rustboy::cartridge::Cartridge;
use rustboy::virtual_memory::{MemoryMappedPeripheral, VirtualMemory};

fn load_cartridge_from_file(path: &str) -> std::io::Result<Vec<u8>> {
    std::fs::read(path)
}

fn main() -> std::io::Result<()> {
    let cartridge = load_cartridge_from_file("roms/cpu_instrs.gb")?;
    let cartridge = Cartridge::load(&cartridge);
    println!("Cartridge: {}", cartridge);

    let mut virtual_memory = VirtualMemory::new(cartridge);
    let _joypad = virtual_memory.joypad_ref();

    virtual_memory.write(0, 0xaf);
    let _ = virtual_memory.read(0);

    Ok(())
}
