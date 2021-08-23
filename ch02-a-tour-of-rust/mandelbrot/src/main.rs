use image::codecs::png::PngEncoder;
use image::ColorType;
use num::Complex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }
    let bounds: (usize, usize) = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels
///
/// The `bounds` argument gives the width and height of the buffer
/// `pixels`, which holds one grayscale pixel per byte.
/// `upper_left` and `lower_right specify points on the complex plane
/// corresponding to the upper-left and lower-right corners of the pixel buffer
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match mandelbrot_escape_iterations(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), Box<dyn Error>> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8)?;
    Ok(())
}

/// Convert the coordinates of a pixel in the output image to a point on the complex plane
///
/// `bounds` is a pair giving the width and height of the output image in pixels
/// `pixel` is a (column, row) pair indicating a particular pixel in the output image
/// `upper_left` and `lower_right` are points on the complex plane designating the area
/// the output image covers
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    let re = upper_left.re + pixel.0 as f64 * width / bounds.0 as f64;
    let im = upper_left.im - pixel.1 as f64 * height / bounds.1 as f64;
    Complex { re, im }
}

/// Parse a pair of comma-separated floating-point numbers as a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    parse_pair(s, ',').map(|(re, im)| Complex { re, im })
}

/// Parse the string `s` as a coordinate pair, e.g.`"400x600"` or `"1.0,5.0"`
///
/// String `s` should have the form <left><sep><right>, where <sep> is the
/// ASCII character given by the `separator` argument and `left` and `right`
/// are both strings that can be parsed by `T::from_str.`
///
/// If `s` has the proper form, return `Some<x,y>`.
/// If `s` does not parse correctly, return `None`
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

/// Test if `c` is in the Mandelbrot set, using at most `limit` iterations
///
/// If `c` is not a member, return Some(i) where i is the number of iterations
/// it took for `c` to leave the circle of radius 2 centered on the origin.
///
/// If `c` is a member, meaning we reached the iteration limit without
/// proving that c is not a member, return None
fn mandelbrot_escape_iterations(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}
