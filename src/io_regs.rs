use crate::joypad::JoyPad;
use crate::virtual_memory::MemoryMappedPeripheral;
use alloc::rc::Rc;

#[derive(Default)]
pub struct IoRegs {
    joypad: Rc<JoyPad>,
    boot_rom_en: u8,
}

impl MemoryMappedPeripheral for IoRegs {
    fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0050 => self.boot_rom_en = data,
            _ => todo!(),
        }
    }

    fn read(&self, address: u16) -> u8 {
        match address {
            0x0050 => self.boot_rom_en,
            _ => todo!(),
        }
    }
}

impl IoRegs {
    pub fn joypad_ref(&self) -> Rc<JoyPad> {
        self.joypad.clone()
    }
}
