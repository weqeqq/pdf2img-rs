use pdfium_render::prelude::*;
use std::path::Path;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

pub fn render_pages<P: AsRef<Path>>(
	pdfium: &Pdfium,
	config: &PdfRenderConfig,
	pdf_path: P,
	password: Option<&str>
) -> Result<Vec<image::DynamicImage>, anyhow::Error> {
	let document = pdfium.load_pdf_from_file(pdf_path.as_ref(), password)?;
	let pages = document.pages();

	let style = ProgressStyle::with_template(
		" [{bar:40}] {pos}/{len} | Rendering pages... {eta} "
	)?.progress_chars("#•·");

	let progress = ProgressBar::new(pages.len() as u64).with_style(style);
	let mut images = Vec::new();

	for page in pages.iter() {
		images.push(page.render_with_config(&config)?.as_image());

		progress.inc(1);
	}

	Ok(images)
}
