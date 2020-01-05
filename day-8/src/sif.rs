use itertools::iproduct;
use std::{
    fmt::{self, Display},
    ops::Index,
};

#[derive(Debug)]
pub struct Layer {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            iproduct!(0..self.height, 0..self.width)
                .map(|(y, x)| {
                    let pixel = match self[(x, y)] {
                        0 => " ",
                        1 => "*",
                        2 => "#",
                        _ => "?",
                    };

                    format!("{1}{0}", pixel, if x == 0 && y > 0 { "\n" } else { "" })
                })
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl Index<(usize, usize)> for Layer {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.pixels[y * self.width + x]
    }
}

#[derive(Debug)]
pub struct SpaceImage {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl SpaceImage {
    pub fn from_slice(width: usize, height: usize, pixels: &[u8]) -> Self {
        let layers = pixels
            .chunks_exact(width * height)
            .map(|chunk| Layer {
                pixels: chunk.to_vec(),
                width,
                height,
            })
            .collect();

        Self {
            width,
            height,
            layers,
        }
    }

    pub fn checksum(&self) -> Option<usize> {
        let (ones, twos) = self
            .layers
            .iter()
            .min_by_key(|l| l.pixels.iter().filter(|&&p| p == 0).count())?
            .pixels
            .iter()
            .fold((0, 0), |(ones, twos), &p| {
                if p == 1 {
                    (ones + 1, twos)
                } else if p == 2 {
                    (ones, twos + 1)
                } else {
                    (ones, twos)
                }
            });

        Some(ones * twos)
    }

    pub fn compose(&self) -> Layer {
        let pixels =
            iproduct!(0..self.height, 0..self.width).fold(Vec::new(), |mut pixels, (y, x)| {
                let value = self
                    .layers
                    .iter()
                    .find_map(|l| {
                        let pixel = l[(x, y)];

                        if pixel != 2 {
                            Some(pixel)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(2);

                pixels.push(value);
                pixels
            });

        Layer {
            width: self.width,
            height: self.height,
            pixels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_example() {
        let pixels: Vec<_> = "123456789012"
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let image = SpaceImage::from_slice(3, 2, pixels.as_slice());

        assert_eq!(image.checksum(), Some(1));
    }

    #[test]
    fn test_composition() {
        let pixels: Vec<_> = "0222112222120000"
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let image = SpaceImage::from_slice(2, 2, pixels.as_slice());

        assert_eq!(format!("{}", image.compose()), "*.\n.*");
    }
}
