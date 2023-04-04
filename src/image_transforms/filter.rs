use anyhow::{Result, anyhow};
use image::{ImageBuffer, Rgb};

pub struct Filter {
	r: Vec<Vec<f32>>,
	g: Vec<Vec<f32>>,
	b: Vec<Vec<f32>>,
}

impl Filter {
	/// Constructs a 3 channel Filter from the given kernels
	///
	/// The size must be an odd number. These rigid kernels must have a center.
	/// Each kernel must be the same length
	pub fn new(r: Vec<Vec<f32>>, g: Vec<Vec<f32>>, b: Vec<Vec<f32>>) -> Result<Self> {
		if !Self::are_same_size(&r, &g, &b) {
			return Err(anyhow!("Kernels must all be the same size"));
		}

		if r.len() % 2 == 0 {
			return Err(anyhow!("Kernel must be an odd size"));
		}

		Ok(Filter {r, g, b})
	}

	/// Checks if each dimension of the kernels are the same size
	fn are_same_size(r: &Vec<Vec<f32>>, g: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> bool {
		let test_len = r.len();

		if g.len() != test_len || b.len() != test_len {
			return false
		}

		for row in r.iter() {
			if row.len() != test_len {
				return false
			}
		}

		for row in g.iter() {
			if row.len() != test_len {
				return false
			}
		}

		for row in b.iter() {
			if row.len() != test_len {
				return false
			}
		}

		true
	}

	/// Runs this filter on some data
	pub fn convolve(&self, image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
		let padding = self.r.len() as u32 / 2;
		let convolvable_height = image.height() - padding;
		let convolvable_width = image.width() - padding;

		let mut output_image = ImageBuffer::new(convolvable_width, convolvable_height);

		// Go pixel by pixel
		for y in padding..convolvable_height {
			for x in padding..convolvable_width {
				let pixel = self.convolve_pixel(x, y, image);
				output_image.put_pixel(x, y, pixel);
			}
		}

		output_image
	}

	// Runs the convolution filter on one pixel
	fn convolve_pixel(&self, x: u32, y: u32, image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Rgb<u8> {
		let mut r = 0.0;
		let mut g = 0.0;
		let mut b = 0.0;

		// Run through each pixel
		for kernel_y in 0..self.r.len() {
			for kernel_x in 0..self.r.len() {
				let x_offset = kernel_x as i32 - self.r.len() as i32 / 2;
				let y_offset = kernel_y as i32 - self.r.len() as i32 / 2;

				let x = x as i32 + x_offset;
				let y = y as i32 + y_offset;
				let pixel = image.get_pixel(x as u32, y as u32);

				let r_pixel = pixel.0[0];
				let g_pixel = pixel.0[1];
				let b_pixel = pixel.0[2];
				let r_kernel = self.r[kernel_y][kernel_x];
				let g_kernel = self.g[kernel_y][kernel_x];
				let b_kernel = self.b[kernel_y][kernel_x];

				r += r_pixel as f32 * r_kernel;
				g += g_pixel as f32 * g_kernel;
				b += b_pixel as f32 * b_kernel;
			}
		}

		Rgb([r as u8,g as u8, b as u8])
	}
}
