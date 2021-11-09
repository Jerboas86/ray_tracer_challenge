use std::{
    fmt::{self, Display},
    fs::File,
    io::Write,
    ops::{Add, Mul, Sub},
};

const RGB: f32 = 255.;

#[derive(Clone, Copy)]
enum Primary {
    Red,
    Blue,
    Green,
}

enum ColorFormat {
    Raw(f32, f32, f32),
    RGB(u8, u8, u8),
}

#[derive(Clone)]
pub struct Color(f32, f32, f32);

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self(red, green, blue)
    }

    fn get_raw(&self) -> ColorFormat {
        ColorFormat::Raw(self.0, self.1, self.2)
    }

    fn get_rgb(&self) -> ColorFormat {
        ColorFormat::RGB(
            (self.0 * RGB) as u8,
            (self.1 * RGB) as u8,
            (self.2 * RGB) as u8,
        )
    }
}

impl Add for &Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        let red = self.0 + rhs.0;
        let green = self.1 + rhs.1;
        let blue = self.2 + rhs.2;

        Color::new(red, green, blue)
    }
}

impl Sub for &Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        let red = self.0 - rhs.0;
        let green = self.1 - rhs.1;
        let blue = self.2 - rhs.2;

        Color::new(red, green, blue)
    }
}

impl Mul for &Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        let red = self.0 * rhs.0;
        let green = self.1 * rhs.1;
        let blue = self.2 * rhs.2;

        Color::new(red, green, blue)
    }
}

impl Mul<f32> for &Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        let red = self.0 * rhs;
        let green = self.1 * rhs;
        let blue = self.2 * rhs;

        Color::new(red, green, blue)
    }
}

impl Mul<&Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        let red = rhs.0 * self;
        let green = rhs.1 * self;
        let blue = rhs.2 * self;

        Color::new(red, green, blue)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        if let ColorFormat::RGB(red, green, blue) = self.get_rgb() {
            write!(f, "{} {} {}", red, green, blue)
        } else {
            Err(fmt::Error)
        }
    }
}
pub struct Ppm(String);

impl Ppm {
    fn stringify(cv: &Canvas) -> Self {
        let mut ppm = String::new();

        Self::header(&mut ppm, cv);
        Self::serialize_colors(&mut ppm, cv);
        Self::ends_with_new_line(&mut ppm);

        Self(ppm)
    }

    fn header(buffer: &mut String, cv: &Canvas) {
        let header = format!("P3\n{} {}\n{}\n", cv.width, cv.height, RGB);
        buffer.push_str(&header);
    }

    fn serialize_colors(buffer: &mut String, cv: &Canvas) {
        let mut counter = 0;
        for c in cv {
            let rgb = c.to_string();
            let rgb_len = rgb.len();

            // prevent adding an overadded color and keep lines to 70 chars max
            if counter > (cv.width * rgb_len) || (counter + rgb_len + 1) >= 69 {
                buffer.push('\n');
                counter = 0
            }

            if counter != 0 {
                buffer.push(' ');
            }

            counter += rgb_len + 1;
            buffer.push_str(&rgb);
        }
    }

    fn ends_with_new_line(buffer: &mut String) {
        if !buffer.ends_with('\n') {
            buffer.push('\n');
        }
    }

    pub fn write_to_file(&self, file: &str) {
        let mut f = File::create(file).expect("Unable to create file");
        f.write_all(self.0.as_bytes())
            .expect("Unable to write data");
    }
}

impl Display for Ppm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Option<Color>) -> Self {
        match color {
            Some(c) => Self {
                width,
                height,
                buffer: vec![c; width * height],
            },
            None => Self {
                width,
                height,
                buffer: vec![Color::new(0., 0., 0.); width * height],
            },
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    fn len(&self) -> usize {
        self.height * self.width
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + self.width * y
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        let idx = self.get_index(x, y);
        let pixel = self.buffer.get_mut(idx);
        if let Some(px) = pixel {
            *px = color.clone();
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        let idx = self.get_index(x, y);
        &self.buffer[idx]
    }

    pub fn to_ppm(&self) -> Ppm {
        Ppm::stringify(self)
    }
}

pub struct CanvasIter<'a> {
    inner: &'a Canvas,
    index: usize,
}

impl<'a> CanvasIter<'a> {
    fn new(canvas: &'a Canvas) -> Self {
        Self {
            inner: canvas,
            index: 0,
        }
    }
}

impl<'a> Iterator for CanvasIter<'a> {
    type Item = &'a Color;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            idx if idx < self.inner.len() => {
                let color = &self.inner.buffer[idx];
                self.index += 1;
                Some(color)
            }
            _ => None,
        }
    }
}

impl<'a> IntoIterator for &'a Canvas {
    type Item = &'a Color;

    type IntoIter = CanvasIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CanvasIter::new(self)
    }
}

#[cfg(test)]
mod test {
    use std::f32::EPSILON;

    use float_eq::assert_float_eq;

    use crate::canvas::Color;

    use super::Canvas;

    #[test]
    fn check_color_elements() {
        let c = Color(-0.5, 0.4, 1.7);

        assert_float_eq!(c.0, -0.5, abs <= EPSILON);
        assert_float_eq!(c.1, 0.4, abs <= EPSILON);
        assert_float_eq!(c.2, 1.7, abs <= EPSILON);
    }

    #[test]
    fn add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let c3 = &c1 + &c2;

        let ref_c = Color::new(1.6, 0.7, 1.);

        assert_float_eq!(c3.0, ref_c.0, abs <= EPSILON);
        assert_float_eq!(c3.1, ref_c.1, abs <= EPSILON);
        assert_float_eq!(c3.2, ref_c.2, abs <= EPSILON);
    }

    #[test]
    fn substract_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let c3 = &c1 - &c2;

        let ref_c = Color::new(0.2, 0.5, 0.5);

        assert_float_eq!(c3.0, ref_c.0, abs <= EPSILON);
        assert_float_eq!(c3.1, ref_c.1, abs <= EPSILON);
        assert_float_eq!(c3.2, ref_c.2, abs <= EPSILON);
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let s = 2.;

        let c2 = &c1 * s;

        let ref_c = Color::new(0.4, 0.6, 0.8);

        assert_float_eq!(c2.0, ref_c.0, abs <= EPSILON);
        assert_float_eq!(c2.1, ref_c.1, abs <= EPSILON);
        assert_float_eq!(c2.2, ref_c.2, abs <= EPSILON);
    }

    #[test]
    fn multiply_colors() {
        let c1 = Color::new(1., 0.2, 0.4);
        let c2 = Color::new(0.9, 1., 0.1);

        let c3 = &c1 * &c2;

        let ref_c = Color::new(0.9, 0.2, 0.04);

        assert_float_eq!(c3.0, ref_c.0, abs <= EPSILON);
        assert_float_eq!(c3.1, ref_c.1, abs <= EPSILON);
        assert_float_eq!(c3.2, ref_c.2, abs <= EPSILON);
    }

    #[test]
    fn creates_a_canvas() {
        let canv = Canvas::new(10, 20, None);

        assert_eq!(canv.width, 10);
        assert_eq!(canv.height, 20);
        assert_eq!(canv.len(), 20 * 10);

        for c in &canv {
            assert_float_eq!(c.0, 0., abs <= EPSILON);
            assert_float_eq!(c.1, 0., abs <= EPSILON);
            assert_float_eq!(c.2, 0., abs <= EPSILON);
        }
    }

    #[test]
    fn writes_pixels_to_canvas() {
        let mut canv = Canvas::new(10, 20, None);
        let red = Color::new(1., 0., 0.);

        canv.write_pixel(2, 3, &red);
        let c = canv.pixel_at(2, 3);

        assert_float_eq!(c.0, red.0, abs <= EPSILON);
        assert_float_eq!(c.1, red.1, abs <= EPSILON);
        assert_float_eq!(c.2, red.2, abs <= EPSILON);
    }

    #[test]
    fn check_header_correctness() {
        let canv = Canvas::new(10, 20, None);

        let ppm = canv.to_ppm().to_string();

        let mut ppm_lines = ppm.lines();

        assert_eq!(ppm_lines.next(), Some("P3"));
        assert_eq!(ppm_lines.next(), Some("10 20"));
        assert_eq!(ppm_lines.next(), Some("255"));
    }

    #[test]
    fn check_stringified_color_correctness() {
        let mut canv = Canvas::new(5, 3, None);

        let c1 = Color::new(1.5, 0., 0.);
        let c2 = Color::new(0., 0.5, 0.);
        let c3 = Color::new(-0.5, 0., 1.);

        canv.write_pixel(0, 0, &c1);
        canv.write_pixel(2, 1, &c2);
        canv.write_pixel(4, 2, &c3);

        let ppm = canv.to_ppm().to_string();

        let mut ppm_lines = ppm.lines().skip(3);

        assert_eq!(ppm_lines.next(), Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"));
        assert_eq!(ppm_lines.next(), Some("0 0 0 0 0 0 0 127 0 0 0 0 0 0 0"));
        assert_eq!(ppm_lines.next(), Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));
    }

    #[test]
    fn splitting_long_lines_in_ppm_without_cutting_color_in_half() {
        let color = Color::new(1., 0.8, 0.6);
        let canv = Canvas::new(10, 2, Some(color));

        let ppm = canv.to_ppm().to_string();

        let mut ppm_lines = ppm.lines().skip(3);

        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"),
            "first line"
        );
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"),
            "second line"
        );
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"),
            "third line"
        );
        assert_eq!(
            ppm_lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"),
            "fourth line"
        );
    }

    #[test]
    fn ppm_ends_with_new_line() {
        let canv = Canvas::new(5, 3, None);

        let ppm = canv.to_ppm().to_string();

        assert!(ppm.ends_with('\n'));
    }
}
