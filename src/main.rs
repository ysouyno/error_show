use std::env;

#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef;
#[cfg(target_os = "windows")]
use winapi::um::{winbase, winnt};

#[cfg(target_os = "linux")]
extern crate libc;

#[cfg(target_os = "linux")]
// from https://stackoverflow.com/questions/40710115/how-does-one-get-the-error-message-as-provided-by-the-system-without-the-os-err
pub fn error_string(errno: i32) -> String {
    use std::ffi::CStr;
    use std::os::raw::c_char;
    use std::os::raw::c_int;
    use std::str;

    const TMPBUF_SZ: usize = 128;
    extern "C" {
        #[cfg_attr(any(target_os = "linux"), link_name = "__xpg_strerror_r")]
        fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: libc::size_t) -> c_int;
    }

    let mut buf = [0 as c_char; TMPBUF_SZ];
    let p = buf.as_mut_ptr();

    unsafe {
        if strerror_r(errno as c_int, p, buf.len()) < 0 {
            panic!("strerror_r failed");
        }

        let p = p as *const _;
        str::from_utf8(CStr::from_ptr(p).to_bytes())
            .unwrap()
            .to_owned()
    }
}

#[cfg(target_os = "windows")]
pub unsafe fn pwstr_to_string(ptr: winnt::PWSTR) -> String {
    use std::slice::from_raw_parts;
    let len = (0_usize..)
        .find(|&n| *ptr.offset(n as isize) == 0)
        .expect("Couldn't find null terminator");
    let array: &[u16] = from_raw_parts(ptr, len);
    String::from_utf16_lossy(array)
}

#[cfg(target_os = "windows")]
fn error_string(err_num: i32) -> String {
    let mut err_msg: winnt::LPWSTR = ptr::null_mut();
    let ret = unsafe {
        winbase::FormatMessageW(
            winbase::FORMAT_MESSAGE_ALLOCATE_BUFFER
                | winbase::FORMAT_MESSAGE_FROM_SYSTEM
                | winbase::FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null_mut(),
            err_num as u32,
            winnt::MAKELANGID(winnt::LANG_NEUTRAL, winnt::SUBLANG_DEFAULT) as u32,
            (&mut err_msg as *mut winnt::LPWSTR) as winnt::LPWSTR,
            0,
            ptr::null_mut(),
        )
    };

    if ret == 0 {
        String::from("Unknown")
    } else {
        let ret = unsafe { pwstr_to_string(err_msg) };

        unsafe {
            winbase::LocalFree(err_msg as minwindef::HLOCAL);
        }

        ret
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} {}", args[0], "err_num");
    }

    let err = args[1].parse::<i32>().unwrap();

    println!("Error({}): {}", err, error_string(err));
}
