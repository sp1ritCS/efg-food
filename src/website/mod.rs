mod mem_file;

use anyhow::Result;
use isahc::{ReadResponseExt, config::Configurable};
use kuchiki::traits::TendrilSink;

use std::ops::{Deref, DerefMut};

pub struct WebsiteParser<T>(isahc::Response<T>);
const SELECTOR: &'static str = "a > .icon-food";
impl WebsiteParser<isahc::Body> {
	pub fn new() -> Result<Self> {
		let request = isahc::Request::get("https://firstwald.de")
			.redirect_policy(isahc::config::RedirectPolicy::Follow)
			.body(())?;
		let response = isahc::send(request)?;
		Ok(Self (response))
	}
	pub fn get_foodtable_link(&mut self) -> Result<String> {
		let parser = kuchiki::parse_html().from_utf8().read_from(self.deref_mut())?;

		let el = parser.select_first(SELECTOR).map_err(|_| super::errors::HtmlError::SelectorNotFound(SELECTOR))?;
		let link = el.as_node().parent().ok_or(super::errors::HtmlError::NoParent)?;
		let attrs = &link.as_element().unwrap().attributes;
		let mut link = attrs.borrow().get("href").unwrap().to_string();

		if !link.starts_with("http") {
			link.insert_str(0, "https://firstwald.de/");
		}

		Ok(link)
	}
}
impl <T> Deref for WebsiteParser<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0.body()
	}
}
impl <T> DerefMut for WebsiteParser<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.0.body_mut()
	}
}

pub fn download_into_memfd() -> Result<mem_file::TmpMemFile> {
	let memfile = unsafe { mem_file::TmpMemFile::new("current_foodtable") }?;

	let mut parser = WebsiteParser::new()?;
	let url = parser.get_foodtable_link()?;

	let request = isahc::Request::get(&url)
		.redirect_policy(isahc::config::RedirectPolicy::Follow)
		.body(())?;
	let mut response = isahc::send(request)?;
	response.copy_to(memfile.deref())?;

	Ok(memfile)
}