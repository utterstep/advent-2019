use std::iter::once;

use intcode::{IntcodeVmError, Interpreter};
use itertools::Itertools;

use super::packet::Packet;

#[derive(Debug)]
pub struct Computer {
    vm: Interpreter,
    inbox: Vec<Packet>,
}

impl Computer {
    pub fn new(mut vm: Interpreter, address: i64) -> Self {
        vm.run_with_input(once(address));

        Self {
            vm,
            inbox: Vec::new(),
        }
    }

    pub fn receive(&mut self, packet: Packet) {
        self.inbox.push(packet);
    }

    pub fn run(&mut self) -> Result<impl Iterator<Item = Packet> + '_, IntcodeVmError> {
        if self.inbox.is_empty() {
            self.vm.run_with_input(once(-1));
        } else {
            self.vm.run_with_input(self.inbox.drain(..).flatten());
        }

        self.vm
            .drain_output()
            .map(|iter| iter.tuples::<(_, _, _)>().map(Packet::from))
    }

    #[cfg(debug_assertions)]
    pub fn get_processed_opcodes(&self) -> usize {
        self.vm.get_processed_opcodes()
    }
}
