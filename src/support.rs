use core::fmt;

#[lang="eh_personality"]
extern fn eh_personality() {}

#[no_mangle]
pub extern fn abort() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, num: isize) {
    for i in 1..num {
        unsafe {
            *dest.offset(i) = *src.offset(i)
        }
    }
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_msg: fmt::Arguments,
                                _file: &'static str,
                                _line: u32) -> ! {
    loop {}
}
