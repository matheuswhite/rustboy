use crate::virtual_memory::MemoryMappedPeripheral;
use core::sync::atomic::{AtomicU8, Ordering};

pub struct JoyPad {
    buttons: AtomicU8,
    register: u8,
}

impl Default for JoyPad {
    fn default() -> Self {
        Self {
            buttons: AtomicU8::new(0),
            register: 0x3F,
        }
    }
}

impl JoyPad {
    pub fn update_button_state(&self, button: JoyPadButton, state: bool) {
        self.buttons
            .fetch_or((state as u8) << button as usize, Ordering::SeqCst);
    }
}

pub enum JoyPadButton {
    Down,
    Up,
    Left,
    Right,
    Start,
    Select,
    B,
    A,
}

impl MemoryMappedPeripheral for JoyPad {
    fn write(&mut self, _address: u16, data: u8) {
        if data & 0x20 != 0 {
            self.register |= 0x20;
        } else {
            self.register &= 0x20u8.reverse_bits();
        }

        if data & 0x10 != 0 {
            self.register |= 0x10;
        } else {
            self.register &= 0x10u8.reverse_bits();
        }
    }

    fn read(&self, _address: u16) -> u8 {
        if (self.register & 0x20) == 0 {
            0b0010_0000 | (self.buttons.load(Ordering::Acquire) >> 4)
        } else if (self.register & 0x10) == 0 {
            0b0001_0000 | (self.buttons.load(Ordering::Acquire) & 0x0f)
        } else {
            0x3F
        }
    }
}
