#![no_std]
#![deny(warnings)]

extern crate alloc;

mod audio;
mod cartridge;
mod cpu;
mod graphics;
pub mod io_regs;
pub mod joypad;
mod ram;
mod serial_data;
pub mod virtual_memory;
