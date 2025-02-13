use colored::Colorize;
use image::{ImageReader, Rgb};

const ASCII_MATRIX: &'static str =
    "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

fn main() -> anyhow::Result<()> {
    let term_size = termsize::get().unwrap();

    let max_width = term_size.cols;
    let max_height = term_size.rows;

    let image = ImageReader::open("hyacine.jpg")?.decode()?;

    let mut width = image.width();
    let mut height = image.height();

    println!("Successfully loaded image!\nImage size: {width} x {height}");

    if width > (max_width as u32) {
        let div = width as f32 / (max_width as f32);

        width = (width as f32 / div) as u32;
        height = (height as f32 / div) as u32;
    }

    if height > (max_height as u32) {
        let div = height as f32 / max_height as f32;

        width = (width as f32 / div) as u32;
        height = (height as f32 / div) as u32;
    }

    let image = image.resize(width, height, image::imageops::FilterType::Nearest);

    width = image.width();
    height = image.height();

    let image = image.into_rgb8();

    let mut brightness_matrix = vec![vec![0; width as usize]; height as usize];

    for (image_pixel, brightness_pixel) in image
        .rows()
        .flatten()
        .zip(brightness_matrix.iter_mut().flatten())
    {
        // println!("{pixel:?}");

        let brightness = (0.21 * image_pixel.0[0] as f32
            + 0.72 * image_pixel.0[1] as f32
            + 0.07 * image_pixel.0[2] as f32) as u8;

        *brightness_pixel = brightness;
    }

    println!(
        "Successfully constructed brightness matrix!\nBrightness matrix size: {} x {}",
        brightness_matrix[0].len(),
        brightness_matrix.len()
    );

    // for row in &brightness_matrix {
    //     for pix in row {
    //         println!("{pix}");
    //     }
    // }

    let ascii_image: Vec<String> = brightness_matrix
        .iter()
        .map(|row| row.iter().map(|b| get_char_from_brightness(*b)).collect())
        .collect();

    // for line in ascii_image {
    //     for c in line.chars() {
    //         println!("{c}");
    //     }
    // }

    for (c, pixel) in ascii_image
        .iter()
        .flat_map(|line| line.chars().chain(std::iter::once('\n')))
        .zip(
            image
                .rows()
                .flat_map(|row| row.into_iter().chain(std::iter::once(&Rgb([0, 0, 0])))),
        )
    {
        print!(
            "{}",
            <char as Into<String>>::into(c).truecolor(pixel.0[0], pixel.0[1], pixel.0[2])
        );
    }

    Ok(())
}

fn get_char_from_brightness(brightness: u8) -> char {
    let num_chars = ASCII_MATRIX.chars().count();

    let threshold = 256.0 / num_chars as f32;

    let index = (brightness as f32 / threshold).floor() as usize;

    ASCII_MATRIX.chars().nth(index).unwrap()
}
