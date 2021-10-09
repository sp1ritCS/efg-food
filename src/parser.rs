use super::{FoodRow, FoodWeek, errors::ParserError};

use anyhow::Result;
use csv::{Position, Reader};

use std::{io::{Read, Seek}, ops::{Deref, DerefMut}};

#[derive(Debug, Default, Clone)]
pub struct IntFoodWeek {
	pub monday: Option<Position>,
	pub tuesday: Option<Position>,
	pub wednesday: Option<Position>,
	pub thursday: Option<Position>,
	pub friday: Option<Position>
}

#[derive(Debug, Default)]
pub struct IntParsedFoodWeek {
	pub monday: Option<FoodRow>,
	pub tuesday: Option<FoodRow>,
	pub wednesday: Option<FoodRow>,
	pub thursday: Option<FoodRow>,
	pub friday: Option<FoodRow>
}
impl IntParsedFoodWeek {
	pub fn try_unwrap(&mut self) -> Option<FoodWeek> {
		Some(FoodWeek {
			monday: self.monday.take()?,
			tuesday: self.tuesday.take()?,
			wednesday: self.wednesday.take()?,
			thursday: self.thursday.take()?,
			friday: self.friday.take()?
		})
	}
	pub fn unwrap(&mut self) -> FoodWeek {
		self.try_unwrap().unwrap()
	}
}

fn insert_irow(field: &mut String, new: Option<&str>) {
	if let Some(new) = new {
		field.push_str(new);
		field.push('\n');
	}
}

pub struct Parser<R> {
	rdr: Reader<R>
}
impl <R: Read + Seek> Parser<R> {
	pub fn new_from_file(reader: R) -> Self {
		Self {
			rdr: Reader::from_reader(reader)
		}
	}

	fn get_positions(&mut self) -> Result<IntFoodWeek> {
		let mut wk = IntFoodWeek::default();
		let mut records = self.records();
		loop {
			let pos = records.reader().position().clone();
			if let Some(result) = records.next() {
				let row = result?;
				let first_field = row.get(0).ok_or(ParserError::EmptyRow {index: row.position().unwrap().line(), record: row.clone()})?;
				match first_field {
					"Montag" =>		wk.monday		= Some(pos),
					"Dienstag" =>	wk.tuesday		= Some(pos),
					"Mittwoch" =>	wk.wednesday	= Some(pos),
					"Donnerstag" =>	wk.thursday		= Some(pos),
					"Freitag" =>	wk.friday		= Some(pos),
					_ => ()
				}
			} else {
				break;
			}
		}
		Ok(wk)
	}

	fn int_parse_week(&mut self, positions: IntFoodWeek) -> Result<IntParsedFoodWeek> {
		let mut week = IntParsedFoodWeek::default();
		let mut records = self.records();
		for i in 1..=5 {
			let pos = match i {
				1 => positions.monday.as_ref(),
				2 => positions.tuesday.as_ref(),
				3 => positions.wednesday.as_ref(),
				4 => positions.thursday.as_ref(),
				5 => positions.friday.as_ref(),
				_ => panic!("out of bounds")
			}.ok_or(ParserError::MissingWeekday { index: i, wk: positions.clone() })?;

			records.reader_mut().seek(pos.clone())?;
			let mut record = records.reader_mut().records();
			
			let mut food = FoodRow::default();
			let mut daterow = 2;
			loop {
				if let Some(result) = record.next() {
					let row = result?;
					if row.get(0) == Some("") || daterow > 0 {
						daterow -= 1;
						insert_irow(&mut food.menu, row.get(1));
						insert_irow(&mut food.vegetarian, row.get(2));
						insert_irow(&mut food.dessert, row.get(3));
						insert_irow(&mut food.dinner, row.get(4));
					} else {
						break;
					}
				} else {
					break;
				}
			}

			food.trim();

			match i {
				1 => week.monday	= Some(food),
				2 => week.tuesday	= Some(food),
				3 => week.wednesday	= Some(food),
				4 => week.thursday	= Some(food),
				5 => week.friday	= Some(food),
				_ => panic!("out of bounds")
			}
		}
		Ok(week)
	}

	pub fn parse_week(&mut self) -> Result<FoodWeek> {
		let positions = self.get_positions()?;
		let mut internal = self.int_parse_week(positions)?;

		internal.try_unwrap().ok_or(super::errors::ParserError::FailedPopulateFields(internal).into())
	}
}

impl <R> Deref for Parser<R> {
	type Target = Reader<R>;

	fn deref(&self) -> &Self::Target {
		&self.rdr
	}
}
impl <R> DerefMut for Parser<R> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.rdr
	}
}