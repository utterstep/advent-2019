use std::iter::once;

use permutohedron::Heap;

use intcode::{IntcodeVmError, Interpreter, InterpreterState};

fn get_amplifier_results<'a>(
    code: &[i64],
    mut setting: impl Iterator<Item = &'a i64>,
) -> Option<i64> {
    setting
        .try_fold::<_, _, Result<_, IntcodeVmError>>(vec![0], |output: Vec<i64>, knob| {
            let mut vm: Interpreter = code.to_vec().into();
            vm.run_with_input(once(knob).chain(output.iter()));

            Ok(vm.into_output()?)
        })
        .map(|output| output.first().copied())
        .ok()
        .flatten()
}

fn get_amplifier_loop_results<'a>(
    code: &[i64],
    setting: impl Iterator<Item = &'a i64>,
) -> Option<i64> {
    let mut vms = setting
        .map(|knob| {
            let mut vm: Interpreter = code.to_vec().into();
            vm.run_with_input(once(knob));

            vm
        })
        .collect::<Vec<_>>();

    // 0 — initial input.
    let mut latest_output = 0;

    loop {
        for vm in &mut vms {
            match vm.get_state() {
                // return output if we found halted VM.
                // bases on condition that all VMs halt at the same time, one after another
                InterpreterState::Halted => return Some(latest_output),
                // feed last output to next VM
                InterpreterState::WaitingForInput => {
                    vm.run_with_input(once(&latest_output));

                    latest_output = vm.drain_output().ok()?.next()?;
                }
                // return, discarding error
                // shouldn't be the case, if VM codes and settings are correct
                InterpreterState::Failed(_e) => {
                    #[cfg(debug_assertions)]
                    eprintln!("VM failure: {:?}", _e);

                    return None;
                }
                // unreachable, as every VM is initialized using settings
                InterpreterState::Initial => unreachable!(),
            }
        }
    }
}

pub(crate) fn find_max_power(code: &[i64], mut settings_options: [i64; 5]) -> Option<i64> {
    let settings = Heap::new(&mut settings_options);

    settings
        .filter_map(|setting| get_amplifier_results(code, setting.iter()))
        .max()
}

pub(crate) fn find_max_loop_power(code: &[i64], mut settings_options: [i64; 5]) -> Option<i64> {
    let settings = Heap::new(&mut settings_options);

    settings
        .filter_map(|setting| get_amplifier_loop_results(code, setting.iter()))
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
        assert_eq!(find_max_power(&code, [0, 1, 2, 3, 4]).unwrap(), 43210);

        let code = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(find_max_power(&code, [0, 1, 2, 3, 4]).unwrap(), 54321);

        let code = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(find_max_power(&code, [0, 1, 2, 3, 4]).unwrap(), 65210);
    }

    #[test]
    fn test_loop_examples() {
        let code = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            find_max_loop_power(&code, [5, 6, 7, 8, 9]).unwrap(),
            139629729
        );

        let code = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(find_max_loop_power(&code, [5, 6, 7, 8, 9]).unwrap(), 18216);
    }
}
