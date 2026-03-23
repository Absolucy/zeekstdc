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
	static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

fn set_last_error(msg: impl Into<String>) {
	let msg = msg.into();
	let msg = match CString::new(msg.as_bytes()) {
		Ok(msg) => msg,
		Err(err) => CString::new(&msg.as_bytes()[..err.nul_position()]).unwrap_or_default(),
	};
	LAST_ERROR.with_borrow_mut(|last_error| *last_error = Some(msg));
}

#[unsafe(no_mangle)]
pub extern "C" fn zs_open_file(file_name: *const c_char, compression_level: i32) -> *mut c_void {
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
	LAST_ERROR.with_borrow(|last_error| {
		last_error
			.as_deref()
			.map(|e| e.as_ptr())
			.unwrap_or(std::ptr::null())
	})
}
