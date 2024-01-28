use crate::cli::Config;

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use pdfium_render::pdfium::Pdfium;
use pdfium_render::render_config::PdfRenderConfig;

use indicatif::ProgressStyle;
use indicatif::ProgressBar;

use console::Term;

mod pdf;
mod img;

fn is_pdf<P: AsRef<Path>>(path: P) -> Result<bool, anyhow::Error> {
	let opt = infer::get_from_path(path)?;

	if let Some(kind) = opt {
		return Ok(kind.extension() == "pdf");
	}

	Ok(false)
}

fn pdf_from_dir<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, anyhow::Error> {
	let entries = fs::read_dir(path.as_ref())?;

	let mut file_paths = Vec::new();
	let mut pdf_paths = Vec::new();

	for entry in entries {
		file_paths.push(entry?.path());
	}

	for file_path in file_paths {
		is_pdf(&file_path)?.then(|| pdf_paths.push(file_path));
	}

	Ok(pdf_paths)
}

fn proc_pdf<P: AsRef<Path>>(
	pdfium: &Pdfium,
	render: &PdfRenderConfig,
	input: P,
	output: P,
	password: Option<&str>
) -> Result<(), anyhow::Error> {
	if !is_pdf(&input)? {
		return Err(anyhow::anyhow!("file is not pdf"));
	}

	let stem = input.as_ref().file_stem().ok_or(anyhow::anyhow!("file_stem error"))?;
	let output = output.as_ref().join(stem);

	let term = Term::stdout();
	let current = format!("{:?}", input.as_ref().file_name().unwrap_or_default());
	term.set_title(format!("Processing: {current}"));

	let images = pdf::render_pages(pdfium, render, input, password)?;
	img::write(images, image::ImageFormat::Png, output)?;

	term.clear_screen()?;
	Ok(())
}

fn mul_proc_pdf<P: AsRef<Path>>(
	pdfium: &Pdfium,
	render: &PdfRenderConfig,
	input: P,
	output: P,
	password: Option<&str>
) -> Result<(), anyhow::Error> {
	let paths = pdf_from_dir(input)?;

	let style = ProgressStyle::with_template(
		" <{bar:40.magenta}> {pos}/{len} | Processing: {msg} "
	)?.progress_chars("@•·");

	let progress = ProgressBar::new(paths.len() as u64).with_style(style);

	for path in paths {
		let current = format!("{:?}", path.file_name().unwrap_or_default());
		progress.set_message(current);

		proc_pdf(&pdfium, &render, path, output.as_ref().to_owned(), password)?;

		progress.inc(1);
	}

	progress.finish();

	Ok(())
}
pub fn render_pdf() -> Result<(), anyhow::Error> {
	let config = Config::new();
	let pdfium = Pdfium::default();

	let input = config.input();
	let output = config.output();
	let render = config.render_config()?;
	let password = config.password();

	match input {
		inp if inp.is_file() => proc_pdf(&pdfium, &render, input, output, password),
		inp if inp.is_dir() => mul_proc_pdf(&pdfium, &render, input, output, password),
		_ => Err(anyhow::anyhow!("how?..")),
	}
}
