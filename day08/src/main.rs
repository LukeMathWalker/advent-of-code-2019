use std::alloc::handle_alloc_error;
use std::str::FromStr;
use ndarray::{Array2, Array3, Axis, ArrayView2};

fn read_input(path: &str) -> Vec<u8> {
    let input = std::fs::read_to_string(path).expect("Failed to read input");
    let digits = input
        .trim()
        .chars()
        .map(|s| u8::from_str(&s.to_string()).expect("Failed to parse digit"))
        .collect();
    digits
}

fn get_layers(width: usize, height: usize, digits: Vec<u8>) -> Array3<u8> {
    let n_layers = digits.len() / (width * height);
    Array3::from_shape_vec((n_layers, width, height), digits).unwrap()
}

fn checksum(layers: &Array3<u8>) -> u32 {
    let layer = layers
        .axis_iter(Axis(0))
        .min_by_key(|l| count_digit(&l, 0))
        .expect("Failed to find maximum layer");
    count_digit(&layer, 1) * count_digit(&layer, 2)
}

fn count_digit(l: &ArrayView2<u8>, digit: u8) -> u32 {
    l.iter().map(|&d| {
        if d == digit {
            1
        } else {
            0
        }
    }).sum()
}

fn get_image(layers: Array3<u8>) -> Array2<u8> {
    layers.map_axis(Axis(0), |pixels| {
        for &pixel in pixels.iter() {
            if pixel != 2 {
                return pixel;
            }
        }
        2
    })
}

fn main() {
    let digits = read_input("input.txt");
    let width = 6;
    let height = 25;

    let layers = get_layers(width, height, digits);
    println!("Checksum: {:?}", checksum(&layers));
    println!("Image:\n{:?}", get_image(layers));
}
