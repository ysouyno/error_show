use std::env;
use winapi::um::winbase;
use std::ptr;
use winapi::um::winnt;

pub unsafe fn pwstr_to_string(ptr: winnt::PWSTR) -> String {
    use std::slice::from_raw_parts;
    let len = (0_usize..)
        .find(|&n| *ptr.offset(n as isize) == 0)
        .expect("Couldn't find null terminator");
    let array: &[u16] = from_raw_parts(ptr, len);
    String::from_utf16_lossy(array)
}

fn format_message(err_num: i32) -> String {
    const MAX_CHARACTERS: u16 = 1024;
    let mut err_msg = [winnt::WCHAR::default(); MAX_CHARACTERS as _];
    let ret = unsafe {
        winbase::FormatMessageW(
            // winbase::FORMAT_MESSAGE_ALLOCATE_BUFFER |
            winbase::FORMAT_MESSAGE_FROM_SYSTEM |
            winbase::FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null_mut(),
            err_num as u32,
            winnt::MAKELANGID(winnt::LANG_NEUTRAL,
                              winnt::SUBLANG_DEFAULT) as u32,
            err_msg.as_mut_ptr(),
            MAX_CHARACTERS.into(),
            ptr::null_mut()
        )
    };

    println!("ret: {}", ret);
    let ret = unsafe {
        pwstr_to_string(err_msg.as_mut_ptr())
    };

    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} {}", args[0], "err_num");
    }

    let err = args[1].parse::<i32>().unwrap();

    println!("{}: {}", err, format_message(err));
}
