#![allow(dead_code)]
use pdfium_render::pdfium::Pdfium;
use console::Term;

mod processing;
mod config;

fn main() -> Result<(), anyhow::Error> {
	let pdfium = Pdfium::default();
	let config = config::Config::new();

	let terminal = Term::stdout();
	terminal.clear_screen()?;
	terminal.hide_cursor()?;

	processing::document(&pdfium, &config)?;

	Ok(())
}
