mod args;
use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat, imageops::FilterType::Triangle, GenericImageView};
use std::{io::BufReader, fs::File};
use std::convert::TryInto;

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data: buffer,
            name
        }
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let (image_1, image_format_1) = find_image_from_path(args.image_1);
    let (image_2, image_format_2) = find_image_from_path(args.image_2);

    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardize_size(image_1, image_2);
    let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);
    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 {dim_1} else {dim_2};
}

fn standardize_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("width: {}, height: {}\n", width, height);

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    
}

/*use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);

    println!("{:?}", output(first_number, operator, second_number, result));
}


fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used.")
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}*/