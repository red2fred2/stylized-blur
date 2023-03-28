mod filter;

use anyhow::Result;
use image::io::Reader;

use filter::Filter;

fn main() -> Result<()> {
	let training_image_path = "test.jpg";
	let output_image_path = "output.png";

	// Load
	println!("Loading training image");
	let result = Reader::open(training_image_path);
	let training_image = result?.decode()?;

	// Screw with it
	println!("Screwing with it");
	let training_image = training_image.to_rgb8();

	// let kernel = vec![
	// 	vec![0.004, 0.016, 0.024, 0.016, 0.004],
	// 	vec![0.016, 0.064, 0.096, 0.064, 0.016],
	// 	vec![0.024, 0.096, 0.144, 0.096, 0.024],
	// 	vec![0.016, 0.064, 0.096, 0.064, 0.016],
	// 	vec![0.004, 0.016, 0.024, 0.016, 0.004],
	// ];

	let kernel = vec![
		vec![0.04, 0.04, 0.04, 0.04, 0.04],
		vec![0.04, 0.04, 0.04, 0.04, 0.04],
		vec![0.04, 0.04, 0.04, 0.04, 0.04],
		vec![0.04, 0.04, 0.04, 0.04, 0.04],
		vec![0.04, 0.04, 0.04, 0.04, 0.04],
	];

	let filter = Filter::new(kernel.clone(), kernel.clone(), kernel)?;

	let output_image = filter.convolve(&training_image);

	// Save
	println!("Saving output image");
	output_image.save(output_image_path)?;

	Ok(())
}
