use std::collections::VecDeque;

use intcode::Interpreter;

pub mod computer;
pub mod packet;
pub mod transmission;

use computer::Computer;
use packet::Packet;
use transmission::Transmission;

pub const NAT: usize = 255;

#[derive(Debug, Clone)]
pub struct Network {
    computers: Vec<Computer>,
    buffer: VecDeque<Transmission>,
    nat: Option<Packet>,
}

impl Network {
    pub fn new(vm: &Interpreter, size: usize) -> Self {
        let computers = (0..size)
            .map(|i| Computer::new(vm.clone(), i as i64))
            .collect();

        Self {
            computers,
            buffer: VecDeque::new(),
            nat: None,
        }
    }

    #[cfg(debug_assertions)]
    pub fn get_processed_opcodes(&self) -> usize {
        self.computers
            .iter()
            .map(Computer::get_processed_opcodes)
            .sum()
    }

    fn is_idle(&self) -> bool {
        self.buffer.is_empty() && self.computers.iter().all(|c| c.inbox_size() == 0)
    }
}

impl Iterator for Network {
    type Item = Transmission;

    fn next(&mut self) -> Option<Self::Item> {
        while self.buffer.is_empty() {
            if self.is_idle() {
                if let Some(packet) = self.nat {
                    self.nat = None;
                    if let Some(computer) = self.computers.get_mut(0) {
                        computer.receive(packet);
                    }

                    return Some(Transmission::new(NAT, packet));
                }
            }

            self.buffer.extend(
                self.computers
                    .iter_mut()
                    .enumerate()
                    .map(|(i, c)| {
                        c.run()
                            .map(move |packets| packets.map(move |p| Transmission::new(i, p)))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .ok()?
                    .into_iter()
                    .flatten(),
            );
        }

        let transmission = self.buffer.pop_front().unwrap();

        if let Some(computer) = self.computers.get_mut(transmission.dst()) {
            computer.receive(transmission.payload())
        } else if transmission.dst() == NAT {
            self.nat = Some(transmission.payload())
        }

        Some(transmission)
    }
}
