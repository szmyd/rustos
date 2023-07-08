// No Standard Library and no c-runtime
#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    draw_bootscreen();
    loop {}
}

pub fn draw_bootscreen() {
    use vga_buffer::Color;
    let mut writer = vga_buffer::WRITER.lock();

    writer.clear_screen();
    writer.set_bg_color(Color::DarkGrey);
    writer.write_string("rustOS [v0.0.1]                                                                ");
    writer.set_bg_color(Color::Black);
    let mut line_num = 1;
    while line_num < vga_buffer::Writer::get_height() {
        writer.write_string("\n");
        line_num += 1;
    }
    writer.set_bg_color(Color::DarkGrey);
    writer.set_fg_color(Color::Yellow);
    writer.write_string("                            Copyright (c) 2023 Brian Szmyd, All rights reserved.");
}
