use std::i64;

#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef;
#[cfg(target_os = "windows")]
use winapi::um::{libloaderapi, winbase, winnt};

#[cfg(target_os = "linux")]
extern crate libc;

use structopt::StructOpt;

/// Show error code information
#[derive(StructOpt, Debug)]
#[structopt(global_settings(&[structopt::clap::AppSettings::AllowNegativeNumbers]))]
struct Opts {
    /// Decimal or hexadecimal error code
    errno: String,
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
fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

#[cfg(target_os = "windows")]
fn error_string_wininet(errno: i32) -> String {
    let mut err_msg: winnt::LPWSTR = ptr::null_mut();
    let hmodule = unsafe {
        libloaderapi::LoadLibraryExW(
            to_wstring("wininet.dll").as_ptr(),
            ptr::null_mut(),
            libloaderapi::DONT_RESOLVE_DLL_REFERENCES,
        )
    };

    if hmodule != ptr::null_mut() {
        let ret = unsafe {
            winbase::FormatMessageW(
                winbase::FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | winbase::FORMAT_MESSAGE_FROM_HMODULE
                    | winbase::FORMAT_MESSAGE_FROM_SYSTEM
                    | winbase::FORMAT_MESSAGE_IGNORE_INSERTS
                    | winbase::FORMAT_MESSAGE_MAX_WIDTH_MASK,
                hmodule as minwindef::LPCVOID,
                errno as u32,
                winnt::MAKELANGID(winnt::LANG_ENGLISH, winnt::SUBLANG_DEFAULT) as u32,
                (&mut err_msg as *mut winnt::LPWSTR) as winnt::LPWSTR,
                0,
                ptr::null_mut(),
            )
        };

        unsafe {
            libloaderapi::FreeLibrary(hmodule);
        }

        if ret == 0 {
            String::from("Unknown.")
        } else {
            let ret = unsafe { pwstr_to_string(err_msg) };

            unsafe {
                winbase::LocalFree(err_msg as minwindef::HLOCAL);
            }

            ret
        }
    } else {
        String::from("Unknown.")
    }
}

#[cfg(target_os = "windows")]
fn error_string(errno: i32) -> String {
    let mut err_msg: winnt::LPWSTR = ptr::null_mut();
    let ret = unsafe {
        winbase::FormatMessageW(
            winbase::FORMAT_MESSAGE_ALLOCATE_BUFFER
                | winbase::FORMAT_MESSAGE_FROM_SYSTEM
                | winbase::FORMAT_MESSAGE_IGNORE_INSERTS
                | winbase::FORMAT_MESSAGE_MAX_WIDTH_MASK,
            ptr::null_mut(),
            errno as u32,
            winnt::MAKELANGID(winnt::LANG_ENGLISH, winnt::SUBLANG_DEFAULT) as u32,
            (&mut err_msg as *mut winnt::LPWSTR) as winnt::LPWSTR,
            0,
            ptr::null_mut(),
        )
    };

    if ret == 0 {
        // Is it a network-related error?
        error_string_wininet(errno)
    } else {
        let ret = unsafe { pwstr_to_string(err_msg) };

        unsafe {
            winbase::LocalFree(err_msg as minwindef::HLOCAL);
        }

        ret
    }
}

#[cfg(target_os = "linux")]
pub fn error_string(errno: i32) -> String {
    use std::ffi::CStr;
    use std::os::raw::c_char;
    use std::os::raw::c_int;
    use std::str;

    extern "C" {
        #[cfg_attr(any(target_os = "linux"), link_name = "__xpg_strerror_r")]
        fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: libc::size_t) -> c_int;
    }

    let mut buf = [0 as c_char; 128];
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

fn main() {
    let opts = Opts::from_args();

    if opts.errno.starts_with("0x") {
        let errno = opts.errno.trim_start_matches("0x");
        let errno = i64::from_str_radix(errno, 16);
        match errno {
            Ok(errno) => println!("Error({}): {}", opts.errno, error_string(errno as i32)),
            _ => println!("Unknown."),
        }
    } else {
        let errno = opts.errno.parse::<i32>().unwrap();
        println!("Error({}): {}", opts.errno, error_string(errno));
    }
}
