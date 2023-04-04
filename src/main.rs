mod image_transforms;

use anyhow::Result;
use image::io::Reader;
use image_transforms::{add, filters};

use crate::image_transforms::crop_to_filter;

fn main() -> Result<()> {
	let input_image_path = "Vending_Machines.jpg";
	let output_image_path = "output.png";

	// Load
	println!("Loading training image");
	let result = Reader::open(input_image_path);
	let input_image = result?.decode()?;

	// Screw with it
	println!("Screwing with it");
	let mut input_image = input_image.to_rgb8();

	let filter = filters::edge_5x5()?;

	let edges = filter.convolve(&input_image);
	let cropped = crop_to_filter(&mut input_image, &filter);

	let output_image = add(&edges, &cropped)?;

	// Save
	println!("Saving output image");
	output_image.save(output_image_path)?;

	Ok(())
}
