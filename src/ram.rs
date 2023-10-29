use crate::virtual_memory::{MemoryMappedPeripheral, WriteBlock};
use alloc::vec;
use alloc::vec::Vec;

pub struct Ram<const S: usize> {
    buffer: Vec<Vec<u8>>,
    banks: usize,
    actual_bank: usize,
}

impl<const S: usize> Ram<S> {
    pub fn new(banks: usize) -> Self {
        Self {
            buffer: (0..S).map(|_| [0xff; S].to_vec()).collect(),
            banks,
            actual_bank: 0,
        }
    }

    pub fn sel_bank(&mut self, bank: usize) {
        if bank >= self.banks {
            return;
        }

        self.actual_bank = bank;
    }
}

impl<const S: usize> Default for Ram<S> {
    fn default() -> Self {
        Self {
            buffer: vec![[0xff; S].to_vec()],
            banks: 1,
            actual_bank: 0,
        }
    }
}

impl<const S: usize> MemoryMappedPeripheral for Ram<S> {
    fn write(&mut self, address: u16, data: u8) {
        if address >= S as u16 {
            return;
        }

        self.buffer[self.actual_bank][address as usize] = data;
    }

    fn read(&self, address: u16) -> u8 {
        if address >= S as u16 {
            return 0xff;
        }

        self.buffer[self.actual_bank][address as usize]
    }
}

impl<const S: usize> WriteBlock for Ram<S> {
    fn write_block(&mut self, base_address: u16, block: &[u8]) {
        let base_address = base_address as usize;

        for i in 0..block.len() {
            if base_address + i >= S {
                return;
            }

            self.buffer[self.actual_bank][base_address + i] = block[i];
        }
    }
}
