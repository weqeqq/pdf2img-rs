use clap::Parser;
use clap::Args;
use clap::ValueEnum;

use std::path::Path;
use std::path::PathBuf;

use image::ImageFormat;
use indicatif::ProgressStyle;
use indicatif::style::TemplateError;

const DEFAULT_PROGRESS_TEMPLATE: &'static str = " |{bar:40.magenta}| {msg} ";
const DEFAULT_PROGRESS_CHARS: &'static str = "@•.";

const DEFAULT_NESTED_PROGRESS_TEMPLATE: &'static str = " ({bar:40.cyan}) {eta} | {msg} ";
const DEFAULT_NESTED_PROGRESS_CHARS: &'static str = "•·.";

const DEFAULT_IMAGE_FORMAT: PdfImageFormat = PdfImageFormat::Png;
const DEFAULT_PAGES_NUM_IN_MEMORY: u16 = 25;

#[derive(Args)]
pub struct IO {
	#[arg(short, long)]
	input: PathBuf,

	#[arg(short, long)]
	output: PathBuf,
}

impl IO {
	pub fn input(&self) -> &Path {
		&self.input
	}

	pub fn output(&self) -> &Path {
		&self.output
	}
}

#[derive(ValueEnum, Clone, Copy)]
enum PdfImageFormat {
	Png,
	Jpeg,
	Webp,
}

#[derive(Args)]
pub struct Target {
	#[arg(long = "target-width", value_name = "WIDTH")]
	width: Option<i32>,

	#[arg(long = "target-height", value_name = "HEIGHT")]
	height: Option<i32>,

	#[arg(long = "image-format", value_enum, default_value_t = DEFAULT_IMAGE_FORMAT)]
	format: PdfImageFormat,

	#[arg(long)]
	original_image_size: bool,

	#[arg(long)]
	normalize_image: bool,

	#[arg(long)]
	threshold_image: bool,
}

impl Target {
	pub fn width(&self) -> Option<i32> {
		self.width
	}

	pub fn height(&self) -> Option<i32> {
		self.height
	}

	pub fn image_format(&self) -> ImageFormat {
		match self.format {
			PdfImageFormat::Png => ImageFormat::Png,
			PdfImageFormat::Jpeg => ImageFormat::Jpeg,
			PdfImageFormat::Webp => ImageFormat::WebP,
		}
	}

	pub fn original_image_size(&self) -> bool {
		self.original_image_size
	}

	pub fn normalize_image(&self) -> bool {
		self.normalize_image
	}

	pub fn threshold_image(&self) -> bool {
		self.threshold_image
	}
}

#[derive(Args)]
pub struct Progress {
	#[arg(
		long = "progress-bar-template",
		value_name = "TEMPLATE",
		default_value_t = String::from(DEFAULT_PROGRESS_TEMPLATE)
	)]
	template: String,

	#[arg(
		long = "nested-progress-bar-template",
		value_name = "TEMPLATE",
		default_value_t = String::from(DEFAULT_NESTED_PROGRESS_TEMPLATE)
	)]
	nested_template: String,

	#[arg(
		long = "progress-bar-chars",
		value_name = "CHARS",
		default_value_t = String::from(DEFAULT_PROGRESS_CHARS)
	)]
	chars: String,

	#[arg(
		long = "nested-progress-bar-chars",
		value_name = "CHARS",
		default_value_t = String::from(DEFAULT_NESTED_PROGRESS_CHARS)
	)]
	nested_chars: String,
}

impl Progress {
	pub fn style(&self) -> Result<ProgressStyle, TemplateError> {
		ProgressStyle::with_template(&self.template).map(|style|
			// ----
			style.progress_chars(&self.chars)
		)
	}

	pub fn nested_style(&self) -> Result<ProgressStyle, TemplateError> {
		ProgressStyle::with_template(&self.nested_template).map(|style|
			// ----
			style.progress_chars(&self.nested_chars)
		)
	}
}

#[derive(Parser)]
pub struct Config {
	#[command(flatten)]
	io: IO,

	#[command(flatten)]
	target: Target,

	#[command(flatten)]
	progress: Progress,

	#[arg(long, value_name = "INTEGER", default_value_t = DEFAULT_PAGES_NUM_IN_MEMORY)]
	number_of_pages_in_memory: u16,
}

impl Config {
	pub fn new() -> Self {
		Self::parse()
	}

	pub fn io(&self) -> &IO {
		&self.io
	}

	pub fn target(&self) -> &Target {
		&self.target
	}
	pub fn progress(&self) -> &Progress {
		&self.progress
	}

	pub fn number_of_pages_in_memory(&self) -> u16 {
		self.number_of_pages_in_memory
	}
}
