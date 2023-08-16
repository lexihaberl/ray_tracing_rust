use std::fs::File;
use std::io::{BufWriter, Error, ErrorKind, Write};

use crate::Color;

#[derive(Debug)]
pub struct Vec2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

pub type Canvas = Vec2D<Color>;

impl Canvas {
    pub fn create_canvas(width: usize, height: usize) -> Canvas {
        let data: Vec<Color> = vec![Color::new(0.0, 0.0, 0.0); width * height];
        Canvas {
            data,
            width,
            height,
        }
    }

    pub fn write_pixel(&mut self, width: usize, height: usize, color: Color) {
        let idx = self.width * height;
        self.data[idx + width] = color;
    }

    pub fn read_pixel(&self, width: usize, height: usize) -> Color {
        let idx = self.width * height;
        self.data[idx + width]
    }

    pub fn to_ppm(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;

        let mut writer = BufWriter::new(file);

        let ppm_str = match self.to_ppm_str() {
            Ok(str) => str,
            Err(err) => return Err(Error::new(ErrorKind::WriteZero, err)),
        };
        writer.write_all(ppm_str.as_bytes())?;
        writer.flush()?;

        Ok(())
    }

    fn to_ppm_str(&self) -> Result<String, std::fmt::Error> {
        use std::fmt::Write;

        let mut ppm_str = String::new();
        writeln!(ppm_str, "P3")?;
        writeln!(ppm_str, "{} {}", self.width, self.height)?;
        writeln!(ppm_str, "255")?;
        // max 70 chars per line allowed, numbers have at most 3 chars due to u8
        // thus max 16*4 + 1*3 = 67 (spaces after numbers) aka 17 numbers possible per line
        let mut chars_written = 0;
        for (idx, color) in self.data.iter().enumerate() {
            let r: u8 = (color.r * 255.0).round().clamp(0.0, 255.0) as u8;
            let g: u8 = (color.g * 255.0).round().clamp(0.0, 255.0) as u8;
            let b: u8 = (color.b * 255.0).round().clamp(0.0, 255.0) as u8;

            let color_values = [r, g, b];
            for value in color_values {
                let mut chars_to_be_added = match value {
                    0..=9 => 1,
                    10..=99 => 2,
                    _ => 3,
                };
                // 1 char for ' ' or '\n'
                chars_to_be_added += 1;
                if chars_written + chars_to_be_added > 70 {
                    ppm_str.pop();
                    ppm_str.push('\n');
                    chars_written = chars_to_be_added;
                } else {
                    chars_written += chars_to_be_added;
                }
                write!(ppm_str, "{} ", value)?;
            }
            // newline if new row begins in the next iteration
            if (idx + 1) % self.width == 0 {
                ppm_str.pop();
                ppm_str.push('\n');
                chars_written = 0;
            }
        }
        Ok(ppm_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let canvas = Canvas::create_canvas(20, 8);
        assert_eq!(canvas.read_pixel(19, 7), Color::new(0.0, 0.0, 0.0))
    }

    #[test]
    fn setting_pixel() {
        let mut canvas = Canvas::create_canvas(20, 8);
        canvas.write_pixel(19, 7, Color::new(1.2, 0.2, 0.0));
        assert_eq!(canvas.read_pixel(19, 7), Color::new(1.2, 0.2, 0.0))
    }

    #[test]
    fn ppm_format_test() {
        let mut canvas = Canvas::create_canvas(5, 3);
        canvas.write_pixel(
            0,
            0,
            Color {
                r: 1.5,
                g: 0.0,
                b: 0.0,
            },
        );
        canvas.write_pixel(
            2,
            1,
            Color {
                r: 0.0,
                g: 0.5,
                b: 0.0,
            },
        );
        canvas.write_pixel(
            4,
            2,
            Color {
                r: -0.5,
                g: 0.0,
                b: 1.0,
            },
        );
        let ppm_str = canvas.to_ppm_str().unwrap();
        let ppm_str_expected = "P3\n\
        5 3\n\
        255\n\
        255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
        assert_eq!(ppm_str, ppm_str_expected)
    }

    #[test]
    fn ppm_long_line_splitting() {
        let mut canvas = Canvas::create_canvas(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for row in 0..10 {
            for col in 0..2 {
                canvas.write_pixel(row, col, color);
            }
        }

        let ppm_str = canvas.to_ppm_str().unwrap();
        let ppm_str_expected = "P3\n\
        10 2\n\
        255\n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153 255 204 153 255 204 153 255 204 153\n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153 255 204 153 255 204 153 255 204 153\n";
        assert_eq!(ppm_str, ppm_str_expected)
    }

    #[test]
    fn ppm_ends_with_newline() {
        let canvas = Canvas::create_canvas(5, 3);
        let str = canvas.to_ppm_str().unwrap();
        assert_eq!(str.chars().nth_back(0), Some('\n'))
    }
}
