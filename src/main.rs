mod image_transforms;

use anyhow::Result;
use image::{io::Reader, DynamicImage, imageops::crop};

use crate::image_transforms::add;

fn main() -> Result<()> {
	let input_image_path = "Vending_Machines.jpg";
	let output_image_path = "output.png";

	// Load
	println!("Loading image");
	let result = Reader::open(input_image_path);
	let input_image = result?.decode()?;
	let mut input_image = input_image.to_rgba32f();
	let input_image = crop(&mut input_image, 0, 0, 1000, 1000).to_image();

	let result2 = Reader::open("test.jpg");
	let input_image2 = result2?.decode()?;
	let mut input_image2 = input_image2.to_rgba32f();
	let input_image2 = crop(&mut input_image2, 0, 0, 1000, 1000).to_image();

	// Screw with it
	println!("Screwing with it");

	let img = add(&input_image, &input_image2)?;

	// Save
	println!("Saving output image");
	let output_image = DynamicImage::from(img).to_rgb8();
	output_image.save(output_image_path)?;

	Ok(())
}
