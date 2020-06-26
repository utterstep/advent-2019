use std::f64;

#[inline]
fn gcd(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();

    loop {
        let next = a % b;

        if next == 0 {
            break b;
        }

        a = b;
        b = next;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct RationalAngle {
    divident: i32,
    divisor: i32,
}

impl RationalAngle {
    pub fn new(divident: i32, divisor: i32) -> Option<Self> {
        if divident == 0 && divisor == 0 {
            return None;
        } else if divisor == 0 {
            return Some(RationalAngle {
                divident: divident.signum(),
                divisor: 0,
            });
        }

        let gcd = gcd(divident, divisor);

        Some(RationalAngle {
            divident: divident / gcd,
            divisor: divisor / gcd,
        })
    }

    pub fn angle(self) -> f64 {
        (f64::from(self.divisor).atan2(f64::from(self.divident)) + f64::consts::FRAC_PI_2)
            .rem_euclid(f64::consts::PI * 2.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(148, 92), 4);
        assert_eq!(gcd(148, -92), 4);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(4, 1), 1);
        assert_eq!(gcd(4, 5), 1);
        assert_eq!(gcd(0, 5), 5);
    }
}
