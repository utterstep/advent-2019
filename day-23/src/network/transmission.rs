use super::Packet;

#[derive(Debug)]
pub struct Transmission {
    src: usize,
    payload: Packet,
}

impl Transmission {
    pub fn new(src: usize, payload: Packet) -> Self {
        Self { src, payload }
    }

    pub fn src(&self) -> usize {
        self.src
    }

    pub fn dst(&self) -> usize {
        self.payload.dst()
    }

    pub fn payload(&self) -> Packet {
        self.payload
    }
}
