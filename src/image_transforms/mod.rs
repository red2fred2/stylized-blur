use anyhow::{anyhow, Result};
use image::{imageops::crop, ImageBuffer, Rgb};

use self::filter::Filter;

pub mod filter;
pub mod filters;

type Image = ImageBuffer<Rgb<u8>, Vec<u8>>;

/// Additively combines two images
pub fn add(a: &Image, b: &Image) -> Result<Image> {
	// Check if the images are the same size
	if a.width() != b.width() || a.height() != b.height() {
		return Err(anyhow!("Images being added are not the same size"))
	}

	let width = a.width();
	let height = a.height();

	let mut output_image = ImageBuffer::new(width, height);

	// Go pixel by pixel
	for y in 0..height {
		for x in 0..width {
			let pixel_a = a.get_pixel(x, y);
			let pixel_b = b.get_pixel(x, y);

			let r = pixel_a.0[0] + pixel_b.0[0];
			let g = pixel_a.0[1] + pixel_b.0[1];
			let b = pixel_a.0[2] + pixel_b.0[2];

			let pixel = Rgb([r, g, b]);
			output_image.put_pixel(x, y, pixel);
		}
	}

	Ok(output_image)
}

/// Crops an image as if it went through the given filter
pub fn crop_to_filter(image: &mut Image, filter: &Filter) -> Image {
	let offset = (filter.size() / 2) as u32;
	let x = offset;
	let y = offset;
	let width = image.width() - offset;
	let height = image.height() - offset;

	crop(image, x, y, width, height).to_image()
}
