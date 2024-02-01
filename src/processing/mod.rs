use crate::config::Config;

use anyhow::anyhow;
use pdfium_render::prelude::*;

use std::path::PathBuf;
use std::path::Path;
use std::fs;

use indicatif::MultiProgress;
use indicatif::ProgressBar;

mod document;
mod image;

fn dir_with_documents(
	mul_bar: &MultiProgress,
	pdfium: &Pdfium,
	config: &Config,
	input: &Path,
	output: &Path
) -> Result<(), anyhow::Error> {
	// -----
	let progress = config.progress();

	let entries = fs::read_dir(input)?;
	let mut paths = Vec::new();

	for result in entries {
		let entry = result?;
		let path = entry.path();

		let is_pdf = match infer::get_from_path(&path)? {
			Some(kind) => kind.extension() == "pdf",
			None => false,
		};

		if is_pdf {
			paths.push(path);
		}
	}

	let to_mul = ProgressBar::new(paths.len() as u64).with_style(progress.style()?);
	let bar = mul_bar.add(to_mul);

	for path in paths {
		bar.tick();
		bar.set_message(format!("Current document: {:?}", path.file_stem().unwrap()));

		single_document(mul_bar, pdfium, config, &path, output)?;

		bar.inc(1);
	}

	Ok(())
}

fn single_document(
	mul_bar: &MultiProgress,
	pdfium: &Pdfium,
	config: &Config,
	input: &Path,
	output: &Path
) -> Result<(), anyhow::Error> {
	let output_with_stem = || -> PathBuf { output.join(input.file_stem().unwrap()) };
	let output = output_with_stem();

	let number_of_pages_in_memory = config.number_of_pages_in_memory();
	let progress = config.progress();
	let target = config.target();

	let document = pdfium.load_pdf_from_file(input, None)?;
	let pages = document.pages();

	let mut current_page = 0;
	let pages_count = pages.len();

	let to_mul = ProgressBar::new(pages_count as u64).with_style(progress.nested_style()?);
	let bar = mul_bar.add(to_mul);

	while current_page < pages_count {
		let mut temp_current = current_page + number_of_pages_in_memory;

		if temp_current > pages_count {
			temp_current = pages_count;
		}

		let range = [current_page, temp_current];
		let mut images = document::render_pages_by_range(&bar, pages, range, target)?;

		image::apply_filters_multiply(&mut images, target);
		image::write_multiply(&bar, &images, &output, range, target)?;

		current_page = temp_current;
	}

	Ok(())
}

pub fn document(pdfium: &Pdfium, config: &Config) -> Result<(), anyhow::Error> {
	// -----
	let input = config.io().input();
	let output = config.io().output();
	let mul_bar = MultiProgress::new();

	match input {
		path if path.is_dir() => dir_with_documents(&mul_bar, pdfium, config, input, output),
		path if path.is_file() => single_document(&mul_bar, pdfium, config, input, output),
		_ => Err(anyhow!("invalid path.")),
	}
}
