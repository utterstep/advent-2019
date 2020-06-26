use itertools::Itertools;

use crate::space::Command;

#[derive(Debug)]
pub struct CompressionDict {
    pub a: String,
    pub b: String,
    pub c: String,
}

#[inline(always)]
fn n_digits_div(mut n: usize) -> usize {
    (2..)
        .take_while(|_| {
            n /= 10;
            n > 0
        })
        .last()
        .unwrap_or(1)
}

fn measure_cmd(cmd: &Command) -> usize {
    match cmd {
        Command::MoveForward(steps) => n_digits_div(*steps),
        Command::TurnLeft | Command::TurnRight => 1,
    }
}

fn slice_contains<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    let end = haystack
        .len()
        .checked_sub(needle.len())
        .map_or(0, |end| end + 1);

    (0..end).any(|i| &haystack[i..(i + needle.len())] == needle)
}

fn find_subslice<'a, T: PartialEq>(
    haystack: &'a [T],
    needle: &'a [T],
) -> impl Iterator<Item = usize> + 'a {
    let end = haystack
        .len()
        .checked_sub(needle.len())
        .map_or(0, |end| end + 1);

    (0..end).filter(move |&i| &haystack[i..(i + needle.len())] == needle)
}

pub fn compress(commands: &[Command], len_limit: usize) -> Option<(CompressionDict, String)> {
    let mut used_cmds = Vec::new();
    used_cmds.resize(commands.len(), false);

    let mut slices = vec![];
    let mut cur_start = 0;

    while cur_start < commands.len() {
        let mut cur_len = measure_cmd(&commands[cur_start]);
        let mut cur_slice_len = 0;

        while cur_start + cur_slice_len < commands.len() - 1 {
            let next_end = cur_start + cur_slice_len + 1;
            let next_cmd = &commands[next_end];
            cur_len += 1 + measure_cmd(next_cmd);

            if cur_len > len_limit
                || !slice_contains(&commands[next_end..], &commands[cur_start..next_end])
                || used_cmds[cur_start..next_end].iter().any(|&used| used)
            {
                break;
            }

            cur_slice_len += 1;
        }

        if cur_slice_len > 0 {
            let subslice = &commands[cur_start..(cur_start + cur_slice_len)];
            find_subslice(&commands, subslice).for_each(|start| {
                used_cmds[start..(start + cur_slice_len)]
                    .iter_mut()
                    .for_each(|used| *used = true);
            });
            slices.push(subslice);
        }

        cur_start += cur_slice_len.max(1);
    }

    if slices.len() == 3 {
        let dict = CompressionDict {
            a: slices[0].iter().map(String::from).join(","),
            b: slices[1].iter().map(String::from).join(","),
            c: slices[2].iter().map(String::from).join(","),
        };

        let compressed = commands
            .iter()
            .map(String::from)
            .join(",")
            .replace(&dict.a, "A")
            .replace(&dict.b, "B")
            .replace(&dict.c, "C");

        Some((dict, compressed))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::TestResult;

    #[quickcheck]
    fn n_digits(n: usize) -> bool {
        n_digits_div(n) == format!("{}", n).len()
    }

    #[quickcheck]
    fn slice_contains_self(arr: Vec<u8>) -> bool {
        slice_contains(&arr, &arr)
    }

    #[quickcheck]
    fn slice_contains_empty_slice(arr: Vec<u8>) -> bool {
        slice_contains(&arr, &[])
    }

    #[quickcheck]
    fn slice_contains_tail(arr: Vec<u8>) -> TestResult {
        if arr.len() < 2 {
            return TestResult::discard();
        }

        TestResult::from_bool(slice_contains(&arr, &arr[1..]))
    }

    #[quickcheck]
    fn slice_contains_head(arr: Vec<u8>) -> TestResult {
        if arr.len() < 2 {
            return TestResult::discard();
        }

        TestResult::from_bool(slice_contains(&arr, &arr[..(arr.len() - 1)]))
    }

    #[quickcheck]
    fn empty_slice_contains(arr: Vec<u8>) -> bool {
        !slice_contains(&[], &arr) || arr.len() == 0
    }

    #[quickcheck]
    fn slice_contains_negative(arr: Vec<u8>) -> bool {
        let arr = arr.into_iter().map(|i| i as u16).collect::<Vec<_>>();

        // u8 couldn't be greater than 255
        !slice_contains(&arr, &[256])
    }

    #[quickcheck]
    fn find_subslice_self(arr: Vec<u8>) -> bool {
        find_subslice(&arr, &arr).collect::<Vec<_>>() == vec![0]
    }

    #[quickcheck]
    fn find_subslice_repeatative(arr: Vec<u8>) -> TestResult {
        if arr.len() < 1 {
            return TestResult::discard();
        }

        let arr_rep = arr
            .iter()
            .cloned()
            .cycle()
            .take(arr.len() * 10)
            .collect::<Vec<_>>();

        TestResult::from_bool(
            find_subslice(&arr_rep, &arr).collect::<Vec<_>>()
                == (0..10).map(|i| i * arr.len()).collect::<Vec<_>>(),
        )
    }

    #[quickcheck]
    fn find_subslice_negative(arr: Vec<u8>) -> bool {
        let arr = arr.into_iter().map(|i| i as u16).collect::<Vec<_>>();

        // u8 couldn't be greater than 255
        find_subslice(&arr, &[256]).count() == 0
    }
}
