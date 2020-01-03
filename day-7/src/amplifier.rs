use std::iter::once;

use permutohedron::Heap;

use intcode::{IntcodeError, IntcodeInterpreter};

fn get_amplifier_results<'a>(
    code: &[i64],
    mut setting: impl Iterator<Item = &'a i64>,
) -> Option<i64> {
    setting
        .try_fold::<_, _, Result<_, IntcodeError>>(vec![0], |output: Vec<i64>, setting| {
            let vm: IntcodeInterpreter = code.to_vec().into();

            Ok(vm
                .run_with_input(once(setting).chain(output.iter()))?
                .into_output())
        })
        .map(|output| output.first().copied())
        .ok()
        .flatten()
}

pub(crate) fn find_max_power(code: Vec<i64>, mut settings_options: [i64; 5]) -> Option<i64> {
    let settings = Heap::new(&mut settings_options);

    settings
        .filter_map(|setting| get_amplifier_results(&code, setting.iter()))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let code = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(find_max_power(code, [0, 1, 2, 3, 4]).unwrap(), 43210);

        let code = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(find_max_power(code, [0, 1, 2, 3, 4]).unwrap(), 54321);

        let code = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(find_max_power(code, [0, 1, 2, 3, 4]).unwrap(), 65210);
    }
}
