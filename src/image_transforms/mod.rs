use anyhow::Result;
use image::{ImageBuffer, Pixel, Rgba, imageops::crop};

use crate::image_transforms::helpers::combine;

use self::filter::Filter;

pub mod helpers;
pub mod filter;
pub mod filters;

/// Additively combines the images
pub fn add<P1, P2>(a: &ImageBuffer<P1, Vec<f32>>, b: &ImageBuffer<P2, Vec<f32>>)
-> Result<ImageBuffer<Rgba<f32>, Vec<f32>>>
where P1: Pixel + Pixel<Subpixel = f32>,
P2: Pixel + Pixel<Subpixel = f32> {
	combine(a, b, add_pixel)
}

#[allow(unused)]
fn add_pixel<P1, P2>(a: &P1, b: &P2, _: u32, _: u32)
-> Rgba<f32>
where P1: Pixel + Pixel<Subpixel = f32>,
P2: Pixel + Pixel<Subpixel = f32> {
	let a = a.to_rgba();
	let a = a.channels();
	let b = b.to_rgba();
	let b = b.channels();

	Rgba([
		a[0] * a[3] + b[0] *b[3],
		a[1] * a[3] + b[1] *b[3],
		a[2] * a[3] + b[2] *b[3],
		a[3] + b[3]
	])
}

/// Crops an image as if it went through the given filter
pub fn crop_to_filter<P>(image: &mut ImageBuffer<P, Vec<f32>>, filter: &Filter)
-> ImageBuffer<P, Vec<f32>>
where P: 'static + Pixel + Pixel<Subpixel = f32> {
	let offset = (filter.size() / 2) as u32;
	let x = offset;
	let y = offset;
	let width = image.width() - offset;
	let height = image.height() - offset;

	crop(image, x, y, width, height).to_image()
}
