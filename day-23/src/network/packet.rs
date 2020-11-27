use std::iter::IntoIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Packet {
    dst: usize,
    coords: [i64; 2]
}

impl Packet {
    pub fn dst(&self) -> usize {
        self.dst
    }

    pub fn y(&self) -> i64 {
        self.coords[1]
    }
}

impl From<(i64, i64, i64)> for Packet {
    fn from((dst, x, y): (i64, i64, i64)) -> Self {
        Self {
            dst: dst as usize,
            coords: [x, y],
        }
    }
}

impl IntoIterator for Packet {
    type Item = i64;
    type IntoIter = PacketIterator;

    fn into_iter(self) -> Self::IntoIter {
        PacketIterator {
            data: self.coords,
            current: 0,
        }
    }
}

#[derive(Debug)]
pub struct PacketIterator {
    data: [i64; 2],
    current: usize
}

impl Iterator for PacketIterator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        match &mut self.current {
            &mut 2 => {
                None
            }
            idx => {
                let val = self.data[*idx];
                *idx += 1;

                Some(val)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_iteration() {
        let packet: Packet = (1, 2, 3).into();
        assert_eq!(packet, Packet { dst: 1, coords: [2, 3] });
        assert_eq!(packet.dst(), 1);
        assert_eq!(packet.into_iter().collect::<Vec<_>>(), vec![2, 3]);
    }
}
