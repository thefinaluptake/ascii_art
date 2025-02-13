use image::ImageReader;

fn main() -> anyhow::Result<()> {
    let image = ImageReader::open("half_hyacine.jpg")?.decode()?;

    println!(
        "Successfully loaded image!\nImage size: {} x {}",
        image.width(),
        image.height()
    );
    Ok(())
}
