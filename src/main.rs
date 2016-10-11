#![feature(lang_items, core_intrinsics)]

#![no_std]
#![no_main]

use core::intrinsics::volatile_store;

pub mod support;

pub fn write32(addr: u32, val: u32) {
    let p = addr as *mut u32;

    unsafe {
        volatile_store(p, val);
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let lcd_base = 0x0d000000;

    let msg: [u32; 32] = [
        0,
        0,
        0,
        0,
        0b1000001000000000000000000000000,
        0b1000001010000000000000000000000,
        0b1000001010000000000000000000000,
        0b1111111010000000000000000000000,
        0b1000001010000000000000000000000,
        0b1000001010000000000000000000000,
        0b1000001010000000000000000000000,
        0b0000000000000000000000000000000,
        0b0000000000000000000000000000000,
        0b1111110111110001111001000010000,
        0b1000000100001010000101100110000,
        0b1111100100001010000101011010000,
        0b1000000111110010000101000010000,
        0b1000000100010010000101000010000,
        0b1000000100001001111001000010000,
        0b0000000000000000000000000000000,
        0b0000000000000000000000000000000,
        0b1111110000000000000000000000111,
        0b1000001010000100111100111110111,
        0b1000001010000101000000001000111,
        0b1111110010000100111100001000010,
        0b1000100010000100000010001000000,
        0b1000010010000101000010001000111,
        0b1000001001111000111100001000111,
        0,
        0,
        0,
        0,
    ];

    for (i, &line) in msg.iter().enumerate() {
        // On the LCD the MSB in on the right so we need to mirror
        // each line before we write the value
        let mut rev = 0;

        let mut line = line;

        for _ in 0..31 {
            rev |= line & 1;
            rev <<= 1;
            line >>= 1;
        }

        let i = i as u32;

        write32(lcd_base + 0x100 + i * 4,
                rev);
    }

    loop {}
}
