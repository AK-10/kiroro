use std::io::{stdin, stdout, Write};
use termion::cursor::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct EditorConfig {
    rows: u16,
    cols: u16,
}

impl EditorConfig {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self { rows, cols }
    }
}

// fn ctrl_key(key: u8) -> u8 {
//     // In ASCII character code, control characters that are not displayed as characters
//     // are assigned to 0~31(0000_0000 ~ 0001_1111)
//     key & 0b0001_1111
// }

fn reset_screen_on_end() {
    print!("\x1b[2j");
    print!("\x1b[H");
    stdout().flush().unwrap();
}

fn editor_draw_rows(rows: u16) {
    // draw terminal rows numbe
    // for now 24
    for _ in 0..rows {
        print!("~\r\n");
    }
    stdout().flush().unwrap();
}

fn refresh_screen(rows: u16) {
    // \x1b is escape character
    // this is write escape sequence to terminal
    // \x1b[2j
    // \x1b = ESC
    // [    = ANSI control sequence indtoducer
    // 2j   = erase entire screen
    print!("\x1b[2j");
    // moves cursor to home position (1, 1)
    print!("\x1b[H");

    editor_draw_rows(rows);

    print!("\x1b[H");
    stdout().flush().unwrap();
}

fn get_window_size() -> (u16, u16) {
    match termion::terminal_size() {
        Ok(ts) => ts,
        Err(_) => {
            // get termsize manually
            // move cursor bottom right
            // then, get cursor position
            // \x1b[nC (n: natural number) move cursor to right direction amount of n
            // \x1b[nB (n: natural number) move cursor to bottom direction amount of n
            // print!("\x1b[999C\x1b[999B");
            print!("{}", termion::cursor::Goto(999, 999));
            stdout().flush().unwrap();

            stdout().cursor_pos().unwrap()
        }
    }
}

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
    let stdin = stdin();

    let term_size = get_window_size();
    let editor_config = EditorConfig::new(term_size.0, term_size.1);

    refresh_screen(editor_config.rows);
    for k in stdin.keys() {
        match k {
            Ok(k) => {
                if k == termion::event::Key::Ctrl('q') {
                    reset_screen_on_end();
                    break
                }
                println!("{:?}\r", k);
            }
            Err(e) => {
                reset_screen_on_end();
                panic!("{}", e);
            }
        }
        refresh_screen(0);
    }
    // for b in stdin.bytes() {
    //     match b {
    //         Ok(c) => {
    //             // \r is carriage return(CR)
    //             // CR control character moves cursor to beginning of line
    //             if c.is_ascii_control() {
    //                 println!("{}\r", c);
    //             } else {
    //                 println!("{} ('{}') \r", c, c as char);
    //             }

    //             if c == ctrl_key(b'q') {
    //                 reset_screen_on_end();
    //                 break;
    //             }
    //         }
    //         Err(e) => {
    //             reset_screen_on_end();
    //             panic!("{}", e)
    //         }
    //     }

    //     refresh_screen(0);
    // }
}
