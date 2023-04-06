use anyhow::{anyhow, Result};
use image::{ImageBuffer, Pixel, Luma, LumaA, Rgb, Rgba};

/// Do some operation to combine two images
#[allow(unused)]
pub fn combine<F, P1, P2, P3>(a: &ImageBuffer<P1, Vec<f32>>, b: &ImageBuffer<P2, Vec<f32>>, operation: F)
-> Result<ImageBuffer<P3, Vec<f32>>>
where F: Fn(&P1, &P2, u32, u32) -> P3,
P1: Pixel + Pixel<Subpixel = f32>,
P2: Pixel + Pixel<Subpixel = f32>,
P3: Pixel + Pixel<Subpixel = f32> {
	// Check if images are the same size
	if a.dimensions() != b.dimensions() {
		return Err(anyhow!("Images being combined are not the same size"))
	}

	let width = a.width();
	let height = a.height();

	let mut out = ImageBuffer::new(width, height);

	for x in 0..width {
		for y in 0..height {
			let pixel_a = a.get_pixel(x, y);
			let pixel_b = b.get_pixel(x, y);
			let pixel_out = operation(pixel_a, pixel_b, x, y);
			out.put_pixel(x, y, pixel_out);
		}
	}

	Ok(out)
}

/// Do some operation on an image
#[allow(unused)]
pub fn operate<F, P1, P2>(image: &ImageBuffer<P1, Vec<f32>>, operation: F)
-> ImageBuffer<P2, Vec<f32>>
where F: Fn(&P1, u32, u32) -> P2,
P1: Pixel + Pixel<Subpixel = f32>,
P2: Pixel + Pixel<Subpixel = f32> {
	let width = image.width();
	let height = image.height();

	let mut out = ImageBuffer::new(width, height);

	for x in 0..width {
		for y in 0..height {
			let pixel = image.get_pixel(x, y);
			let pixel_out = operation(pixel, x, y);
			out.put_pixel(x, y, pixel_out);
		}
	}

	out
}

/// Converts this image to a luma representation
#[allow(unused)]
pub fn to_luma<P>(image: &ImageBuffer<P, Vec<f32>>)
-> ImageBuffer<Luma<f32>, Vec<f32>>
where P: Pixel + Pixel<Subpixel = f32> {
	operate(image, |p, _, _| p.to_luma())
}

/// Converts this image to a luma representation with alpha channel
#[allow(unused)]
pub fn to_luma_alpha<P>(image: &ImageBuffer<P, Vec<f32>>)
-> ImageBuffer<LumaA<f32>, Vec<f32>>
where P: Pixel + Pixel<Subpixel = f32> {
	operate(image, |p, _, _| p.to_luma_alpha())
}

/// Converts this image to an rgb representation
#[allow(unused)]
pub fn to_rgb<P>(image: &ImageBuffer<P, Vec<f32>>)
-> ImageBuffer<Rgb<f32>, Vec<f32>>
where P: Pixel + Pixel<Subpixel = f32> {
	operate(image, |p, _, _| p.to_rgb())
}

/// Converts this image to an rgb representation with alpha channel
#[allow(unused)]
pub fn to_rgba<P>(image: &ImageBuffer<P, Vec<f32>>)
-> ImageBuffer<Rgba<f32>, Vec<f32>>
where P: Pixel + Pixel<Subpixel = f32> {
	operate(image, |p, _, _| p.to_rgba())
}
