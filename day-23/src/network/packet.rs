use std::iter::IntoIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Packet {
    dst: i64,
    x: i64,
    y: i64,
}

impl Packet {
    pub fn dst(&self) -> i64 {
        self.dst
    }
}

impl From<(i64, i64, i64)> for Packet {
    fn from((dst, x, y): (i64, i64, i64)) -> Self {
        Self { dst, x, y }
    }
}

impl IntoIterator for Packet {
    type Item = i64;
    type IntoIter = PacketIterator;

    fn into_iter(self) -> Self::IntoIter {
        PacketIterator::Data {
            data: [self.x, self.y],
            current: 0,
        }
    }
}

#[derive(Debug)]
pub enum PacketIterator {
    Empty,
    Data { data: [i64; 2], current: usize },
}

impl Iterator for PacketIterator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        match self {
            Self::Empty => None,
            Self::Data { data, current } => match current {
                &mut 2 => {
                    *self = Self::Empty;

                    None
                }
                idx => {
                    let val = data[*idx];
                    *idx += 1;

                    Some(val)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_iteration() {
        let packet: Packet = (1, 2, 3).into();
        assert_eq!(packet, Packet { dst: 1, x: 2, y: 3 });
        assert_eq!(packet.dst(), 1);
        assert_eq!(packet.into_iter().collect::<Vec<_>>(), vec![2, 3]);
    }
}
