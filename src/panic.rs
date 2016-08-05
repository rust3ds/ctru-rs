use core::fmt::{Arguments, Write};

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: Arguments, file: &str, line: u32) -> ! {
    use gfx::Screen;
    use console::Console;

    let mut error_top = Console::init(Screen::Top);
    let mut error_bottom = Console::init(Screen::Bottom);

    writeln!(error_top, "--------------------------------------------------").unwrap();
    writeln!(error_top, "PANIC in {} at line {}:", file, line).unwrap();
    writeln!(error_top, "    {}", fmt).unwrap();
    write!(error_top, "\x1b[29;00H--------------------------------------------------").unwrap();

    writeln!(error_bottom, "").unwrap();     

    loop {}
}
