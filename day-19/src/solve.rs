use std::f64;

use intcode::Interpreter;

const ANGLE_ESTIMATOR_DEPTH: i64 = 100;
const ANGLE_ESTIMATOR_FLOAT: f64 = ANGLE_ESTIMATOR_DEPTH as f64;

const ANGLE_ERR_EPSILON: f64 = 0.9;

pub fn check_point(code: &[i64], (x, y): (i64, i64)) -> bool {
    let mut interpreter = Interpreter::from(code.to_owned());
    interpreter.run_with_input(&[x, y]);

    *interpreter
        .into_output()
        .expect("intcode program failed")
        .first()
        .expect("intcode program didn't emit output")
        == 1
}

pub fn find_square_base(code: &[i64], target_side: i64) -> (i64, i64) {
    let angle = estimate_angle(&code);
    let (mut x, mut y) = angle.estimate_square_base(target_side);

    loop {
        while !check_point(&code, (x, y)) {
            x += 1;
        }

        if check_square(&code, x, y, target_side) {
            break (x, y - target_side + 1);
        } else {
            y += 1;
        }
    }
}

fn check_square(code: &[i64], x: i64, y: i64, target_side: i64) -> bool {
    let delta = target_side - 1;

    check_point(&code, (x, y))
        && check_point(&code, (x + delta, y))
        && check_point(&code, (x, y - delta))
        && check_point(&code, (x + delta, y - delta))
}

#[derive(Debug)]
pub struct Angle {
    left: f64,
    right: f64,
    alpha: f64,
}

impl Angle {
    pub fn estimate_square_base(&self, target_side: i64) -> (i64, i64) {
        let target_side = target_side as f64;

        let h_small = target_side * (1. + self.alpha.tan()) / self.alpha.tan();

        let y = h_small * (self.right / (self.right - self.left));
        let x = self.left * (y / ANGLE_ESTIMATOR_FLOAT);

        (
            (x * ANGLE_ERR_EPSILON) as i64,
            (y * ANGLE_ERR_EPSILON) as i64,
        )
    }
}

pub fn estimate_angle(code: &[i64]) -> Angle {
    let left = {
        let mut left = 0;

        loop {
            if check_point(code, (left, ANGLE_ESTIMATOR_DEPTH)) {
                break left;
            }

            left += 1;
        }
    };

    let right = {
        let mut right = left + 1;

        loop {
            if !check_point(code, (right, ANGLE_ESTIMATOR_DEPTH)) {
                break right - 1;
            }

            right += 1;
        }
    };

    let left = left as f64;
    let right = right as f64;

    let alpha = f64::consts::FRAC_PI_2
        - (f64::consts::PI * 2.
            + ((-ANGLE_ESTIMATOR_FLOAT).atan2(-right) - 0_f64.atan2(left - right)));

    Angle { left, right, alpha }
}
