use num::Complex;

const BASE: [i64; 4] = [0, 1, 0, -1];
const BASE_COMPLEX: [Complex<f64>; 4] = [
    Complex::new(0., 0.),
    Complex::new(1., 0.),
    Complex::new(0., 0.),
    Complex::new(-1., 0.),
];

#[derive(Debug)]
pub struct PatternIterator<T: 'static + Clone>
{
    base: std::iter::Cycle<std::slice::Iter<'static, T>>,
    current_item: T,
    times_left: usize,
    n_repeat: usize,
}

macro_rules! impl_pattern {
    ($type: ty, $base: ident) => {
        impl Iterator for PatternIterator<$type>
        {
            type Item = $type;

            fn next(&mut self) -> Option<Self::Item> {
                if self.times_left > 0 {
                    self.times_left -= 1;
                } else {
                    self.current_item = *self.base.next()?;
                    self.times_left = self.n_repeat - 1;
                }

                Some(self.current_item)
            }
        }

        impl PatternIterator<$type>
        {
            pub fn new(n_repeat: usize) -> Self {
                let mut i = $base.iter().cycle();
                let current_item = *(i.next().unwrap());

                Self {
                    base: i,
                    current_item,
                    times_left: n_repeat - 1,
                    n_repeat,
                }
            }
        }
    };
}

impl_pattern!(i64, BASE);
impl_pattern!(Complex<f64>, BASE_COMPLEX);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern() {
        let p = PatternIterator::<i64>::new(3);
        assert_eq!(
            p.take(11).collect::<Vec<_>>(),
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]
        );

        let p = PatternIterator::<i64>::new(2);
        assert_eq!(
            p.take(15).collect::<Vec<_>>(),
            vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1],
        );

        let p = PatternIterator::<i64>::new(1);
        assert_eq!(
            p.take(8).collect::<Vec<_>>(),
            vec![1, 0, -1, 0, 1, 0, -1, 0],
        );
    }
}
