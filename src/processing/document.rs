use crate::config::Target;
use super::image::Images;

use pdfium_render::prelude::*;
use indicatif::ProgressBar;
use anyhow::anyhow;

const DEFAULT_DPI: (f32, f32) = (72.0, 72.0);

fn contains_single_image(page: &PdfPage) -> bool {
	let page_objs = page.objects();
	let mut img_obj_entries = 0;

	for obj in page_objs.iter() {
		if let Some(_) = obj.as_image_object() {
			img_obj_entries += 1;
		}
	}

	if img_obj_entries == 1 {
		true
	} else {
		false
	}
}

fn first_image_entry_dpi(page: &PdfPage) -> Result<(f32, f32), anyhow::Error> {
	let page_objs = page.objects();
	let mut dpi = None;

	for obj in page_objs.iter() {
		match obj.as_image_object() {
			Some(imgobj) => {
				let horizontal = imgobj.horizontal_dpi()?;
				let vertical = imgobj.vertical_dpi()?;

				dpi = Some((horizontal, vertical));
				break;
			}
			None => (),
		}
	}

	match dpi {
		Some(dpi) => Ok(dpi),
		None => Err(anyhow!("Image is not found.")),
	}
}

pub fn get_page_size(page: &PdfPage, target: &Target) -> (i32, i32) {
	let width = target.width();
	let height = target.height();
	let original_image_size = target.original_image_size();

	let dpi = match original_image_size {
		true =>
			match contains_single_image(page) {
				true => first_image_entry_dpi(page).unwrap(),
				false => DEFAULT_DPI,
			}

		false => DEFAULT_DPI,
	};

	let pth_to_pix = |pth: PdfPoints, dpi: f32| { (pth.to_inches() * dpi) as i32 };
	let std_width = pth_to_pix(page.width(), dpi.0);
	let std_height = pth_to_pix(page.height(), dpi.1);

	let val_by_mul = |val: i32, mul: (i32, i32)| -> i32 {
		(((mul.0 as f32) / (mul.1 as f32)) * (val as f32)) as i32
	};

	if width.is_some() && height.is_some() {
		let width = width.unwrap();
		let height = height.unwrap();

		return (width, height);
	}

	if width.is_some() && height.is_none() {
		let width = width.unwrap();
		let height = val_by_mul(std_height, (width, std_width));

		return (width, height);
	}

	if width.is_none() && height.is_some() {
		let height = height.unwrap();
		let width = val_by_mul(std_width, (height, std_height));

		return (width, height);
	}

	(std_width, std_height)
}

pub fn render_pages_by_range(
	bar: &ProgressBar,
	pages: &PdfPages,
	range: [u16; 2],
	target: &Target
) -> Result<Images, anyhow::Error> {
	// -----
	let mut images = Images::new();

	for index in range[0]..range[1] {
		bar.set_message(format!("Rendering::({})", index - range[0] + 1));

		let page = pages.get(index)?;
		let (width, height) = get_page_size(&page, target);

		let image = page.render(width, height, None)?.as_image();
		images.push(image);

		bar.inc(1);
	}

	Ok(images)
}
