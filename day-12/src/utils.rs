fn gcd(mut m: usize, mut n: usize) -> usize {
    while m != 0 {
        let temp = m;
        m = n % temp;
        n = temp;
    }
    n
}

pub fn lcm(m: usize, n: usize) -> usize {
    let g = gcd(m, n);

    g * m / g * n / g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(5, 3), 1);

        assert_eq!(gcd(6, 9), 3);

        assert_eq!(gcd(100, 15), 5);

        assert_eq!(gcd(100, 100), 100);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(12, 8), 24);
    }
}
