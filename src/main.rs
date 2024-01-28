#![allow(dead_code)]

use console::Term;

mod proc;
mod cli;

fn prepare_console() -> Result<(), anyhow::Error> {
	let term = Term::stdout();

	term.clear_screen()?;
	term.hide_cursor()?;

	Ok(())
}

fn main() -> Result<(), anyhow::Error> {
	prepare_console()?;
	proc::render_pdf()?;
	Ok(())
}
