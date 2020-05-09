use std::collections::VecDeque;

use intcode::Interpreter;

pub mod computer;
pub mod packet;

use computer::Computer;
use packet::Packet;

#[derive(Debug)]
pub struct Network {
    computers: Vec<Computer>,
    buffer: VecDeque<Packet>,
}

impl Network {
    pub fn new(vm: Interpreter, size: usize) -> Self {
        let computers = (0..size)
            .map(|i| Computer::new(vm.clone(), i as i64))
            .collect();

        Self {
            computers,
            buffer: VecDeque::new(),
        }
    }

    #[cfg(debug_assertions)]
    pub fn get_processed_opcodes(&self) -> usize {
        self.computers.iter().map(Computer::get_processed_opcodes).sum()
    }
}

impl Iterator for Network {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        while self.buffer.is_empty() {
            self.buffer.extend(
                self.computers
                    .iter_mut()
                    .map(Computer::run)
                    .collect::<Result<Vec<_>, _>>()
                    .ok()?
                    .into_iter()
                    .flatten(),
            );
        }

        let packet = self.buffer.pop_front().unwrap();

        if let Some(computer) = self.computers.get_mut(packet.dst() as usize) {
            computer.receive(packet)
        }

        Some(packet)
    }
}
