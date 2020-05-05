use std::f64::consts::PI;

use num::{Complex, traits::identities::{One, Zero}};

mod brt;

use brt::BitReversalTable;

use crate::pattern::PatternIterator;

pub trait MultExt<T> {
    fn mult(self, other: &[T]);
}

impl MultExt<i64> for &mut [i64] {
    fn mult(self, other: &[i64]) {
        self.iter_mut().zip(other.iter()).for_each(|(a, &b)| *a *= b);
    }
}

impl MultExt<f64> for &mut [f64] {
    fn mult(self, other: &[f64]) {
        self.iter_mut().zip(other.iter()).for_each(|(a, &b)| *a *= b);
    }
}

impl MultExt<Complex<f64>> for &mut [Complex<f64>] {
    fn mult(self, other: &[Complex<f64>]) {
        self.iter_mut().zip(other.iter()).for_each(|(a, &b)| *a *= b);
    }
}

pub fn flawed_ft(data: &[i64]) -> Vec<i64> {
    (1..=data.len()).map(|i| {
        let pattern = PatternIterator::<i64>::new(i);

        data.iter().zip(pattern).map(|(&a, b)| a * b).sum::<i64>().abs() % 10
    }).collect()
}

pub struct Fft {
    table: BitReversalTable
}

impl Fft {
    pub fn new(size: usize) -> Option<Self> {
        Some(Self {
            table: BitReversalTable::new(size)?
        })
    }

    pub fn compute_complex(&self, data: &mut [Complex<f64>], inverse: bool)
    {
        self.table.permute(data);
        let n = data.len();
        let mut len = 2;
        let angle_sign = if inverse { -1. } else { 1. };

        while len <= n {
            let angle = (2. * PI / len as f64) * angle_sign;
            let wlen = Complex::new(angle.cos(), angle.sin());

            let mut i = 0;
            while i < n {
                let mut w = Complex::one();

                for j in 0..(len / 2) {
                    let u = data[i + j];
                    let v = data[i + j + len / 2] * w;

                    data[i + j] = u + v;
                    data[i + j + len / 2] = u - v;

                    w *= wlen;
                }

                i += len;
            }

            len <<= 1;
        }

        if inverse {
            for v in data.iter_mut() {
                *v /= n as f64;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::pattern::PatternIterator;

    // #[test]
    // fn test_fft_complex() {
    //     let fft = Fft::new(16).unwrap();

    //     let mut input = [1., 2., 3., 4., 5., 6., 7., 8.].iter().map(Complex::from).collect::<Vec<_>>();
    //     let mut data = [Complex::zero(); 16];
    //     data[input.len()..].swap_with_slice(&mut input);
    //     fft.compute_complex(&mut data, false);

    //     // let mut pattern = [0., 0., 0., 0., 0., 0., 0., 1.].iter().map(Complex::from).collect::<Vec<_>>();
    //     let mut pattern = PatternIterator::<Complex<f64>>::new(1).take(8).collect::<Vec<_>>();
    //     let mut pdata = [Complex::zero(); 16];
    //     pdata[pattern.len()..].swap_with_slice(&mut pattern);
    //     fft.compute_complex(&mut pdata, false);

    //     data.mult(&pdata);
    //     fft.compute_complex(&mut data, true);

    //     println!("{:?}", data);
    //     assert_eq!(data.iter().map(|c| c.re.round()).sum::< f64>(), 4.);
    // }

    #[test]
    fn test_flawed_ft() {
        assert_eq!(
            flawed_ft(&[1, 2, 3, 4, 5, 6, 7, 8]),
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        );
    }
}
