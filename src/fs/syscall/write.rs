use crate::{mm::user::verify::verify_buffer, process::task::CURRENT, syscall::errno::Errno};

use super::read::get_file;

pub fn sys_write(fd: isize, buf: usize, len: usize) -> Result<usize, Errno> {
	if len == 0 {
		return Ok(0);
	}

	let current = unsafe { CURRENT.get_ref() };

	let buf = verify_buffer(buf, len, current)?;

	get_file(fd)?.write(buf)
}
