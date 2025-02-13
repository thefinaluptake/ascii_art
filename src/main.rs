use image::ImageReader;

fn main() -> anyhow::Result<()> {
    let image = ImageReader::open("half_hyacine.jpg")?.decode()?;

    let width = image.width();
    let height = image.height();

    println!("Successfully loaded image!\nImage size: {width} x {height}");

    let image = image.into_rgb8();

    for row in image.rows() {
        for pixel in row {
            // println!("{pixel:?}");
        }
    }

    Ok(())
}
