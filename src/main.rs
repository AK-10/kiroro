use std::io::{stdin, stdout, Read, Write};
use termion::cursor::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct EditorConfig {
    #[allow(dead_code)]
    cols: u16,
    rows: u16,
}

impl EditorConfig {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}

// In ASCII character code, control characters that are not displayed as characters
// are assigned to 0~31(0000_0000 ~ 0001_1111)
// so ctrl char code can be get by bit and
// example:
// b'q' & 0b0001_1111
#[allow(dead_code)]
fn ctrl_key(key: u8) -> u8 {
    key & 0b001_1111
}

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

// return (col, row)
#[allow(dead_code)]
fn get_cursor_pos() -> (u16, u16) {
    // get terminal status by \x1b[6n (https://vt100.net/docs/vt100-ug/chapter3.html#DSR)
    // temrinal responses to stdin (http://vt100.net/docs/vt100-ug/chapter3.html#CPR)
    // like `\x1b[{row};{col}R`
    // after print \x1b[6n, parse response
    print!("\x1b[6n");
    print!("\r\n");

    let mut response = Vec::<u8>::new();

    for b in stdin().bytes() {
        match b.unwrap() {
            b'\x1b' | b'[' => {}
            b'R' => break,
            b => response.push(b),
        }
    }
    let row_col = String::from_utf8(response).unwrap();
    let row_col = row_col.splitn(2, ';').collect::<Vec<&str>>();
    let row_col: Vec<u16> = row_col
        .iter()
        .map(|num_str| num_str.parse::<u16>().unwrap())
        .collect();

    (row_col[1], row_col[0])
}

// return (col, row)
fn get_window_size() -> (u16, u16) {
    match termion::terminal_size() {
        Ok(ts) => ts,
        Err(_) => {
            // get termsize manually
            // move cursor bottom right
            // \x1b[nC (n: natural number) move cursor to right direction amount of n
            // \x1b[nB (n: natural number) move cursor to bottom direction amount of n
            // print!("\x1b[999C\x1b[999B");
            print!("{}", termion::cursor::Goto(999, 999));
            stdout().flush().unwrap();

            // then, get cursor position
            // get_cursor_pos()
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
                match k {
                    // if k == ctrl_key(b'q')
                    termion::event::Key::Ctrl('q') => {
                        reset_screen_on_end();
                        break;
                    }
                    termion::event::Key::Ctrl(c) => {
                        println!("{}\r", c);
                    }
                    termion::event::Key::Char(c) => {
                        println!("{} ({})\r", c as u8, c);
                    }
                    _ => println!("{:?}\r", k),
                }
            }
            Err(e) => {
                reset_screen_on_end();
                panic!("{}", e);
            }
        }
        refresh_screen(0);
    }
}
