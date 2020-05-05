#[derive(Debug)]
pub struct BitReversalTable {
    table: Box<[usize]>
}

impl BitReversalTable {
    pub fn new(size: usize) -> Option<Self> {
        if !size.is_power_of_two() {
            return None;
        }

        let mut table = vec![0; size];
        let mut cur_size = 1;

        while cur_size < size {
            let (head, tail) = table.split_at_mut(cur_size);
            for (m, n) in head.iter_mut().zip(tail.iter_mut()) {
                *m *= 2;
                *n = *m + 1;
            }

            cur_size *= 2;
        }

        Some(Self { table: table.into_boxed_slice() })
    }

    /// Permutes `data` based on table values
    ///
    /// Prerequirements:
    /// * `self.len() == data.len()`, otherwise the behaviour is undefined
    pub fn permute<T>(&self, data: &mut [T]) {
        for (i, j) in (0..data.len()).zip(self.table.iter()) {
            if *j > i {
                data.swap(i, *j);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brt_new() {
        let t = BitReversalTable::new(1).unwrap();
        assert_eq!(t.table, vec![0].into_boxed_slice());

        let t = BitReversalTable::new(4).unwrap();
        assert_eq!(
            t.table,
            vec![0, 2, 1, 3].into_boxed_slice(),
        );

        let t = BitReversalTable::new(16).unwrap();
        assert_eq!(
            t.table,
            vec![0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15].into_boxed_slice(),
        );

        assert!(BitReversalTable::new(3).is_none());

        assert!(BitReversalTable::new(0).is_none());
    }

    #[test]
    fn test_permute() {
        let t = BitReversalTable::new(4).unwrap();
        let mut data = vec![0, 1, 2, 3];

        t.permute(&mut data);
        assert_eq!(data, [0, 2, 1, 3]);
        t.permute(&mut data);
        assert_eq!(data, [0, 1, 2, 3]);
    }
}
