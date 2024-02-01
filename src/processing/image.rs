use crate::config::Target;

use image::ImageFormat;
use rayon::prelude::*;

use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;

use std::fs;
use std::path::Path;
use indicatif::ProgressBar;
use std::sync::Mutex;

pub type Image = image::DynamicImage;
pub type Images = Vec<Image>;

pub fn write_multiply(
	bar: &ProgressBar,
	images: &Images,
	path: &Path,
	range: [u16; 2],
	target: &Target
) -> Result<(), anyhow::Error> {
	let in_error = AtomicBool::new(false);

	let set_error = || in_error.store(true, Ordering::Relaxed);
	let get_error = || -> bool { in_error.load(Ordering::Relaxed) };
	let handle_error = |result: &Result<(), anyhow::Error>| {
		if result.is_err() {
			set_error();
		}
	};

	let format = target.image_format();
	let image_name = |index: u16, format: ImageFormat| -> String {
		format!("{}.{}", index, format.extensions_str()[0])
	};

	let count = Mutex::new(0);
	let results: Vec<Result<(), anyhow::Error>> = images
		.par_iter()
		.zip(range[0]..range[1])
		.map(|(image, index)| {
			if !get_error() {
				let result = write(image, path, &image_name(index, format), format);
				handle_error(&result);

				result?;
			}

			let mut lock_count = count.lock().unwrap();

			*lock_count += 1;
			bar.set_message(format!("writing::({})", *lock_count));

			Ok(())
		})
		.collect();

	for result in results {
		result?;
	}

	Ok(())
}

pub fn write(
	image: &Image,
	path: &Path,
	name: &str,
	format: ImageFormat
) -> Result<(), anyhow::Error> {
	if !path.exists() {
		fs::create_dir_all(path)?;
	}

	let output = path.join(name);
	image.save_with_format(output, format)?;

	Ok(())
}

fn normalize(image: &Image) -> Image {
	let mut luma_image = image.to_luma8();

	let max = *luma_image.iter().max().unwrap();
	let min = *luma_image.iter().min().unwrap();

	for byte in luma_image.iter_mut() {
		let norm = (((*byte as f32) - (min as f32)) / ((max as f32) - (min as f32))) * 255.0;
		*byte = norm as u8;
	}

	Image::ImageLuma8(luma_image)
}

fn threshold(image: &Image) -> Image {
	let mut luma_image = image.to_luma8();

	for byte in luma_image.iter_mut() {
		*byte = if *byte < 255 / 2 { 0 } else { 255 };
	}

	Image::ImageLuma8(luma_image)
}

pub fn apply_filters(image: &mut Image, target: &Target) {
	let normalize_image = target.normalize_image();
	let threshold_image = target.threshold_image();

	if normalize_image {
		*image = normalize(image);
	}

	if threshold_image {
		*image = threshold(image);
	}
}

pub fn apply_filters_multiply(images: &mut Images, target: &Target) {
	for image in images {
		apply_filters(image, target);
	}
}
