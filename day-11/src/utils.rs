pub fn minmax<T: Ord + Copy>(iter: impl IntoIterator<Item = T>) -> Option<(T, T)> {
    let mut iter = iter.into_iter();

    let mut min = match iter.next() {
        None => return None,
        Some(value) => value,
    };
    let mut max = min;

    iter.for_each(|value| {
        if value < min {
            min = value;
        } else if value > max {
            max = value;
        }
    });

    Some((min, max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minmax() {
        let data: &[i32] = &[];
        assert_eq!(minmax(data), None);

        assert_eq!(minmax(&[1, 2, 3]), Some((&1, &3)));
        assert_eq!(minmax(&[1]), Some((&1, &1)));
        assert_eq!(minmax(&[543, 23434, 3]), Some((&3, &23434)));
        assert_eq!(minmax(&[2, 2, 2]), Some((&2, &2)));
    }
}
