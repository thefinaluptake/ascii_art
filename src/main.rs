use image::ImageReader;

const ASCII_MATRIX: &'static str =
    "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

fn main() -> anyhow::Result<()> {
    let image = ImageReader::open("half_hyacine.jpg")?.decode()?;

    let width = image.width();
    let height = image.height();

    println!("Successfully loaded image!\nImage size: {width} x {height}");

    let image = image.into_rgb8();

    let mut brightness_matrix = vec![vec![0; width as usize]; height as usize];

    for (image_row, brightness_row) in image.rows().zip(brightness_matrix.iter_mut()) {
        for (image_pixel, brightness_pixel) in image_row.zip(brightness_row.iter_mut()) {
            // println!("{pixel:?}");

            let brightness = (((image_pixel.0[0] as f32)
                + (image_pixel.0[1] as f32)
                + (image_pixel.0[2] as f32))
                / 3.0) as u8;

            *brightness_pixel = brightness;
        }
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

    Ok(())
}
