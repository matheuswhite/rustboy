use crate::joypad::JoyPad;
use crate::virtual_memory::MemoryMappedPeripheral;
use alloc::rc::Rc;

#[derive(Default)]
pub struct IoRegs {
    joypad: Rc<JoyPad>,
}

impl MemoryMappedPeripheral<0x80> for IoRegs {
    fn write(&mut self, _address: u16, _data: u8) {
        todo!()
    }

    fn read(&self, _address: u16) -> u8 {
        todo!()
    }
}

impl IoRegs {
    pub fn joypad_ref(&self) -> Rc<JoyPad> {
        self.joypad.clone()
    }
}
