#![allow(clippy::not_unsafe_ptr_arg_deref)]
use std::{
	cell::RefCell,
	ffi::{CStr, CString, c_char, c_void},
	fs::File,
	io::Write,
};
use zeekstd::{EncodeOptions, Encoder};

type OurEncoder = Encoder<'static, File>;

thread_local! {
	static LAST_ERROR: RefCell<CString> = RefCell::new(CString::default());
}

fn set_last_error(msg: impl Into<String>) {
	let msg = msg.into();
	let msg = match CString::new(msg.as_bytes()) {
		Ok(msg) => msg,
		Err(err) => CString::new(&msg.as_bytes()[..err.nul_position()]).unwrap_or_default(),
	};
	LAST_ERROR.set(msg)
}

#[unsafe(no_mangle)]
pub extern "C" fn zs_open_file(file_name: *const c_char, compression_level: i32) -> *mut c_void {
	#[cfg(feature = "extra-safety-checks")]
	if file_name.is_null() {
		set_last_error("no file name passed to zs_open_file");
		return std::ptr::null_mut();
	}
	let file_name = unsafe { CStr::from_ptr(file_name) }.to_string_lossy();
	let file = match File::create(file_name.as_ref()) {
		Ok(file) => file,
		Err(err) => {
			set_last_error(err.to_string());
			return std::ptr::null_mut();
		}
	};
	match Encoder::with_opts(
		file,
		EncodeOptions::new().compression_level(compression_level),
	)
	.map(Box::new)
	{
		Ok(encoder) => Box::into_raw(encoder) as *mut c_void,
		Err(err) => {
			set_last_error(err.to_string());
			std::ptr::null_mut()
		}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn zs_write(encoder: *mut c_void, data: *const u8, len: usize) -> bool {
	#[cfg(feature = "extra-safety-checks")]
	{
		if encoder.is_null() {
			set_last_error("null encoder passed to zs_write");
			return false;
		}
		if data.is_null() || len == 0 {
			// nothing to write, don't bother
			return true;
		}
	}
	let encoder = encoder as *mut OurEncoder;
	let bytes = unsafe { std::slice::from_raw_parts(data, len) };
	match unsafe { (*encoder).write_all(bytes) } {
		Ok(_) => true,
		Err(err) => {
			set_last_error(err.to_string());
			false
		}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn zs_flush(encoder: *mut c_void) -> bool {
	#[cfg(feature = "extra-safety-checks")]
	if encoder.is_null() {
		set_last_error("null encoder passed to zs_flush");
		return false;
	}
	let encoder = encoder as *mut OurEncoder;
	match unsafe { (*encoder).flush() } {
		Ok(_) => true,
		Err(err) => {
			set_last_error(err.to_string());
			false
		}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn zs_finish(encoder: *mut c_void) -> u64 {
	#[cfg(feature = "extra-safety-checks")]
	if encoder.is_null() {
		set_last_error("null encoder passed to zs_finish");
		return 0;
	}
	let encoder = encoder as *mut OurEncoder;
	let encoder = unsafe { Box::from_raw(encoder) };
	match encoder.finish() {
		Ok(written) => written,
		Err(err) => {
			set_last_error(err.to_string());
			0
		}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn zs_last_error() -> *const c_char {
	LAST_ERROR.with_borrow(|last_error| last_error.as_ptr())
}
