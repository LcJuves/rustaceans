use std::ffi::CStr;
use std::ffi::CString;

use libc::strlen;

fn main() {
    let rs = "hello world";
    let cs = CString::new(rs).unwrap();

    unsafe {
        let len = strlen(cs.as_ptr());
        println!("len: {}", len);
    }

    unsafe {
        let nrs = CStr::from_ptr(cs.as_ptr());
        println!("new rust string: {}", nrs.to_string_lossy());
    }
}
