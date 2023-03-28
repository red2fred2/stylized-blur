mod filter;
mod test_filters;

use anyhow::Result;
use image::io::Reader;

fn main() -> Result<()> {
	let training_image_path = "building.jpg";
	let output_image_path = "output.png";

	// Load
	println!("Loading training image");
	let result = Reader::open(training_image_path);
	let training_image = result?.decode()?;

	// Screw with it
	println!("Screwing with it");
	let training_image = training_image.to_rgb8();

	let filter = test_filters::edge_multicolor()?;

	let output_image = filter.convolve(&training_image);

	// Save
	println!("Saving output image");
	output_image.save(output_image_path)?;

	Ok(())
}
