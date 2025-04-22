use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// assumes libwapp.so is on your linker path
#[link(name = "goffi", kind = "dylib")]
unsafe extern "C" {
    fn wappalyzer(a: *const c_char, b: *const c_char) -> *mut c_char;
}

fn main() {
    // Prepare some inputs:
    let a = CString::new("input1").unwrap();
    let b = CString::new("input2").unwrap();

    // SAFELY call the foreign function:
    let raw: *mut c_char = unsafe { wappalyzer(a.as_ptr(), b.as_ptr()) };

    // Convert result back into a Rust String:
    let result = unsafe {
        assert!(!raw.is_null());
        let s = CStr::from_ptr(raw).to_string_lossy().into_owned();
        // free the C-allocated string to avoid leaks:
        libc::free(raw as *mut libc::c_void);
        s
    };

    println!("wappalyzer returned: {}", result);
}
