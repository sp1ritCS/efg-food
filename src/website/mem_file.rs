use anyhow::Result;
use std::ops::Deref;
use std::path::PathBuf;
use std::fs::File;
use std::os::unix::io::FromRawFd;

pub struct TmpMemFile {
	fd: isize,
	file: File
}

impl TmpMemFile {
	pub unsafe fn new<T: AsRef<str>>(name: T) -> Result<Self> {
		let name = std::ffi::CString::new(name.as_ref())?;
		let fd = libc::syscall(libc::SYS_memfd_create, name.as_ptr(), 0) as isize;
		
		let file = File::from_raw_fd(fd as i32);

		Ok(Self {
			fd,
			file
		})
	}

	pub unsafe fn get_path(&self) -> PathBuf {
		let mut path = PathBuf::new();
		path.push("/proc");
		path.push(libc::getpid().to_string());
		path.push("fd");
		path.push(self.fd.to_string());
		path
	}
}

impl Deref for TmpMemFile {
	type Target = File;

	fn deref(&self) -> &Self::Target {
		&self.file
	}
}