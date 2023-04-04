mod image_transforms;

use anyhow::Result;
use image::io::Reader;
use image_transforms::filters;

fn main() -> Result<()> {
	let input_image_path = "test.jpg";
	let output_image_path = "output.png";

	// Load
	println!("Loading training image");
	let result = Reader::open(input_image_path);
	let input_image = result?.decode()?;

	// Screw with it
	println!("Screwing with it");
	let input_image = input_image.to_rgb8();

	let filter = filters::edge_multicolor()?;

	let output_image = filter.convolve(&input_image);

	// Save
	println!("Saving output image");
	output_image.save(output_image_path)?;

	Ok(())
}
