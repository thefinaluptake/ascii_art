use image::ImageReader;

const ASCII_MATRIX: &'static str =
    "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

fn main() -> anyhow::Result<()> {
    let image = ImageReader::open("half_hyacine.jpg")?.decode()?;

    let mut width = image.width();
    let mut height = image.height();

    println!("Successfully loaded image!\nImage size: {width} x {height}");

    if width > 220 {
        let div = width as f32 / 220.;

        width = (width as f32 / div) as u32;
        height = (height as f32 / div) as u32;
    }

    let image = image.resize(width, height, image::imageops::FilterType::Nearest);

    let image = image.into_rgb8();

    let mut brightness_matrix = vec![vec![0; width as usize]; height as usize];

    for (image_pixel, brightness_pixel) in image
        .rows()
        .flatten()
        .zip(brightness_matrix.iter_mut().flatten())
    {
        // println!("{pixel:?}");

        let brightness =
            (((image_pixel.0[0] as f32) + (image_pixel.0[1] as f32) + (image_pixel.0[2] as f32))
                / 3.0) as u8;

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

    for line in ascii_image {
        println!("{line}");
    }

    Ok(())
}

fn get_char_from_brightness(brightness: u8) -> char {
    let num_chars = ASCII_MATRIX.chars().count();

    let threshold = 256.0 / num_chars as f32;

    let index = (brightness as f32 / threshold).floor() as usize;

    ASCII_MATRIX.chars().nth(index).unwrap()
}
