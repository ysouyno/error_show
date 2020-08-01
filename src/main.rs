use std::env;
use std::ptr;
use winapi::shared::minwindef;
use winapi::um::{winbase, winnt};

pub unsafe fn pwstr_to_string(ptr: winnt::PWSTR) -> String {
    use std::slice::from_raw_parts;
    let len = (0_usize..)
        .find(|&n| *ptr.offset(n as isize) == 0)
        .expect("Couldn't find null terminator");
    let array: &[u16] = from_raw_parts(ptr, len);
    String::from_utf16_lossy(array)
}

fn format_message(err_num: i32) -> String {
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

    println!("Error({}): {}", err, format_message(err));
}
