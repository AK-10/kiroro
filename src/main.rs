use std::io::{stdin, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    // change rawmode
    // TODO: describe canonical mode and raw mode
    // dropped stdout, restore original state
    // into_raw_mode invoke libc::cfmakeraw()
    // cfmakeraw() set like `version7` driver's row mode
    //
    // specifically, set flags below
    // termios_p->c_iflag &= ~(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON)
    // termios_p->c_oflag &= ~OPOST;
    // termios_p->c_lflag &= ~(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    // termios_p->c_cflag &= ~(CSIZE | PARENB);
    // termios_p->c_cflag |= CS8;
    //
    // into_raw_mode: https://github.com/redox-os/termion/blob/dce5e7500fd709987f9bf8f3911e4daa61d0ad14/src/raw.rs#L101-L114
    // raw_terminal_attr: https://github.com/redox-os/termion/blob/8054e082b01c3f45f89f0db96bc374f1e378deb1/src/sys/unix/attr.rs#L17-L19
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in stdin().bytes() {
        match b {
            Ok(c) => {
                // \r is carriage return(CR)
                // CR control character moves cursor to beginning of line
                if c.is_ascii_control() {
                    println!("{}\r", c);
                } else {
                    println!("{} ('{}') \r", c, c as char)
                }

                if c == b'q' {
                    break;
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
