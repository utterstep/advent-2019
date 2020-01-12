use std::error::Error;

use itertools::iproduct;

use advent_utils::{get_config, read_file, Part};

const VIEW_DISTANCE: i64 = 50;

mod solve;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let code_str = read_file(config.input_file)?;

    let code: Vec<_> = code_str
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    match config.part {
        Part::One => {
            let affected_points = iproduct!(0..VIEW_DISTANCE, 0..VIEW_DISTANCE)
                // iproduct actually produces order (y, x),
                // but is doesn't matter in case area is symmetric (like this one, 50x50)
                .map(|point| solve::check_point(&code, point))
                .filter(|&output| output)
                .count();

            println!("points are affected by tractor ray: {}", affected_points);
        }
        Part::Two => {
            let (x, y) = solve::find_square_base(&code, 100);
            println!("square base is at: {}", x * 10000 + y);
        }
    }

    Ok(())
}
