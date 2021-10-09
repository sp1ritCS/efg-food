use thiserror::Error;
use super::parser;

#[derive(Error, Debug)]
pub enum HtmlError {
	#[error("selector {0} did not match anything")]
	SelectorNotFound(&'static str),
	#[error("selector matched as top level element")]
	NoParent,
}

#[derive(Error, Debug)]
pub enum ParserError {
	#[error("empty row at index {index}: {record:?}")]
	EmptyRow {
		index: u64,
		record: csv::StringRecord
	},
	#[error("failed populating all fields of week: {0:?}")]
	FailedPopulateFields(parser::IntParsedFoodWeek),
	#[error("missing a weekday at index {index}: {wk:?}")]
	MissingWeekday {
		index: i32,
		wk: parser::IntFoodWeek
	}
}