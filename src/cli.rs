use clap::Parser;
use std::path::PathBuf;
use pdfium_render::render_config::PdfRenderConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
	/// Path to a file or directory with pdf files.
	#[arg(short, long, value_name = "PATH")]
	input: PathBuf,

	/// Path to the directory where the images will be saved.
	#[arg(short, long, value_name = "PATH")]
	output: PathBuf,

	/// The width that the images will have.
	#[arg(long, value_name = "WIDTH")]
	target_width: Option<i32>,

	/// The height that the images will have.
	#[arg(long, value_name = "HEIGHT")]
	target_height: Option<i32>,
	
	/// Password for the pdf file, if the pdf is protected.
	#[arg(long, value_name = "PASSWORD")]
	password: Option<String>
}

impl Config {
	pub fn new() -> Self {
		Config::parse()
	}

	pub fn input(&self) -> &PathBuf {
		&self.input
	}

	pub fn output(&self) -> &PathBuf {
		&self.output
	}
	
	pub fn password(&self) -> Option<&str> {
		self.password.as_deref()
	}

	pub fn render_config(&self) -> Result<PdfRenderConfig, anyhow::Error> {
		let width = self.target_width;
		let height = self.target_height;
		let render = Some(PdfRenderConfig::new());

		let render = if let Some(width) = width {
			Some(PdfRenderConfig::new().set_target_width(width))
		} else {
			render
		};

		let render = if let Some(height) = height {
			Some(PdfRenderConfig::new().set_target_width(height))
		} else {
			render
		};

		let render = if let (Some(width), Some(height)) = (width, height) {
			Some(PdfRenderConfig::new().set_target_size(width, height))
		} else {
			render
		};

		let render = match render {
			Some(render) => render,
			None => PdfRenderConfig::new(),
		};

		Ok(render)
	}
}
