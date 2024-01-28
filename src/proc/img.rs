use image::DynamicImage;
use image::ImageFormat;

use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use rayon::prelude::*;

fn single(
	index: usize,
	path: &Path,
	image: &DynamicImage,
	format: ImageFormat
) -> Result<(), image::ImageError> {
	let formatstr = format.extensions_str()[0];
	let name = format!("{index}.{formatstr}");
	let path = path.join(name);

	image.save_with_format(path, format)
}

pub fn write<P: AsRef<Path>>(
	images: Vec<DynamicImage>,
	format: ImageFormat,
	path: P
) -> Result<(), anyhow::Error> {
	let in_error = AtomicBool::new(false);
	let path = path.as_ref();
	fs::create_dir_all(&path)?;

	let style = ProgressStyle::with_template(
		" [{bar:40}] {pos}/{len} | Writing images... {eta} "
	)?.progress_chars("#•·");

	let progress = ProgressBar::new(images.len() as u64).with_style(style);
	let result: Vec<Result<(), image::ImageError>> = images
		.par_iter()
		.enumerate()
		.map(|(index, image)| {
			if !in_error.load(Ordering::Relaxed) {
				let result = single(index, path, image, format);

				if let Err(err) = result {
					in_error.store(true, Ordering::Relaxed);
					return Err(err);
				}

				progress.inc(1);
			}

			Ok(())
		})
		.collect();

	for result in result {
		result?;
	}
	
	Ok(())
}
