mod website;
pub mod parser;
pub mod errors;

use anyhow::Result;
use tabula::TabulaVM;
use serde::Serialize;

use std::io::Write;

#[derive(Debug, Default, Serialize)]
pub struct FoodRow {
	pub menu: String,
	pub vegetarian: String,
	pub dessert: String,
	pub dinner: String
}

#[derive(Debug, Serialize)]
pub struct FoodWeek {
	pub monday: FoodRow,
	pub tuesday: FoodRow,
	pub wednesday: FoodRow,
	pub thursday: FoodRow,
	pub friday: FoodRow
}

impl FoodRow {
	pub fn trim(&mut self) {
		self.menu = self.menu.trim().to_string();
		self.vegetarian = self.vegetarian.trim().to_string();
		self.dessert = self.dessert.trim().to_string();
		self.dinner = self.dinner.trim().to_string();
	}
}

fn main() -> Result<()> {
	let mut args = std::env::args();
	args.next().unwrap();

	let current = website::download_into_memfd()?;
	
	let jvm = TabulaVM::new(&args.next().expect("missing first param (path to jar)"), false)?;
	let env = jvm.attach()?;
	let tabula = env.configure_tabula(None, Some(&vec![1]), tabula::OutputFormat::Csv, true, tabula::ExtractionMethod::Basic, true, None)?;

	let file = tabula.parse_document(&unsafe {current.get_path()}, "parsed_foodtbl")?;

	let mut parser = parser::Parser::new_from_file(file);
	let week = parser.parse_week()?;

	let mut stdout = std::io::stdout();
	serde_json::to_writer(&stdout, &week)?;
	stdout.write(b"\n")?;

	// make sure current is only dropped after it has been used
	drop(current);

	Ok(())
}
